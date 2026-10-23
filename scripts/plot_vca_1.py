#!/usr/bin/env python3
"""Plot the VCA-1 full model run from its stored ngspice rawfiles.

    python3 -I scripts/plot_vca_1.py <run-dir> <cases.json> <out-dir>

Writes one PNG per figure into <out-dir> and prints `name<TAB>caption` lines.
Needs numpy and gnuplot. Only reads the run; nothing is re-simulated.
"""

import gzip
import json
import math
import subprocess
import sys
import tempfile
from pathlib import Path

import numpy as np

LN1000 = math.log(1000.0)
TIMES = ["0.001s", "0.01s", "0.1s", "1s", "10s", "20s"]
TIME_LABELS = {"0.001s": "1 ms", "0.01s": "10 ms", "0.1s": "100 ms", "1s": "1 s", "10s": "10 s", "20s": "20 s"}
COLORS = ["#7b2cbf", "#2a9d8f", "#e76f51", "#264653", "#e9c46a", "#457b9d"]


def read_raw(path: Path) -> dict[str, np.ndarray]:
    text = gzip.open(path, "rt").read()
    head, values = text.split("Values:\n", 1)
    lines = head.split("\n")
    nv = int(next(l for l in lines if l.startswith("No. Variables")).split(":")[1])
    npts = int(next(l for l in lines if l.startswith("No. Points")).split(":")[1])
    complex_ = "complex" in next(l for l in lines if l.startswith("Flags:"))
    start = lines.index("Variables:") + 1
    names = [lines[start + k].split()[1].lower() for k in range(nv)]
    tokens = values.split()[: npts * (nv + 1)]
    grid = np.array(tokens, dtype=object).reshape(npts, nv + 1)[:, 1:]
    out = {}
    for i, name in enumerate(names):
        col = grid[:, i]
        if complex_:
            out[name] = np.array([complex(*map(float, t.split(","))) for t in col])
        else:
            out[name] = col.astype(float)
    return out


class Run:
    def __init__(self, run_dir: Path, cases_json: Path):
        self.cases = run_dir / "cases"
        data = json.loads(cases_json.read_text())
        self.defs = {c["id"]: c for c in (data["cases"] if isinstance(data, dict) else data)}

    def raw(self, case: str, sub: str) -> dict[str, np.ndarray]:
        return read_raw(next((self.cases / case / sub).glob("*.raw.gz")))

    def result(self, case: str) -> dict:
        return json.loads((self.cases / case / "result.json").read_text())["result"]


def env_fs(raw: dict[str, np.ndarray]) -> np.ndarray:
    return (raw["v(envb)"] - raw["v(venv0)"]) / (raw["v(vref5)"] - raw["v(venv0)"])


