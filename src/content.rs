// Site content for the hardware research line and the tonewheel91 instruments.
// Every figure here is copied from a dated result or plan document; the source
// is named next to each block so a figure can be re-checked before it changes.

pub const TONEWHEEL91_REPO_URL: &str = "https://github.com/orange-dot/tonewheel91";

/// How far a piece of work has come. Physical tests are a separate statement,
/// because no hardware item has reached them yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Concept,
    Plan,
    InProgress,
    Model,
    CadRelease,
    Playable,
}

impl Stage {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Concept => "Concept",
            Self::Plan => "Plan",
            Self::InProgress => "In progress",
            Self::Model => "Electrical model",
            Self::CadRelease => "CAD release",
            Self::Playable => "Playable",
        }
    }

    pub const fn class(self) -> &'static str {
        match self {
            Self::Concept => "stage stage-concept",
            Self::Plan => "stage stage-plan",
            Self::InProgress => "stage stage-progress",
            Self::Model => "stage stage-model",
            Self::CadRelease => "stage stage-release",
            Self::Playable => "stage stage-playable",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fact {
    pub label: &'static str,
    pub value: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Module {
    pub slug: &'static str,
    pub code: &'static str,
    pub role: &'static str,
    pub stage: Stage,
    pub summary: &'static str,
    pub facts: &'static [Fact],
    pub open: &'static [&'static str],
    pub sections: &'static [Section],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Section {
    pub title: &'static str,
    pub body: &'static str,
    pub points: &'static [&'static str],
    pub code: Option<&'static str>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Figure {
    pub src: &'static str,
    pub alt: &'static str,
    pub caption: &'static str,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FigureGroup {
    pub title: &'static str,
    pub figures: &'static [Figure],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Item {
    pub code: &'static str,
    pub title: &'static str,
    pub stage: Stage,
    pub summary: &'static str,
    pub points: &'static [&'static str],
}

pub const HERO_TITLE: &str = "A keyboard instrument with its own physical sound source.";
pub const HERO_BODY: &str = "Mamut is built from discrete analog circuits, a key surface that senses touch and picks up its sound, and C instrument models that play today. The hardware is designed and simulated; physical tests come next.";

pub const PHYSICAL_NOTE: &str = "Physical tests: not run";

// Sources: docs/46 (system), docs/35, docs/45, docs/40, docs/41, docs/43, docs/50, STATUS.md.
pub const VOICE_INTRO: &str = "One monophonic voice from five through-hole boards: oscillator, filter, amplifier, control and an optional drive stage, with one instrument supply. Each board has its own plan, model run and acceptance criteria.";

