#!/usr/bin/env python3
"""Copy selected research release images into public/ and write src/research_assets.rs.

Only the latest release of each module is used. Sources are named below, so a
new release means changing one path and re-running:

    python3 -I scripts/sync_research_assets.py

Needs ImageMagick (`magick`) and poppler (`pdftoppm`). The research repo is
private; nothing but the files listed here leaves it.
"""

import re
import shutil
import subprocess
import sys
import tempfile
import zipfile
from pathlib import Path

SITE = Path(__file__).resolve().parent.parent
ART = Path("/home/dev/work-base/workspace/research/mamut-synthesis-research/artifacts")
PUBLIC = SITE / "public"
OUT_RS = SITE / "src" / "research_assets.rs"

SCHEMATIC_WIDTH = 2400
BOARD_WIDTH = 1800
PLOT_WIDTH = 1600

VCO_RELEASE = ART / "mamut-vco-b1-tht-release/20261016T111414.398101Z"
VCO_SIM = ART / "mamut-vco-b1-tht-sim/20261016T103633.864779Z"
VCF_ZIP = ART / "mamut-vcf-1-release/20261017T145452.041965Z.zip"
VCF_ROOT = "20261017T145452.041965Z"
VCA_SCHEMATIC = ART / "mamut-vca-1-work/t12-sch-live-check/mamut-vca-1-a0-tht-schematic.pdf"
VCA_BOARD = ART / "mamut-vca-1-work/t10-board-11"
VCA_RUN = ART / "mamut-vca-1-sim/20261020T054307.251840Z-vca1-full"
VCA_CASES = ART.parent / "sim/mamut-vca-1-tht/cases.json"
VCA_RENDERS = ART / "mamut-vca-1-photoreal/20261020T171344Z/cad"
VCA_AI = ART / "mamut-vca-1-photoreal/20261020T171344Z/photo"
VCO_AI = ART / "mamut-vco-b1-tht-photoreal/20261016T144822Z/photo"
VCF_AI = ART / "mamut-vcf-1-release/mamut-vcf-1-top-photorealistic.png"

# Every photographic-looking image of a board is AI-generated from the CAD data.
AI_GROUP = "AI-generated images"
AI_NOTE = "AI-generated image, not a photograph. No board has been built."

VCF_PLOTS = [
    ("core-bode", "Core frequency response"),
    ("controls", "Cutoff and resonance control response"),
    ("input-transfer", "Input transfer"),
    ("compression", "Compression at high input level"),
    ("spectra", "Output spectra"),
    ("waveforms", "Low-pass, band-pass and mixed waveforms"),
    ("matter", "Mix control at 0, 0.5 and 1 against an independent sum"),
    ("matter-full-spectrum", "Mix control, full spectrum"),
    ("events", "Start-up, CV steps and overload recovery"),
    ("output-loading", "Output loading"),
    ("sensitivity", "Part sensitivity"),
    ("sampled-reference", "Comparison with the sampled software filter reference"),
    ("b1-integration", "Chain with the VCO-1 oscillator"),
]


def run(*args):
    subprocess.run(args, check=True)


def webp(src: Path, dst: Path, width: int) -> tuple[int, int]:
    dst.parent.mkdir(parents=True, exist_ok=True)
    run("magick", str(src), "-resize", f"{width}x>", "-strip", "-quality", "82", str(dst))
    out = subprocess.run(
        ["magick", "identify", "-format", "%w %h", str(dst)],
        check=True, capture_output=True, text=True,
    ).stdout.split()
    return int(out[0]), int(out[1])


def sheet_number(path: Path) -> int:
    return int(re.search(r"(\d+)\.png$", path.name).group(1))


def sheet_titles(pdf: Path, pages: int) -> list[str | None]:
    """KiCad prints `Sheet: /<name>/` in each title block; the root sheet is `/`."""
    titles = []
    for i in range(1, pages + 1):
        text = subprocess.run(
            ["pdftotext", "-f", str(i), "-l", str(i), str(pdf), "-"],
            check=True, capture_output=True, text=True,
        ).stdout
        match = re.search(r"^Sheet: /(.*?)/?$", text, re.M)
        name = match.group(1) if match else None
        titles.append(name or ("Overview" if match else None))
    return titles


class Module:
    def __init__(self, slug: str):
        self.slug = slug
        self.groups: list[tuple[str, list[tuple[str, str, str, int, int]]]] = []
        self.pdf: str | None = None
        self.dir = PUBLIC / "voice" / slug
        if self.dir.exists():
            shutil.rmtree(self.dir)

    def ai_images(self, items: list[tuple[Path, str, str]]):
        self.group(AI_GROUP, [(src, name, f"{AI_NOTE} {caption}", BOARD_WIDTH) for src, name, caption in items])

    def group(self, title: str, items: list[tuple[Path, str, str, int]]):
        figures = []
        for src, name, caption, width in items:
            dst = self.dir / f"{name}.webp"
            w, h = webp(src, dst, width)
            figures.append((f"/voice/{self.slug}/{name}.webp", caption, caption, w, h))
        self.groups.append((title, figures))

    def schematics(self, pages: list[Path], titles_from: Path | None = None):
        total = len(pages)
        titles = sheet_titles(titles_from, total) if titles_from else [None] * total
        self.group("Schematic", [
            (p, f"schematic-{i:02}", f"Sheet {i} of {total}" + (f": {t}" if t else ""), SCHEMATIC_WIDTH)
            for i, (p, t) in enumerate(zip(pages, titles), 1)
        ])

    def schematic_pdf(self, src: Path):
        self.dir.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(src, self.dir / "schematic.pdf")
        self.pdf = f"/voice/{self.slug}/schematic.pdf"