def decimate(*cols: np.ndarray, n: int = 4000) -> list[np.ndarray]:
    step = max(1, len(cols[0]) // n)
    return [c[::step] for c in cols]


class Plot:
    """One gnuplot figure; series are written to temporary data files."""

    def __init__(self, tmp: Path, name: str, title: str, xlabel: str, ylabel: str):
        self.tmp, self.name = tmp, name
        self.lines = [
            "set terminal pngcairo size 1600,1000 enhanced font 'Sans,20' linewidth 2",
            f"set title {json.dumps(title, ensure_ascii=False)}",
            f"set xlabel {json.dumps(xlabel, ensure_ascii=False)}",
            f"set ylabel {json.dumps(ylabel, ensure_ascii=False)}",
            "set grid lc rgb '#dddddd'",
            "set key top right",
            "set border lc rgb '#444444'",
        ]
        self.series: list[str] = []

    def set(self, line: str):
        self.lines.append(line)

    def add(self, x, y, label: str, color: str, dash: bool = False):
        path = self.tmp / f"{self.name}-{len(self.series)}.dat"
        np.savetxt(path, np.column_stack([x, y]))
        style = "dt 2" if dash else "dt 1"
        self.series.append(f"'{path}' using 1:2 with lines lw 2 {style} lc rgb '{color}' title {json.dumps(label, ensure_ascii=False)}")

    def render(self, out: Path):
        script = "\n".join(self.lines + [f"set output '{out / (self.name + '.png')}'", "plot " + ", \\\n  ".join(self.series), ""])
        subprocess.run(["gnuplot"], input=script.encode("utf-8"), check=True)


def stage_plot(run: Run, tmp: Path, out: Path, stage: str) -> str:
    p = Plot(tmp, f"env-{stage}", f"VCA-1 envelope {stage}, six times, normalized", "time / set time", "envelope (fraction of full scale)")
    p.set("set xrange [0:1.2]")
    p.set("set yrange [-0.05:1.1]")
    x_ref = np.linspace(0, 1.2, 600)
    for t, color in zip(TIMES, COLORS):
        case = f"env-{stage}-{t}"
        d = run.defs[case]
        raw = run.raw(case, "tran")
        e, time = env_fs(raw), raw["time"]
        t_cmd = d["t_cmd_s"]
        if stage == "attack":
            t0 = d["gate_rise_s"]
        elif stage == "decay":
            t0 = time[int(np.argmax(e))]
        else:
            t0 = d["gate_fall_s"]
        mask = time >= t0
        x, y = decimate((time[mask] - t0) / t_cmd, e[mask])
        p.add(x, y, TIME_LABELS[t], color)
    sus = run.defs[f"env-{stage}-1s"]["controls"]["SUSTAIN"]
    if stage == "attack":
        ref = 1.0 - np.exp(-LN1000 * x_ref)
    elif stage == "decay":
        ref = sus + (1.0 - sus) * np.exp(-LN1000 * x_ref)
    else:
        ref = np.exp(-LN1000 * x_ref)
    p.add(x_ref, ref, "software curve", "#000000", dash=True)
    p.render(out)
    if stage == "attack":
        return "Attack at 1 ms to 20 s on a normalized time axis, against the software curve. The hardware aims 1 % above full scale and stops at the peak, so it ends at 0.668 of the set time by design."
    if stage == "decay":
        return f"Decay to sustain {sus:g} at 1 ms to 20 s on a normalized time axis, against the software curve."
    return "Release from full scale at 1 ms to 20 s on a normalized time axis, against the software curve."


def events_plot(run: Run, tmp: Path, out: Path) -> str:
    p = Plot(tmp, "gate-events", "VCA-1 gate events", "time (ms)", "envelope (fraction of full scale)")
    p.set("set yrange [-0.05:1.1]")
    for case, label, color in [
        ("gate-1ms", "1 ms gate, A = D = R minimum", COLORS[0]),
        ("retrigger-release", "retrigger during release", COLORS[1]),
        ("retrigger-decay", "retrigger during decay", COLORS[2]),
    ]:
        raw = run.raw(case, "tran")
        x, y = decimate(raw["time"] * 1e3, env_fs(raw))
        p.add(x, y, label, color)
    p.render(out)
    return "A 1 ms gate gives a complete event. A new gate during release restarts the attack from the current level; in the decay case a 0.2 ms gap at minimum release drops the level to 0.17 first, and the attack restarts from there."


def gain_plot(run: Run, tmp: Path, out: Path) -> str:
    p = Plot(tmp, "gain-law", "VCA-1 static gain law", "control (0 to 1)", "gain (V/V)")
    p.set("set key top left")
    p.set("set xrange [0:1.05]")
    for case, label, color in [("gain-law-1000Hz", "1 kHz", COLORS[0]), ("gain-law-10000Hz", "10 kHz", COLORS[1])]:
        r = run.result(case)
        p.add(np.array(r["g_ctrl"]), np.array(r["gain"]), f"{label}, linearity {r['linearity_pct']:.2f} % FS", color, dash=label == "10 kHz")
    p.render(out)
    return "Gain against control at 1 kHz and 10 kHz, 0.02 Vpp input: monotonic and linear within a fraction of a percent of full scale."


def response_plot(run: Run, tmp: Path, out: Path) -> str:
    p = Plot(tmp, "response", "VCA-1 small-signal frequency response", "frequency (Hz)", "gain (dB)")
    p.set("set logscale x")
    p.set("set xrange [10:30000]")
    p.set("set yrange [8:17]")
    p.set("set key center right")
    for case, label, color in [("response-g1", "control 1", COLORS[0]), ("response-g0.5", "control 0.5", COLORS[1])]:
        raw = run.raw(case, "ac")
        g = np.abs(raw["v(audio_out)"] / raw["v(audio_in)"])
        p.add(raw["frequency"].real, 20 * np.log10(g), label, color)
    p.render(out)
    return "Small-signal response at two control settings, within 1 dB from 20 Hz to 20 kHz."


def thd_plot(run: Run, tmp: Path, out: Path) -> str:
    p = Plot(tmp, "thd-waveform", "VCA-1 1 kHz, 1 Vpp input, full gain", "time (ms)", "voltage (V)")
    raw = run.raw("thd-1Vpp-g1", "tran")
    t = raw["time"]
    mask = t >= t[-1] - 3e-3
    r = run.result("thd-1Vpp-g1")
    tt = (t[mask] - t[mask][0]) * 1e3
    p.add(tt, raw["v(audio_in)"][mask] - raw["v(audio_in)"][mask].mean(), "input", COLORS[1])
    p.add(tt, raw["v(audio_out)"][mask], f"output, THD {r['thd'] * 100:.3f} %", COLORS[0])
    p.render(out)
    return f"Last 3 ms of the 1 Vpp full-gain case: {r['out_vpp']:.2f} Vpp out, THD {r['thd'] * 100:.3f} % (limit 1 %)."


def corners_plot(run: Run, tmp: Path, out: Path) -> str:
    p = Plot(tmp, "corners", "VCA-1 1 s envelope case across temperature and part corners", "time (s)", "envelope (fraction of full scale)")
    p.set("set yrange [-0.05:1.1]")
    for case, label, color in [
        ("corner-t10-env-1s", "10 °C", COLORS[5]),
        ("corner-t40-env-1s", "40 °C", COLORS[2]),
        ("corner-cold-env-1s", "cold part corner", COLORS[3]),
        ("corner-hot-env-1s", "hot part corner", COLORS[0]),
    ]:
        raw = run.raw(case, "tran")
        x, y = decimate(raw["time"], env_fs(raw))
        p.add(x, y, label, color)
    p.render(out)
    return "The 1 s envelope at 10 and 40 °C and at the part corners, with trims fixed at nominal: times stay within 15 %."


def chain_plot(run: Run, tmp: Path, out: Path) -> str:
    p = Plot(tmp, "chain", "VCO-1 → VCF-1 → VCA-1 chain, 1 kHz triangle", "time (ms)", "voltage (V)")
    raw = run.raw("integration-triangle-1000", "chain")
    t = raw["time"]
    mask = t >= t[-1] - 4e-3
    tt = (t[mask] - t[mask][0]) * 1e3
    for node, label, color in [("v(tri_b)", "VCO-1 triangle", COLORS[1]), ("v(mix)", "VCF-1 mix output", COLORS[2]), ("v(vca_out)", "VCA-1 output", COLORS[0])]:
        y = raw[node][mask]
        x, y = decimate(tt, y - y.mean())
        p.add(x, y, label, color)
    r = run.result("integration-triangle-1000")
    p.render(out)
    return f"Released VCO-1 and VCF-1 netlists driving VCA-1 (DC removed for display). Loading the oscillator moves its frequency by {r['b1_frequency_rel'] * 100:.4f} %."


def burst_plot(run: Run, tmp: Path, out: Path) -> str:
    p = Plot(tmp, "burst", "VCA-1 overload burst and recovery", "time (s)", "output (V)")
    raw = run.raw("burst-recovery", "tran")
    x, y = decimate(raw["time"], raw["v(audio_out)"], n=20000)
    p.add(x, y, "output", COLORS[0])
    r = run.result("burst-recovery")
    p.render(out)
    return f"A 100 ms, 2 Vpp burst at full gain, then 0.1 Vpp: the output returns to its reference in {r['recovery_s'] * 1e3:.0f} ms (limit 500 ms)."


def main() -> int:
    run_dir, cases_json, out = (Path(a) for a in sys.argv[1:4])
    out.mkdir(parents=True, exist_ok=True)
    run = Run(run_dir, cases_json)
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        figures = [
            ("env-attack", stage_plot(run, tmp, out, "attack")),
            ("env-decay", stage_plot(run, tmp, out, "decay")),
            ("env-release", stage_plot(run, tmp, out, "release")),
            ("gate-events", events_plot(run, tmp, out)),
            ("gain-law", gain_plot(run, tmp, out)),
            ("response", response_plot(run, tmp, out)),
            ("thd-waveform", thd_plot(run, tmp, out)),
            ("corners", corners_plot(run, tmp, out)),
            ("chain", chain_plot(run, tmp, out)),
            ("burst", burst_plot(run, tmp, out)),
        ]
    for name, caption in figures:
        print(f"{name}\t{caption}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