pub const VOICE_MODULES: &[Module] = &[
    Module {
        slug: "vco-1",
        code: "VCO-1 B1-THT",
        role: "Oscillator",
        stage: Stage::CadRelease,
        summary: "Discrete linear triangle core with independent pulse width, a protected CV input and buffered outputs. Every part is leaded.",
        facts: &[
            Fact { label: "Board", value: "160 × 120 mm, 82 positions, all through-hole" },
            Fact { label: "Model run", value: "48 cases, 594 numerical checks pass" },
            Fact { label: "CAD checks", value: "ERC, DRC, unconnected items, parity: 0" },
            Fact { label: "Print package", value: "16 pages: schematic, assembly, copper, drilling, parts list" },
        ],
        sections: &[],
        open: &[
            "The CV input is linear; it does not track 1 V/oct. The control board closes that loop.",
            "No board has been built or measured.",
        ],
    },
    Module {
        slug: "vcf-1",
        code: "VCF-1 THT",
        role: "Filter",
        stage: Stage::CadRelease,
        summary: "Discrete-transistor state-variable filter with low-pass, band-pass and mixed outputs. No op-amp, OTA or reference ICs.",
        facts: &[
            Fact { label: "Board", value: "260 × 190 mm, 400 through-hole positions" },
            Fact { label: "Model run", value: "199 cases, 1208 checks, 0 failed" },
            Fact { label: "CAD checks", value: "ERC, DRC, unconnected items, parity: 0" },
            Fact { label: "Release", value: "2026-10-17" },
        ],
        sections: &[],
        open: &["No board has been built or measured."],
    },
    Module {
        slug: "vca-1",
        code: "VCA-1 THT",
        role: "Amplifier and envelope",
        stage: Stage::CadRelease,
        summary: "Linear voltage-controlled amplifier with a built-in ADSR envelope. A discrete translinear gain cell, exponential envelope converters with temperature compensation, and a buffered 0 to 5 V envelope output. Every part is leaded.",
        facts: &[
            Fact { label: "Board", value: "339 × 334 mm, 2 layers, 734 positions, all through-hole" },
            Fact { label: "Model run", value: "100 cases, 0 failed; 22 requirement groups (2026-10-20)" },
            Fact { label: "Run cost", value: "251 MB of results, 2 h 51 min" },
            Fact { label: "CAD checks", value: "ERC, DRC, unconnected items, parity: 0" },
            Fact { label: "SPICE parity", value: "Exported netlist identical to the assessed circuit (703 elements)" },
            Fact { label: "Schematic", value: "34 wired sheets; ERC 0; 734 parts and 409 nets match the simulated circuit" },
            Fact { label: "Supply", value: "+12 V, at most 58.4 mA with the gate high" },
        ],
        sections: &[
            Section {
                title: "Signal and control path",
                body: "Audio is amplified about six times, converted to a current and multiplied by the control current in the gain cell. The envelope, a level CV and an offset are summed, clamped to 0..1 and converted to that control current.",
                points: &[],
                code: Some("AUDIO_IN → input amp (×6) → V-to-I → gain cell → I-to-V buffer → AC coupling → AUDIO_OUT\n                                         ↑ I_CTRL\nENV + LEVEL_CV + OFFSET → summer → clamp 0..1 → V-to-I"),
            },
            Section {
                title: "Gain cell",
                body: "A linearized current-mode cell from matched discrete transistors: a diode pair and an output pair. The gain is linear in the control current and does not depend on the thermal voltage. The input current is kept at 70 % of the diode current or less at the 1 Vpp maximum clean input.",
                points: &[],
                code: Some("i_out  = i_in · I_CTRL / I_DIODE\nG_full = (R_OUT / R_IN) · (I_CTRL_MAX / I_DIODE)"),
            },
            Section {
                title: "Envelope",
                body: "Attack, decay and release pots drive exponential converters with PTC compensation. Three transconductance followers charge one timing capacitor, read through a JFET buffer. A Schmitt trigger and an attack latch select the segment; a peak comparator ends the attack.",
                points: &[
                    "Times from 1 ms to 20 s, each within 20 % of target in the model",
                    "Attack aims 1 % above full scale, so the peak arrives at 0.668 × the attack time; the shape error before the peak is at most 1 % of full scale",
                    "A new gate during release restarts the attack from the current level",
                    "A 1 ms gate gives a complete event; a held gate does not retrigger",
                ],
                code: None,
            },
            Section {
                title: "What the model run checks",
                body: "The 2026-10-20 run covers 100 cases. Selected limits, all met:",
                points: &[
                    "Gain law monotonic, linearity within 2 % of full scale; full gain within 0.5 dB",
                    "Off state at most −70 dB at 1 kHz and −60 dB at 10 kHz",
                    "THD at most 1 % at 1 Vpp, 0.3 % at 0.1 Vpp; response within 1 dB from 20 Hz to 20 kHz",
                    "Control feedthrough with 1 ms segments: peak at most 20 mV, settled step at most 10 mV",
                    "10 / 25 / 40 °C and part corners with fixed trims: gain drift at most 10 %, envelope times within 15 %",
                    "Misuse: reverse supply and ±12 V on the control inputs stay inside part limits",
                    "In the chain with VCO-1 and VCF-1: oscillator frequency moves at most 0.0063 %, amplitude at most 0.59 %",
                    "Halving the simulation time step changes the results by at most 2 %",
                ],
                code: None,
            },
            Section {
                title: "Relation to the software voice",
                body: "The module is a functional analog interpretation of the amplifier and envelope in the analog voice model. It is not a copy of the software transfer functions, and it does not claim to sound the same: the hardware attack ends earlier by design, and no listening comparison exists.",
                points: &[],
                code: None,
            },
        ],
        open: &[
            "One full run before this one failed 95 of 100 cases on metric and measurement errors; it is kept as evidence.",
            "Matched-pair thermal bonding and the remaining assembly steps are open.",
            "Panel pot linearity is not specified yet.",
            "No board has been built or measured.",
        ],
    },
    Module {
        slug: "ctl-1",
        code: "CTL-1 THT",
        role: "Control",
        stage: Stage::InProgress,
        summary: "MIDI, USB and CV control for the voice: drives oscillator pitch through a servo loop, sets filter cutoff, calibrates the filter and gates the amplifier.",
        facts: &[
            Fact { label: "Ports", value: "Frozen" },
            Fact { label: "Work", value: "Third task stage in progress" },
        ],
        sections: &[],
        open: &["Firmware has not run on target hardware."],
    },
    Module {
        slug: "trc-1",
        code: "TRC-1 THT",
        role: "Drive stage",
        stage: Stage::InProgress,
        summary: "Optional drive stage after the amplifier that follows a triode reference curve, with a manual drive control.",
        facts: &[
            Fact { label: "Ports", value: "Frozen" },
            Fact { label: "Work", value: "Third task stage in progress" },
        ],
        sections: &[],
        open: &["No released model or CAD yet."],
    },
    Module {
        slug: "pwr-1",
        code: "PWR-1",
        role: "Instrument supply",
        stage: Stage::Plan,
        summary: "Power for the complete instrument from one 24 V adapter, with a central +12 V rail on a discrete pass regulator and no host computer.",
        facts: &[
            Fact { label: "Input", value: "24 V, 280 W adapter" },
            Fact { label: "+12 V", value: "12 A rated for 32 voices (8.25 A estimated)" },
            Fact { label: "Other rails", value: "−12 V 2 A, +5 V 3 A, +24 V auxiliary" },
        ],
        sections: &[],
        open: &["Baseline waits for confirmation. No circuit, simulation or hardware yet."],
    },
];