def vco() -> Module:
    m = Module("vco-1")
    m.schematics(
        sorted((VCO_RELEASE / "schematics/png").glob("schematic-*.png"), key=sheet_number),
        titles_from=VCO_SIM / "schematics.pdf",
    )
    m.schematic_pdf(VCO_SIM / "schematics.pdf")
    m.group("Board", [
        (VCO_RELEASE / "pcb/top.png", "pcb-top", "Copper and silkscreen, top", BOARD_WIDTH),
        (VCO_RELEASE / "pcb/bottom.png", "pcb-bottom", "Copper, bottom", BOARD_WIDTH),
        (VCO_RELEASE / "assembly/assembly-top.png", "assembly-top", "Assembly drawing, top", BOARD_WIDTH),
    ])
    m.group("Simulation", [
        (VCO_RELEASE / "simulation/waveforms.png", "sim-waveforms", "Triangle and pulse output waveforms", PLOT_WIDTH),
        (VCO_RELEASE / "simulation/tuning.png", "sim-tuning", "Frequency against control voltage", PLOT_WIDTH),
    ])
    m.ai_images([
        (VCO_AI / "01-hero.png", "ai-hero", "Whole board."),
        (VCO_AI / "02-top.png", "ai-top", "Top view."),
    ])
    return m


def vcf(tmp: Path) -> Module:
    with zipfile.ZipFile(VCF_ZIP) as z:
        for name in z.namelist():
            if name.endswith((".png", ".pdf")) and "/source/" not in name:
                z.extract(name, tmp)
    root = tmp / VCF_ROOT
    m = Module("vcf-1")
    m.schematics(sorted((root / "schematics/png").glob("schematic-*.png"), key=sheet_number))
    m.schematic_pdf(root / "schematics/full-schematic.pdf")
    m.group("Board", [
        (root / "pcb/top.png", "pcb-top", "Copper and silkscreen, top", BOARD_WIDTH),
        (root / "pcb/bottom.png", "pcb-bottom", "Copper, bottom", BOARD_WIDTH),
        (root / "assembly/assembly-top.png", "assembly-top", "Assembly drawing, top", BOARD_WIDTH),
    ])
    m.group("Simulation", [
        (root / f"simulation/report/{name}.png", f"sim-{name}", caption, PLOT_WIDTH)
        for name, caption in VCF_PLOTS
    ])
    m.ai_images([(VCF_AI, "ai-top", "Top view.")])
    return m


def vca(tmp: Path) -> Module:
    pages_dir = tmp / "vca-pages"
    pages_dir.mkdir()
    run("pdftoppm", "-png", "-r", "150", str(VCA_SCHEMATIC), str(pages_dir / "schematic"))
    m = Module("vca-1")
    m.schematics(sorted(pages_dir.glob("schematic-*.png"), key=sheet_number), titles_from=VCA_SCHEMATIC)
    m.schematic_pdf(VCA_SCHEMATIC)
    m.group("Board", [
        (VCA_RENDERS / "01-top-perspective.png", "render-top", "KiCad 3D render from the board file, top three-quarter view. A planned assembly, not a built board.", BOARD_WIDTH),
        (VCA_RENDERS / "03-top-orthographic.png", "render-ortho", "KiCad 3D render, orthographic top view", BOARD_WIDTH),
        (VCA_BOARD / "routed.png", "pcb-routed", "Routed copper, both layers", BOARD_WIDTH),
    ])
    # The run stores raw waveforms but no plots; draw them from the rawfiles.
    plots_dir = tmp / "vca-plots"
    listing = subprocess.run(
        [sys.executable, "-I", str(SITE / "scripts/plot_vca_1.py"), str(VCA_RUN), str(VCA_CASES), str(plots_dir)],
        check=True, capture_output=True, text=True,
    ).stdout
    m.group("Simulation", [
        (plots_dir / f"{name}.png", f"sim-{name}", caption, PLOT_WIDTH)
        for name, caption in (line.split("\t", 1) for line in listing.splitlines())
    ])
    m.ai_images([
        (VCA_AI / "01-hero.png", "ai-hero", "Whole board."),
        (VCA_AI / "07-audio-cell.png", "ai-audio-cell", "Audio cell and output region."),
    ])
    return m


def rust_str(s: str) -> str:
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def emit(modules: list[Module]):
    lines = [
        "// Generated by scripts/sync_research_assets.py. Do not edit by hand.",
        "use crate::content::{Figure, FigureGroup};",
        "",
        "pub fn gallery(slug: &str) -> &'static [FigureGroup] {",
        "    match slug {",
    ]
    for m in modules:
        lines.append(f"        {rust_str(m.slug)} => &[")
        for title, figures in m.groups:
            lines.append(f"            FigureGroup {{ title: {rust_str(title)}, figures: &[")
            for src, alt, caption, w, h in figures:
                lines.append(
                    f"                Figure {{ src: {rust_str(src)}, alt: {rust_str(alt)}, "
                    f"caption: {rust_str(caption)}, width: {w}, height: {h} }},"
                )
            lines.append("            ] },")
        lines.append("        ],")
    lines += ["        _ => &[],", "    }", "}", "", "pub fn schematic_pdf(slug: &str) -> Option<&'static str> {", "    match slug {"]
    for m in modules:
        if m.pdf:
            lines.append(f"        {rust_str(m.slug)} => Some({rust_str(m.pdf)}),")
    lines += ["        _ => None,", "    }", "}", ""]
    OUT_RS.write_text("\n".join(lines))


def main() -> int:
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        modules = [vco(), vcf(tmp), vca(tmp)]
    emit(modules)
    for m in modules:
        count = sum(len(f) for _, f in m.groups)
        print(f"{m.slug}: {count} images, pdf={'yes' if m.pdf else 'no'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
