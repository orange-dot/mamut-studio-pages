#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RepoKind {
    Epm1,
    Epm2,
}

pub const EPM2_PUBLIC_REPO_URL: &str = "https://github.com/orange-dot/mamut-epm-hw-";

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HeroContent {
    pub eyebrow: &'static str,
    pub title: &'static str,
    pub body: &'static str,
    pub status: &'static str,
    pub primary_cta: &'static str,
    pub secondary_cta: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Stat {
    pub label: &'static str,
    pub value: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PageIntro {
    pub kicker: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DetailSection {
    pub title: &'static str,
    pub body: &'static str,
    pub bullets: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ProductLine {
    pub code: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    pub status: &'static str,
    pub repo_path: &'static str,
    pub repo: RepoKind,
    pub bullets: &'static [&'static str],
    pub source_path: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LabCard {
    pub label: &'static str,
    pub title: &'static str,
    pub body: &'static str,
    pub detail: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DocCategory {
    pub title: &'static str,
    pub body: &'static str,
    pub docs: &'static [DocCard],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DocCard {
    pub title: &'static str,
    pub summary: &'static str,
    pub source_path: &'static str,
    pub repo: RepoKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BlogPost {
    pub slug: &'static str,
    pub series: &'static str,
    pub title: &'static str,
    pub intro: &'static str,
    pub body: &'static str,
    pub bullets: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BlogPostSection {
    pub title: &'static str,
    pub body: &'static str,
    pub bullets: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AdjacentProject {
    pub kicker: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    pub repo_path: &'static str,
    pub bullets: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DrumEngineTrack {
    pub title: &'static str,
    pub url: &'static str,
    pub note: &'static str,
    pub track_id: &'static str,
}

pub const HERO: HeroContent = HeroContent {
    eyebrow: "Mamut EPM",
    title: "Current software instrument.",
    body: "EPM1 plays now: browser demo, desktop runtime, patch bank, macro controls, and PC4 MIDI work. EPM2 follows the hardware path through analog studies, KiCad capture, simulation, and bench preparation.",
    status: "Play the browser demo first, then use Notes and Docs for the current build notes.",
    primary_cta: "Open play",
    secondary_cta: "Open notes",
};

pub const STATS: &[Stat] = &[
    Stat {
        label: "Current demo",
        value: "Browser demo + software runtime",
    },
    Stat {
        label: "Runtime focus",
        value: "Patch bank + MIDI control",
    },
    Stat {
        label: "Hardware line",
        value: "EPM2 in study and capture",
    },
    Stat {
        label: "Patch language",
        value: "Horizont, Pec, Baklja, Gravitacija",
    },
];

pub const HOME_FEATURES: &[DetailSection] = &[
    DetailSection {
        title: "EPM1",
        body: "EPM1 is the software instrument that runs now: Rust runtime, factory patches, browser rendering, and PC4-oriented MIDI work.",
        bullets: &[
            "Standalone runtime for audio, MIDI, and live play.",
            "Browser demo and desktop runtime use the same patch names and macro targets.",
            "Transport is locally frozen pending shared platform extraction.",
        ],
    },
    DetailSection {
        title: "EPM2",
        body: "EPM2 is the hardware track for a desktop analog poly synth: simulation, circuit capture, test points, and bench workflow.",
        bullets: &[
            "P1 VCO study, KiCad capture, and bench-prep remain the active path.",
            "External MIDI control keeps V1 focused on the sound engine.",
            "EPM1 keeps the patch and control model testable while hardware work continues.",
        ],
    },
];

pub const PRODUCTS_INTRO: PageIntro = PageIntro {
    kicker: "Lines",
    title: "Current work split.",
    summary: "EPM1 is the runnable software instrument. EPM2 is the hardware build track. The split keeps current software sessions and physical hardware work easy to follow.",
};

pub const LAB_INTRO: PageIntro = PageIntro {
    kicker: "Lab",
    title: "EPM2 hardware lab.",
    summary: "A compact view of the current hardware path: P1 VCO simulation, KiCad capture, bench expectations, and the public source repo.",
};

pub const LAB_STAGES: &[LabCard] = &[
    LabCard {
        label: "Step 01",
        title: "Simulation ladder",
        body: "P1 starts in ngspice with a numbered study ladder.",
        detail: "00 to 101: RC smoke, ramp/reset, threshold reset, expo transfer, integrated chain, temperature, sync, and transistor reset sensitivity.",
    },
    LabCard {
        label: "Step 02",
        title: "KiCad capture",
        body: "Promoted blocks move into a hierarchical KiCad project after the mechanism is clear enough to review.",
        detail: "Current capture covers input tune, power reference, bench I/O, expo converter, threshold comparator, reset output, and current integrator.",
    },
    LabCard {
        label: "Step 03",
        title: "Bench notes",
        body: "Expected behavior is written before physical measurement so simulation, capture, and hardware can be compared directly.",
        detail: "Bench notes track ramp window, reset level, frequency range, sync movement, and test point intent for the P1 oscillator path.",
    },
    LabCard {
        label: "Step 04",
        title: "Public source",
        body: "The public repo carries the hardware study: docs, simulation, KiCad capture, bench expectations, and helper tools.",
        detail: "It keeps docs, sim, KiCad, bench expectations, and tools together while leaving deploy, investor, and outreach material out.",
    },
];

pub const LAB_RESULTS: &[LabCard] = &[
    LabCard {
        label: "60",
        title: "Integrated VCO chain",
        body: "The current `CV -> expo -> core -> saw/pulse` reference works as the main P1 study platform.",
        detail: "The documented range is roughly 489 Hz to 4021 Hz across the modest -1 V to +2 V sweep.",
    },
    LabCard {
        label: "70",
        title: "Temperature drift",
        body: "The simplified expo model is characterized before compensation work hides the drift.",
        detail: "The +1 V / 0 V ratio bends across temperature, which gives the expo work a concrete baseline.",
    },
    LabCard {
        label: "80",
        title: "Sync hook",
        body: "The first reset-oriented sync comparison shows that the oscillator period can be steered.",
        detail: "The reference notes compare a free-running period near 1.0 ms with a sync-forced period near 0.7 ms.",
    },
    LabCard {
        label: "100/101",
        title: "Transistor reset pocket",
        body: "The reset path has moved from a behavioral shortcut into a bench-facing transistor study.",
        detail: "The current useful region stays stable around the 2.2 k to 4.7 k base-drive area, with extreme weak drive failing.",
    },
];

pub const LAB_NEXT_STEPS: &[&str] = &[
    "Keep P1 focused on the first oscillator block before turning the work into a full voice board.",
    "Deepen the expo/integrator path only where simulation and KiCad capture already point to a concrete question.",
    "Use the public repo as the source home for the hardware work.",
];

pub const PRODUCT_LINES: &[ProductLine] = &[
    ProductLine {
        code: "EPM1",
        title: "Software runtime",
        summary: "Rust runtime for the current software instrument: standalone play, factory patches, live-set behavior, and PC4-oriented MIDI work.",
        status: "Active: runnable software instrument",
        repo_path: "/home/dev/sel4/mamut-sint-sw",
        repo: RepoKind::Epm1,
        bullets: &[
            "Core workspace is in place: params, patch, identity, DSP, engine, and standalone.",
            "Sprint 3 hardening and Sprint 4 runtime work are done; Sprint 6 centers the PC4 rig.",
            "Transport is locally frozen pending shared platform extraction.",
        ],
        source_path: "README.md",
    },
    ProductLine {
        code: "EPM2",
        title: "Hardware track",
        summary: "Hardware path for the later physical instrument: desktop analog poly direction, digital control work, simulation, KiCad, and bench notes.",
        status: "Early study, capture, and bench-prep",
        repo_path: "/home/dev/sel4/mamut-sint-hw",
        repo: RepoKind::Epm2,
        bullets: &[
            "Desktop analog poly direction with an external MIDI controller.",
            "P1 VCO study, KiCad capture, and bench workflow define the current forward path.",
            "EPM2 uses the software patch and control model as a reference while the hardware path develops.",
        ],
        source_path: "README.md",
    },
];

pub const BLOG_INTRO: PageIntro = PageIntro {
    kicker: "Notes",
    title: "Working notes.",
    summary: "Short notes from the current software runtime, hardware path, and related rig work.",
};

pub const PC4_BRIDGE: AdjacentProject = AdjacentProject {
    kicker: "Related repo",
    title: "PC4 Microkit Studio",
    summary: "A separate repo for PC4 rig integration, playback control, session logs, and local-first flow around the performance setup.",
    repo_path: "/home/dev/sel4/pc4-microkit-studio",
    bullets: &[
        "Keeps rig orchestration and session files outside the instrument repo.",
        "Covers playback control, musical control boundaries, and hardware bench workflow around the PC4 setup.",
        "Keeps the performance setup in its own source home.",
    ],
};

pub const DRUM_ENGINE_FEATURED_TRACK_ID: &str = "2332273322";

pub const DRUM_ENGINE_PRESET_CONTROLS: &[Stat] = &[
    Stat {
        label: "Tempo",
        value: "143 BPM",
    },
    Stat {
        label: "Chunk",
        value: "4 bars",
    },
    Stat {
        label: "Mode",
        value: "Groove-led",
    },
    Stat {
        label: "Energy",
        value: "0.88",
    },
    Stat {
        label: "Density",
        value: "0.82",
    },
    Stat {
        label: "Risk",
        value: "0.95",
    },
    Stat {
        label: "Fill",
        value: "0.60",
    },
    Stat {
        label: "Surface",
        value: "0.80",
    },
    Stat {
        label: "Humanize",
        value: "0.42",
    },
    Stat {
        label: "Timing",
        value: "0.19",
    },
    Stat {
        label: "Velocity",
        value: "0.92",
    },
    Stat {
        label: "Anti-repeat",
        value: "0.90",
    },
    Stat {
        label: "Loose",
        value: "0.20",
    },
];

pub const DRUM_ENGINE_EVIDENCE: &[LabCard] = &[
    LabCard {
        label: "Intent",
        title: "ADG/AIG layer",
        body: "ADG/AIG keeps the drum decision musical before it becomes MIDI for the PC4.",
        detail: "Groove intent before MIDI output.",
    },
    LabCard {
        label: "Play",
        title: "PC4 rig loop",
        body: "mioXM routes the generated MIDI to the Kurzweil PC4, and the Yamaha AG03 monitors the session.",
        detail: "Local MIDI and audio tied to the real rig.",
    },
    LabCard {
        label: "Taste",
        title: "Manual corpus",
        body: "Selected material and live controls steer the drummer toward takes that fit the track.",
        detail: "Profile-led and intake-led choices stay readable.",
    },
    LabCard {
        label: "Memory",
        title: "Feedback loop",
        body: "Correction and comparison notes help the drummer companion remember what worked while the player keeps the taste call.",
        detail: "Learning follows correction and preference from the player.",
    },
];

pub const DRUM_ENGINE_TRACKS: &[DrumEngineTrack] = &[
    DrumEngineTrack {
        title: "jeans-instability-experiment-moises-ai-guitar-stem",
        url: "https://soundcloud.com/mamut_studio/jeans-instability-experiment",
        note: "Guitar-stem experiment made with Moises AI.",
        track_id: "2332306544",
    },
    DrumEngineTrack {
        title: "jeans instability release candidate 1",
        url: "https://soundcloud.com/mamut_studio/jeans-instability-release",
        note: "Featured take from the saved Drum Engine live set.",
        track_id: DRUM_ENGINE_FEATURED_TRACK_ID,
    },
    DrumEngineTrack {
        title: "jeans instability jam 3",
        url: "https://soundcloud.com/mamut_studio/jeans-instability-jam-3",
        note: "Later live jam from the same SoundCloud set.",
        track_id: "2332264673",
    },
    DrumEngineTrack {
        title: "Jeans Instability v2",
        url: "https://soundcloud.com/mamut_studio/jeans-instability-v2",
        note: "Earlier version with a Moises AI bass track stem.",
        track_id: "2330361851",
    },
    DrumEngineTrack {
        title: "Jeans Instability",
        url: "https://soundcloud.com/mamut_studio/jeans-instability",
        note: "Original organic take in the public SoundCloud set.",
        track_id: "2318818655",
    },
];

pub const DRUM_ENGINE_NOTE_SLUGS: &[&str] = &[
    "drum-engine-companion",
    "adg-aig-drum-language",
    "pc4-drum-rig-flow",
    "drum-engine-feedback-taste-memory",
];

pub const BLOG_POSTS: &[BlogPost] = &[
    BlogPost {
        slug: "drum-engine-companion",
        series: "Drum Engine",
        title: "Drummer Companion",
        intro: "The Drum Engine is the current PC4MS rhythm partner: groove state, live controls, selected material, and hardware playback in one loop.",
        body: "This note follows the Authorial Drum Engine case: a drummer companion listens to the material, locks a posture, shapes drum behavior, and keeps the player in control.",
        bullets: &[
            "Starts from played material, profile law, and live controls.",
            "Tracks groove state across chunks so the drummer can lock, adapt, and shape fills.",
            "Uses the Jeans Instability reference preset as a player-controlled starting point.",
            "Keeps generated MIDI, ADG events, and listening references tied to the take.",
        ],
    },
    BlogPost {
        slug: "adg-aig-drum-language",
        series: "Drum Engine",
        title: "ADG And AIG Drum Language",
        intro: "ADG/AIG holds the drum decision before MIDI: groove intent, gesture, surface, density, timing feel, and relation.",
        body: "ADG keeps drum decisions readable as musical intent. MIDI is the transport that makes those decisions playable on the current rig.",
        bullets: &[
            "AIG is the broader articulated instrument gesture frame.",
            "ADG is the drum dialect for voice, gesture, contact, energy, role, timing, and relationship.",
            "Generated MIDI is a lowering step from ADG drum decisions into the PC4-compatible performance path.",
            "The practical test is whether ADG edits survive into audible groove changes.",
        ],
    },
    BlogPost {
        slug: "pc4-drum-rig-flow",
        series: "Drum Engine",
        title: "PC4 Drum Rig Flow",
        intro: "The current rig flow connects the Drum Engine to hardware: ADG/AIG decisions become MIDI, mioXM routes them, the Kurzweil PC4 plays, and Yamaha AG03 monitoring returns the sound.",
        body: "This note describes the local performance chain used by the Drum Engine case. Drum decisions are judged through the same studio path used for the public takes.",
        bullets: &[
            "Player controls shape the live profile and generated drum decision.",
            "AIG/ADG events lower into MIDI note, velocity, timing, and fill behavior.",
            "mioXM carries the MIDI path into the Kurzweil PC4.",
            "Yamaha AG03 monitoring and recording make the result audible as a performance.",
        ],
    },
    BlogPost {
        slug: "drum-engine-feedback-taste-memory",
        series: "Drum Engine",
        title: "Feedback And Taste Memory",
        intro: "The Drum Engine keeps authorship visible through manual corpus selection, live groove state, comparison notes, and training adjustments.",
        body: "This note describes how feedback becomes session memory. The drummer companion remembers correction and preference while the player decides what counts as a better take.",
        bullets: &[
            "Manual corpus selection gives chosen material more weight than arbitrary variation.",
            "LiveGrooveState carries locked density, energy, accent cells, coupling, and phase across chunks.",
            "Training feedback and comparison records preserve why a take was accepted, rejected, or revised.",
            "Generated MIDI and source notes keep the loop readable after listening.",
        ],
    },
    BlogPost {
        slug: "core-stance",
        series: "Direction",
        title: "Project Boundaries",
        intro: "The project keeps software, hardware, and rig work separated enough that each part can be tested on its own.",
        body: "EPM1, EPM2, and PC4 rig work have different jobs. The software runtime should stay runnable, the hardware path should stay tied to simulation and bench work, and related orchestration should stay in its own repo.",
        bullets: &[
            "Software behavior should be audible and playable now.",
            "Hardware work should move through simulation, capture, and bench measurement.",
            "Rig orchestration belongs in its own repo.",
        ],
    },
    BlogPost {
        slug: "experimental-performance-monster",
        series: "Direction",
        title: "Performance Target",
        intro: "The current sound target is wide, dense, and responsive under macro control. That target guides both the software runtime and the hardware studies.",
        body: "The instrument work is organized around a few concrete behaviors: broad stereo movement, heat and density under control, rupture as a performance gesture, and a global gravity axis that can pull a patch together.",
        bullets: &[
            "Horizont as width, distance, and air.",
            "Pec as heat, density, and pressure.",
            "Baklja as the rupture layer that arrives through performance.",
        ],
    },
    BlogPost {
        slug: "why-two-lines-exist",
        series: "Direction",
        title: "Why Two Lines Exist",
        intro: "The software runtime and the hardware build path move at different speeds, so they are tracked as separate lines.",
        body: "EPM1 exists so patches, macros, and performance behavior can be played now. EPM2 exists so the physical instrument can move through circuit study, capture, and bench work.",
        bullets: &[
            "EPM1 is the current runnable instrument.",
            "EPM2 is the physical hardware track.",
            "Both lines share patch language and macro behavior where that is useful.",
        ],
    },
    BlogPost {
        slug: "epm1-posture",
        series: "Direction",
        title: "EPM1 Runtime Scope",
        intro: "EPM1 is the part that can be run and checked now, so the immediate work stays close to runtime behavior.",
        body: "The software line is focused on a standalone runtime first. It exposes patch checks, dry-run behavior, device selection, live play, and PC4 rig work before plugin or editor polish.",
        bullets: &[
            "Standalone runtime before plugin/editor work.",
            "Factory bank and locked live set are part of the current runtime shape.",
            "Transport is intentionally frozen locally until the shared platform layer is ready.",
            "Audio, MIDI, PC4, and browser demo flows are treated as first-class behavior.",
        ],
    },
    BlogPost {
        slug: "epm2-posture",
        series: "Direction",
        title: "EPM2 Hardware Scope",
        intro: "EPM2 is the hardware track. The current work is still early: simulation, circuit capture, and preparation for measured prototypes.",
        body: "The hardware line keeps the desktop analog poly direction: external MIDI controller, analog signal path, digital control for stateful parts, and room for voice-level variation where it helps the result.",
        bullets: &[
            "External MIDI controller in V1.",
            "External MIDI controller remains the main playing surface.",
            "Scene-scale controls and macro targets stay central.",
        ],
    },
    BlogPost {
        slug: "boundaries-and-roles",
        series: "Direction",
        title: "Boundaries And Roles",
        intro: "The split is useful only if each line keeps a clear job.",
        body: "EPM1 stays focused on runnable software instrument behavior. EPM2 stays focused on desktop hardware study and bench work. PC4 rig orchestration stays separate from both.",
        bullets: &[
            "EPM1 is the runnable software instrument.",
            "EPM2 is the desktop hardware track.",
            "Related rig automation belongs in a separate repo.",
        ],
    },
    BlogPost {
        slug: "epm1-runtime-stack",
        series: "System",
        title: "EPM1 Runtime Stack",
        intro: "The software side is split into explicit runtime pieces with clear responsibilities.",
        body: "The software line is decomposed into crates for parameters, patches, identity terms, DSP, engine behavior, and standalone execution. That makes runtime behavior easier to follow and test.",
        bullets: &[
            "Stable parameter and macro registry.",
            "Canonical TOML patch model and patch checks.",
            "Shared DSP, engine logic, and standalone runtime.",
        ],
    },
    BlogPost {
        slug: "pc4-controller-map-session",
        series: "System",
        title: "PC4 Controller Map Session",
        intro: "A real Kurzweil PC4 session now exercises EPM1 as a one-way MIDI-controlled software instrument.",
        body: "On April 29, 2026, the EPM1 standalone runtime ran through the live rig: Kurzweil PC4 into mioXM DIN 1, then into the Rust standalone synth, with audio out through the Yamaha AG03 on hw:1,0. The session covered the PC4 controller surface that matters for one-way play into Mamut while keeping the detailed session notes in the EPM1 repo.",
        bullets: &[
            "Clean idle held after PC4 Global MIDI cleanup; the startup MIDI log stayed quiet across note, CC, and program-change channels.",
            "Sustain, K1-K9, S1-S9, SW1-SW9, mod wheel, pitch bend, and channel aftertouch were observed in the standalone MIDI log.",
            "Reserved controls were detected as incoming MIDI and left outside synth action mapping.",
            "Patch-switch actions survived on the exclusive AG03 hw:1,0 path with stable ALSA device ownership.",
            "Program Change stayed out of scope for this run because PC4 Global Program Change stayed Off for stable idle behavior.",
        ],
    },
    BlogPost {
        slug: "shared-macro-and-patch-language",
        series: "System",
        title: "Shared Macro And Patch Language",
        intro: "The useful overlap between EPM1 and EPM2 is the patch vocabulary, macro targets, and behavior names.",
        body: "The shared material is practical: patch names, macro vocabulary, tonal identity terms, and behavior expectations that can be tested in software and referenced by hardware work.",
        bullets: &[
            "Horizont, Pec, Baklja, and Gravitacija remain core.",
            "Factory patches already express the language in software.",
            "Patch memory carries instrument behavior and preset storage.",
        ],
    },
    BlogPost {
        slug: "epm2-instrument-architecture",
        series: "System",
        title: "EPM2 Instrument Architecture",
        intro: "The hardware shape combines an analog voice path with digital control where state, calibration, or allocation need it.",
        body: "The hardware line centers the analog voice path while allowing digital control for tuning, allocation, memory, calibration, and panel-state responsibilities where that tradeoff is useful.",
        bullets: &[
            "Analog path remains central to the sound engine.",
            "Digital control stabilizes the stateful and calibration-heavy layers.",
            "The open work is deciding which parts should remain analog, digital, or hybrid.",
        ],
    },
    BlogPost {
        slug: "simulation-first",
        series: "Build Path",
        title: "Simulation First",
        intro: "The hardware path uses simulation before boards so circuit behavior can be checked cheaply and repeatedly.",
        body: "The current study loop uses NGSpice as the daily tool for learning behavior before expensive hardware commitments are made.",
        bullets: &[
            "Simulation playbooks are already documented.",
            "P1-specific notes and walkthroughs exist for the integrated chain.",
            "Behavioral boundaries are being made explicit before deeper transistor work.",
        ],
    },
    BlogPost {
        slug: "capture-and-hardware",
        series: "Build Path",
        title: "Capture And Hardware",
        intro: "The hardware path becomes useful when KiCad capture represents real electrical choices.",
        body: "The repo now carries electrically meaningful capture for P1 blocks, while leaving some areas provisional until they need more fidelity.",
        bullets: &[
            "Input tune, power reference, current integrator, threshold comparator, and reset output are already named as meaningful capture targets.",
            "Hardware organization follows bench relevance and concrete review questions.",
            "The repo layout preserves room for later board and subsystem growth.",
        ],
    },
    BlogPost {
        slug: "bench-loop",
        series: "Build Path",
        title: "Bench Loop",
        intro: "Simulation and capture need measurement feedback once prototypes exist.",
        body: "Bench, notes, and expected-behavior folders are in place so measurements can tighten the loop between simulation and physical reality once the next prototypes land.",
        bullets: &[
            "Bench files have a dedicated home.",
            "Expected behavior lives beside measurement notes.",
            "Study logs and promoted docs can stay in sync.",
        ],
    },
    BlogPost {
        slug: "pc4-microkit-studio",
        series: "Adjacent Work",
        title: "PC4 Microkit Studio",
        intro: "PC4 Microkit Studio is related rig infrastructure, so it stays outside the EPM1 and EPM2 repos.",
        body: "PC4 Microkit Studio is a host-first standalone repo for a studio rig. It centers typed contracts, musical control boundaries, session files, playback control, and hardware bench work around a PC4 setup.",
        bullets: &[
            "Separate canonical repository and separate rig frame.",
            "Related to the performance setup and kept outside the EPM1/EPM2 repo split.",
            "Linked here with a concise summary and outbound repo context.",
        ],
    },
];

pub const DOC_CATEGORIES: &[DocCategory] = &[
    DocCategory {
        title: "EPM1 Runtime Session",
        body: "Hardware-backed runtime notes for the current playable EPM1 instrument.",
        docs: &[DocCard {
            title: "PC4 Full Controller Map",
            summary: "Real Kurzweil PC4 session for one-way sustain, knobs, sliders, switches, wheels, and aftertouch into EPM1.",
            source_path: "docs/live-sessions/2026-04-29-pc4-ag03-full-controller-map.md",
            repo: RepoKind::Epm1,
        }],
    },
    DocCategory {
        title: "EPM2 Overview",
        body: "Overview documents for the current EPM2 hardware shape and working constraints.",
        docs: &[
            DocCard {
                title: "README",
                summary: "Repo-level overview for EPM2 and the map of the working areas.",
                source_path: "README.md",
                repo: RepoKind::Epm2,
            },
            DocCard {
                title: "Project Philosophy",
                summary: "Design constraints and priorities for the instrument work.",
                source_path: "docs/project-philosophy.md",
                repo: RepoKind::Epm2,
            },
            DocCard {
                title: "Direction Notes",
                summary: "The chosen direction for the desktop analog poly and the current V1 boundaries.",
                source_path: "docs/product-direction.md",
                repo: RepoKind::Epm2,
            },
            DocCard {
                title: "Open Questions",
                summary: "Open design questions that still need decisions or measurements.",
                source_path: "docs/open-questions.md",
                repo: RepoKind::Epm2,
            },
        ],
    },
    DocCategory {
        title: "EPM2 Sound And Architecture",
        body: "Sound, control, and architecture notes for the hardware system.",
        docs: &[
            DocCard {
                title: "Performance Target",
                summary: "Behavior targets for how the synth should respond under play.",
                source_path: "docs/vision-experimental-performance-monster.md",
                repo: RepoKind::Epm2,
            },
            DocCard {
                title: "Sound Direction",
                summary: "The tonal center and the relationship between Horizont, Pec, Baklja, and Gravitacija.",
                source_path: "docs/sound-direction.md",
                repo: RepoKind::Epm2,
            },
            DocCard {
                title: "Tonal Architecture",
                summary: "How the identity terms map into system and sound roles.",
                source_path: "docs/tonal-architecture.md",
                repo: RepoKind::Epm2,
            },
            DocCard {
                title: "Gravitacija Spec",
                summary: "The current engine-wide modulation and response axis.",
                source_path: "docs/gravitacija-spec.md",
                repo: RepoKind::Epm2,
            },
            DocCard {
                title: "Architecture Notes",
                summary: "Practical technical direction and working architecture notes.",
                source_path: "docs/architecture-notes.md",
                repo: RepoKind::Epm2,
            },
            DocCard {
                title: "Oscillator Strategy",
                summary: "The role split between anchor, rupture carrier, and density support.",
                source_path: "docs/oscillator-strategy.md",
                repo: RepoKind::Epm2,
            },
        ],
    },
    DocCategory {
        title: "EPM2 Execution Path",
        body: "The practical documents for turning EPM2 into benchable hardware.",
        docs: &[
            DocCard {
                title: "Analog Learning Roadmap",
                summary: "The staged path from studies and power labs into polyphonic translation.",
                source_path: "docs/analog-learning-roadmap.md",
                repo: RepoKind::Epm2,
            },
            DocCard {
                title: "P1 Discrete VCO Prototype",
                summary: "Oscillator prototype plan and intended measurement workflow.",
                source_path: "docs/p1-discrete-vco-prototype.md",
                repo: RepoKind::Epm2,
            },
            DocCard {
                title: "P1 Oscillator Skeleton Spec",
                summary: "Locked interfaces, trims, rails, and test points for the first oscillator cut.",
                source_path: "docs/p1-oscillator-skeleton-spec.md",
                repo: RepoKind::Epm2,
            },
            DocCard {
                title: "Simulation Playbook",
                summary: "Toolchain guidance for Linux simulation work and how to use each piece well.",
                source_path: "docs/simulation-playbook.md",
                repo: RepoKind::Epm2,
            },
            DocCard {
                title: "System Block Diagram",
                summary: "The current system-level signal and control flow shape.",
                source_path: "docs/system-block-diagram.md",
                repo: RepoKind::Epm2,
            },
            DocCard {
                title: "Current Usage Guide",
                summary: "The day-to-day guide for using the current P1 simulation chain as a working tool.",
                source_path: "docs/p1-current-usage-guide.md",
                repo: RepoKind::Epm2,
            },
        ],
    },
];

pub fn blog_post_by_slug(slug: &str) -> Option<BlogPost> {
    BLOG_POSTS.iter().copied().find(|post| post.slug == slug)
}

pub fn blog_post_sections(slug: &str) -> &'static [BlogPostSection] {
    match slug {
        "drum-engine-companion" => DRUM_ENGINE_COMPANION_SECTIONS,
        "adg-aig-drum-language" => ADG_AIG_DRUM_LANGUAGE_SECTIONS,
        "pc4-drum-rig-flow" => PC4_DRUM_RIG_FLOW_SECTIONS,
        "drum-engine-feedback-taste-memory" => DRUM_ENGINE_FEEDBACK_MEMORY_SECTIONS,
        _ => &[],
    }
}

pub const DRUM_ENGINE_COMPANION_SECTIONS: &[BlogPostSection] = &[
    BlogPostSection {
        title: "Player surface",
        body: "The current surface is a responsive drummer companion for the PC4MS rig. It listens to material and live controls, then shapes drummer behavior that can be accepted, corrected, or revised.",
        bullets: &[
            "The musician remains the author of the track and the take.",
            "The engine contributes groove behavior, performance posture, and repeatable output.",
            "The case lives beside EPM because the rig, MIDI, and hardware playback are part of the same studio practice.",
        ],
    },
    BlogPostSection {
        title: "Source center",
        body: "`crates/drum-engine` already has the center of gravity for this story: profile-led, groove-led, and intake-led modes; live chunk requests; generated MIDI events; ADG gestures; source notes; and training records.",
        bullets: &[
            "Profile-led behavior gives the drummer a stable identity.",
            "Intake-led behavior lets captured or played material influence the next drum decision.",
            "Groove-led behavior blends profile and intake while preserving a performance center.",
        ],
    },
    BlogPostSection {
        title: "Reference live preset",
        body: "The Jeans Instability reference preset gives the page a concrete session state: 143 BPM, four-bar chunks, high energy, dense surface, deliberate humanization, anti-repeat, and controlled looseness.",
        bullets: &[
            "The preset is a saved player state for this exact live set.",
            "The same values connect the case page, the public take, and the PC4MS workbench flow.",
            "The listening context stays repeatable: same rig, same controls, same performance target.",
        ],
    },
    BlogPostSection {
        title: "Working flow",
        body: "The strongest version is one working flow: groove intent, profile taste, generated MIDI, hardware playback, listening references, and feedback memory.",
        bullets: &[
            "A short public take is more useful here than a broad catalogue of possible features.",
            "The page should make the reader understand how the drummer decision reaches the rig.",
            "Follow-up work can deepen this flow with stronger source notes, exports, and A/B listening passes.",
        ],
    },
];

pub const ADG_AIG_DRUM_LANGUAGE_SECTIONS: &[BlogPostSection] = &[
    BlogPostSection {
        title: "Drum language",
        body: "AIG is the general articulated gesture frame. ADG is the drum dialect used to describe drum behavior before it is lowered into transport events.",
        bullets: &[
            "The data can carry voice, gesture, energy, timing feel, surface, and phrase role.",
            "The format gives player correction a readable object to revise.",
            "The drummer workflow can preserve intent while changing local output details.",
        ],
    },
    BlogPostSection {
        title: "Drum gesture fields",
        body: "ADG gives drum decisions names that match musical work: anchors, ghosts, fills, surfaces, timing feel, density, contact, and relationships between kit elements.",
        bullets: &[
            "Kick and snare can carry anchor behavior while hats carry surface and motion.",
            "Fill pressure and density can change without flattening the entire pattern into louder hits.",
            "Timing feel can be represented as phrase behavior before it becomes MIDI ticks.",
        ],
    },
    BlogPostSection {
        title: "Lowering to MIDI",
        body: "The current rig still needs MIDI because the Kurzweil PC4 is the playback target. The important design choice is that MIDI is produced from ADG/AIG drum decisions.",
        bullets: &[
            "ADG keeps the authored decision readable.",
            "Generated MIDI makes the decision playable on the current hardware.",
            "Source notes connect the high-level decision to the low-level event output.",
        ],
    },
    BlogPostSection {
        title: "Player correction",
        body: "ADG/AIG stays visible so the player can ask for a tighter lock, more open surface, lower fill pressure, or a different phrase relation while preserving the take identity.",
        bullets: &[
            "The edit target is musical language rather than only individual note edits.",
            "The same representation can drive comparison, training, and future export views.",
            "The current site presents this as a working language layer that can keep evolving.",
        ],
    },
];

pub const PC4_DRUM_RIG_FLOW_SECTIONS: &[BlogPostSection] = &[
    BlogPostSection {
        title: "Player path",
        body: "The player path begins with live controls and a drummer profile. It resolves a drum decision, exports MIDI events, and sends the result through the local hardware chain.",
        bullets: &[
            "Live controls shape energy, density, risk, fill pressure, surface, humanization, and looseness.",
            "The profile and manual corpus provide taste boundaries.",
            "The exported MIDI carries note, velocity, timing, and fill behavior into the rig.",
        ],
    },
    BlogPostSection {
        title: "Hardware chain",
        body: "The current chain is ADG/AIG decision to MIDI, mioXM routing, Kurzweil PC4 playback, and Yamaha AG03 monitoring and recording.",
        bullets: &[
            "mioXM routes generated MIDI into the hardware rig.",
            "The PC4 plays the audible drum performance for this workflow.",
            "The AG03 gives the session a consistent monitoring and capture path.",
        ],
    },
    BlogPostSection {
        title: "Listening references",
        body: "The SoundCloud takes are the quickest way to hear this rig direction. They put the drum flow next to real listening while the code and source notes preserve the build path.",
        bullets: &[
            "The release candidate is the featured take for the case page.",
            "Related takes show nearby experiments and revisions.",
            "The public page should keep one embed and a concise track list so the reader stays focused.",
        ],
    },
    BlogPostSection {
        title: "What the flow connects",
        body: "The flow connects the Drum Engine to the studio setup: ADG/AIG drum decisions become a PC4 performance and are heard through the same local chain used for the published tracks.",
        bullets: &[
            "The build story is tied to hardware.",
            "The output path is specific enough for local replay.",
            "The site presents the rig as part of the creative system rather than a footnote.",
        ],
    },
];

pub const DRUM_ENGINE_FEEDBACK_MEMORY_SECTIONS: &[BlogPostSection] = &[
    BlogPostSection {
        title: "Manual corpus",
        body: "Manual corpus selection gives chosen material and decisions weight. That lets the drummer companion treat chosen references as better starting points than random variation.",
        bullets: &[
            "The corpus can steer repeat reduction, event selection, and local law.",
            "The corpus layer connects listening judgment to the next pass.",
            "The site should describe this as taste memory under player control.",
        ],
    },
    BlogPostSection {
        title: "Live groove state",
        body: "LiveGrooveState carries groove memory between chunks: locked density, velocity energy, accent cells, coupling, confidence, and the current response phase.",
        bullets: &[
            "Learning chunks can observe the material.",
            "Locked chunks can preserve the groove identity.",
            "Higher-energy variation can happen while the locked groove remains the reference.",
        ],
    },
    BlogPostSection {
        title: "Training records",
        body: "Training feedback records and comparison records preserve what the player preferred and why. That makes future revisions more grounded than a one-off random take.",
        bullets: &[
            "Feedback can target a whole take, ADG event, bass event, or comparison side.",
            "Comparison records preserve winner, confidence, take refs, and diff summary.",
            "Training adjustments stay scoped to the relevant drummer identity.",
        ],
    },
    BlogPostSection {
        title: "Session loop",
        body: "The loop is strongest when every audible result connects back to profile, live controls, ADG events, generated MIDI, and feedback records.",
        bullets: &[
            "Source notes support debugging and musical review.",
            "Generated MIDI and source notes make audition and comparison repeatable.",
            "The player stays able to correct the drummer without losing the history of the take.",
        ],
    },
];

fn configured_url(value: Option<&'static str>) -> Option<&'static str> {
    value.and_then(|value| {
        if value.trim().is_empty() {
            None
        } else {
            Some(value)
        }
    })
}

fn default_repo_root(repo: RepoKind) -> Option<&'static str> {
    match repo {
        RepoKind::Epm1 => Some("https://github.com/orange-dot/mamut-sint-sw"),
        RepoKind::Epm2 => Some(EPM2_PUBLIC_REPO_URL),
    }
}

fn configured_repo_input(repo: RepoKind) -> Option<&'static str> {
    match repo {
        RepoKind::Epm1 => configured_url(option_env!("MAMUT_SINT_SW_SOURCE_BASE_URL"))
            .or_else(|| default_repo_root(RepoKind::Epm1)),
        RepoKind::Epm2 => configured_url(option_env!("MAMUT_SINT_HW_SOURCE_BASE_URL"))
            .or_else(|| default_repo_root(RepoKind::Epm2)),
    }
}

fn normalize_repo_urls(input: &str) -> (String, String) {
    let trimmed = input.trim_end_matches('/');
    if let Some((root, _)) = trimmed.split_once("/blob/") {
        (root.to_string(), trimmed.to_string())
    } else {
        (trimmed.to_string(), format!("{trimmed}/blob/main"))
    }
}

pub fn repo_root_url(repo: RepoKind) -> Option<String> {
    configured_repo_input(repo).map(|input| normalize_repo_urls(input).0)
}

pub fn source_url(repo: RepoKind, path: &str) -> Option<String> {
    configured_repo_input(repo).map(|input| {
        let (_, source_base) = normalize_repo_urls(input);
        format!("{source_base}/{path}")
    })
}

pub fn pc4_microkit_studio_url() -> Option<String> {
    let configured = configured_url(option_env!("PC4_MICROKIT_STUDIO_URL"))
        .unwrap_or("https://github.com/orange-dot/pc4-microkit-studio");
    Some(normalize_repo_urls(configured).0)
}