pub const VOICE_SIGNAL_PATH: &str =
    "MIDI / USB / CV → CTL-1 → VCO-1 → VCF-1 → VCA-1 → (TRC-1) → out";

pub const ASSEMBLY_POLICY: &str = "Every board is meant to be soldered by hand. Through-hole is preferred but no longer required; parts are chosen from packages that can be hand-soldered and are available in 2026.";

// Sources: docs/22, docs/24, docs/25, docs/29, docs/30.
pub const KEYBOARD_INTRO: &str = "A sensing layer for a standard synthesizer keyboard. It measures where and how a finger touches each key and picks up the sound of that contact, so touch becomes both control and excitation.";

pub const KEYBOARD_STAGES: &[Item] = &[
    Item {
        code: "P0-D rev. B",
        title: "Discrete piezo receiver",
        stage: Stage::Model,
        summary: "A fully discrete voltage receiver for a piezo film sensor, with schematic, PCB, parts list and bench instructions.",
        points: &[
            "204 simulation cases, 396 criteria pass",
            "Test points and three trial assemblies in the CAD package",
        ],
    },
    Item {
        code: "Strip P1",
        title: "Per-key sensor strip",
        stage: Stage::CadRelease,
        summary: "A routed 12 × 120 mm two-layer board, one per key, with leaded parts on the front and 0805 parts on the back.",
        points: &[
            "ERC, DRC, unconnected items, parity: 0",
            "Same 204 cases / 396 criteria as P0-D",
            "Fit in a donor keyboard is not measured",
        ],
    },
    Item {
        code: "H0",
        title: "Hybrid sensor: audio, touch, force",
        stage: Stage::Model,
        summary: "Separate discrete channels for contact audio, touch and force from one key surface.",
        points: &[
            "104 cases, 71 required checks pass within the stated model limits",
            "Electrode board uses assumed dimensions",
        ],
    },
    Item {
        code: "H0-B",
        title: "Receiver bench PCB",
        stage: Stage::CadRelease,
        summary: "Routed two-layer 230 × 190 mm receiver board for bench measurements of the hybrid sensor.",
        points: &[
            "241 electrical positions; ERC, DRC, unconnected items, parity: 0",
            "Schematic PDF, assembly drawings, parts list, Gerber and drill files",
        ],
    },
    Item {
        code: "Top surface",
        title: "Sensors under the finger",
        stage: Stage::Concept,
        summary: "Moves the piezo film to the playing surface of the key and compares capacitive touch, force-sensing resistors, capacitive force and membrane position, with a replaceable sensor cap if a thin overlay is not enough.",
        points: &["Layer stack and response are not validated"],
    },
];

// Sources: docs/02, docs/06–09, docs/16.
pub const SOURCES_INTRO: &str = "The long line: physical mechanisms that could give the instrument a sound source of its own, the way a spinning tonewheel or a struck metal tine defines an earlier keyboard. Each one is tested on its own before any combination.";

pub const SOURCES: &[Item] = &[
    Item {
        code: "S1",
        title: "Bistable cell array",
        stage: Stage::Model,
        summary: "Mechanical cells that snap between two stable states, with programmable coupling between cells. The hypothesis is that the transition itself can be played.",
        points: &[
            "First KiCad and SPICE package: 39 cases, 107 numerical checks pass",
            "Short-pulse priming failed and is recorded as a result",
            "Next CAD steps: a coupled pair, then four cells",
        ],
    },
    Item {
        code: "S2",
        title: "Magnetic avalanche source",
        stage: Stage::Concept,
        summary: "Signal from the small, abrupt magnetization jumps in a ferromagnetic material under a changing field, shaped into a controllable musical source.",
        points: &["Concept and generator design only"],
    },
    Item {
        code: "S3",
        title: "Granular contact network",
        stage: Stage::Concept,
        summary: "Sound from pressure, friction and rearranging contacts in a granular medium, with the contact network as the oscillator.",
        points: &["Concept and design only"],
    },
    Item {
        code: "S4",
        title: "Reconfigurable resonant body",
        stage: Stage::Concept,
        summary: "A resonant body whose geometry changes measurably while it sounds.",
        points: &["Concept and design only"],
    },
];

pub const CLAIM_LEVELS: &[Fact] = &[
    Fact { label: "Known basis", value: "A physical effect or precedent with a cited source." },
    Fact { label: "Proposal", value: "Our construction, control or working target." },
    Fact { label: "Calculation", value: "Output of a named model under stated assumptions." },
    Fact { label: "Simulation result", value: "Versioned inputs, raw data and pass criteria." },
    Fact { label: "Physical result", value: "A measured sample. None exist yet." },
];

// Sources: tonewheel91 README.md, docs/design.md, docs/piano-backlog.md, docs/analog-backlog.md.
pub const INSTRUMENTS_INTRO: &str = "tonewheel91 is a public C23 codebase with three component-modeled keyboard instruments. Each runs live from a Linux ALSA MIDI-in / PCM-out driver, and an offline MIDI-file renderer produces bit-identical output for tests.";

pub const INSTRUMENTS: &[Item] = &[
    Item {
        code: "Organ",
        title: "Tonewheel organ",
        stage: Stage::Playable,
        summary: "Ninety-one always-running tonewheels behind a gear-ratio table, a key-contact and busbar network, percussion, a vibrato/chorus scanner, preamp drive and a two-rotor rotary speaker.",
        points: &[
            "Key contacts addressed by key travel, not only by strike speed",
            "One wear control for level spread, leakage, motion AM and mains hum; zero reproduces the ideal unit bit for bit",
            "Preamp drive derived from a circuit-level triode reference",
        ],
    },
    Item {
        code: "ep73",
        title: "Tine electric piano",
        stage: Stage::Playable,
        summary: "73 struck voices, each with three clamped-free tine modes, a second polarisation and a per-voice asymmetric pickup field.",
        points: &[
            "Velocity scales loudness and timbre",
            "Dampers, sustain pedal, tremolo, drive, cabinet and per-note condition",
            "Shares kernels with the organ without changing its output",
        ],
    },
    Item {
        code: "Analog",
        title: "Analog voice model",
        stage: Stage::Playable,
        summary: "Voice cards with two VCOs, a driven four-pole low-pass, RC envelopes, cross-modulation and unison, carried into a pool of 64 slots with 32 active by default.",
        points: &[
            "Aperiodic tile oscillator and wavetable source next to the analog core",
            "Bounded card-to-card differences before any effect",
            "Chorus and reverb as a stereo finish",
        ],
    },
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Note {
    pub slug: &'static str,
    pub date: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
}

/// New notes. The archive keeps the EPM1 / drum-engine notes reachable.
pub const NOTES: &[Note] = &[];

pub fn module_by_slug(slug: &str) -> Option<&'static Module> {
    VOICE_MODULES.iter().find(|module| module.slug == slug)
}
