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
    body: "EPM1 is runnable now: browser demo, desktop runtime, patch bank, macro controls, and PC4 MIDI work. EPM2 tracks the hardware path through analog studies, KiCad capture, simulation, and bench preparation.",
    status: "Play the browser demo first, then use Notes and Docs for the implementation trail.",
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
        body: "EPM1 is the software build that can be run now: Rust runtime, factory patches, browser rendering, and PC4-oriented MIDI work.",
        bullets: &[
            "Standalone runtime with audio, MIDI, and live performance paths.",
            "Browser demo and desktop runtime use the same patch names and macro targets.",
            "Transport is locally frozen pending shared platform extraction.",
        ],
    },
    DetailSection {
        title: "EPM2",
        body: "EPM2 is the hardware research track for a desktop analog poly synth: simulation, circuit capture, test points, and bench workflow.",
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
    summary: "EPM1 is the runnable software instrument. EPM2 is the hardware research and build track. The split keeps current software work and physical hardware work easy to inspect.",
};

pub const LAB_INTRO: PageIntro = PageIntro {
    kicker: "Lab",
    title: "EPM2 hardware lab.",
    summary: "A compact view of the current hardware study path: P1 VCO simulation, KiCad capture, bench expectations, and the public source repo.",
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
        body: "Promoted blocks move into a hierarchical KiCad project only after the mechanism is understood enough to review.",
        detail: "Current capture covers input tune, power reference, bench I/O, expo converter, threshold comparator, reset output, and current integrator.",
    },
    LabCard {
        label: "Step 03",
        title: "Bench contract",
        body: "Expected behavior is written before physical measurement so simulation, capture, and hardware can disagree productively.",
        detail: "Bench notes track ramp window, reset level, frequency range, sync movement, and test point intent for the P1 oscillator path.",
    },
    LabCard {
        label: "Step 04",
        title: "Public source",
        body: "The public repo is a cleaned hardware-study export for docs, simulation, KiCad capture, bench expectations, and helper tools.",
        detail: "It keeps docs, sim, KiCad, bench expectations, and tools together while leaving deploy, investor, and outreach material out.",
    },
];

pub const LAB_RESULTS: &[LabCard] = &[
    LabCard {
        label: "60",
        title: "Integrated VCO chain",
        body: "The current `CV -> expo -> core -> saw/pulse` reference behaves as a usable study platform.",
        detail: "The documented range is roughly 489 Hz to 4021 Hz across the modest -1 V to +2 V sweep.",
    },
    LabCard {
        label: "70",
        title: "Temperature drift",
        body: "The simplified expo model is intentionally characterized before compensation work hides the failure mode.",
        detail: "The +1 V / 0 V ratio bends across temperature, which is useful baseline evidence before deeper expo work.",
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
        body: "The reset path has moved beyond a gentle behavioral shortcut into a more bench-facing transistor study.",
        detail: "The current useful region stays stable around the 2.2 k to 4.7 k base-drive area, with extreme weak drive failing.",
    },
];

pub const LAB_NEXT_STEPS: &[&str] = &[
    "Keep P1 focused on the first oscillator block before turning the work into a full voice board.",
    "Deepen the expo/integrator path only where simulation and KiCad capture already point to a concrete question.",
    "Use the public repo as the source trail for hardware evidence.",
];

pub const PRODUCT_LINES: &[ProductLine] = &[
    ProductLine {
        code: "EPM1",
        title: "Software runtime",
        summary: "Rust runtime for the current software instrument: standalone execution, factory patches, live-set behavior, and PC4-oriented MIDI work.",
        status: "Active: runnable software instrument",
        repo_path: "/home/dev/sel4/mamut-sint-sw",
        repo: RepoKind::Epm1,
        bullets: &[
            "Core workspace is implemented: params, patch, identity, DSP, engine, and standalone.",
            "Sprint 3 hardening and Sprint 4 runtime work are done; Sprint 6 centers the PC4 rig.",
            "Transport is locally frozen pending shared platform extraction.",
        ],
        source_path: "README.md",
    },
    ProductLine {
        code: "EPM2",
        title: "Hardware track",
        summary: "Hardware program for the later physical instrument: desktop analog poly direction, digital control work, and a bench-oriented path anchored in simulation and KiCad.",
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
    title: "Working notes and design records.",
    summary: "Short notes from the current software runtime, hardware study path, and related rig work.",
};

pub const PC4_BRIDGE: AdjacentProject = AdjacentProject {
    kicker: "Related repo",
    title: "PC4 Microkit Studio",
    summary: "A separate repo for PC4 rig integration, playback control, session evidence, and local-first orchestration around the performance setup.",
    repo_path: "/home/dev/sel4/pc4-microkit-studio",
    bullets: &[
        "Keeps rig orchestration and session artifacts outside the instrument repo.",
        "Covers playback control, authority boundaries, and hardware bench workflow around the PC4 setup.",
        "Dedicated infrastructure for the performance setup.",
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
        body: "The engine keeps groove intent and drummer behavior above raw MIDI events, then lowers the decision into a PC4-playable performance.",
        detail: "Semantic groove representation before MIDI output.",
    },
    LabCard {
        label: "Play",
        title: "PC4 rig loop",
        body: "The current operator path sends AIG/ADG drum decisions through mioXM to the Kurzweil PC4, then monitors the result through the Yamaha AG03 audio path.",
        detail: "Local MIDI and audio playback tied to the real rig.",
    },
    LabCard {
        label: "Authority",
        title: "Manual corpus",
        body: "The system treats selected manual material, live controls, traces, and comparisons as authority instead of presenting every variation as equally good.",
        detail: "Profile-led and intake-led decisions remain inspectable.",
    },
    LabCard {
        label: "Memory",
        title: "Feedback loop",
        body: "Training adjustments and comparison records let the drummer companion remember what worked without taking authorship away from the player.",
        detail: "Learning is framed as correction and preference under player authority.",
    },
];

pub const DRUM_ENGINE_TRACKS: &[DrumEngineTrack] = &[
    DrumEngineTrack {
        title: "jeans-instability-experiment-moises-ai-guitar-stem",
        url: "https://soundcloud.com/mamut_studio/jeans-instability-experiment",
        note: "Moises AI guitar stem experiment take.",
        track_id: "2332306544",
    },
    DrumEngineTrack {
        title: "jeans instability release candidate 1",
        url: "https://soundcloud.com/mamut_studio/jeans-instability-release",
        note: "Featured release candidate from the reference Drum Engine live set.",
        track_id: DRUM_ENGINE_FEATURED_TRACK_ID,
    },
    DrumEngineTrack {
        title: "jeans instability jam 3",
        url: "https://soundcloud.com/mamut_studio/jeans-instability-jam-3",
        note: "Later live jam take from the same public series.",
        track_id: "2332264673",
    },
    DrumEngineTrack {
        title: "Jeans Instability v2",
        url: "https://soundcloud.com/mamut_studio/jeans-instability-v2",
        note: "Earlier version with Moises AI bass track stem added.",
        track_id: "2330361851",
    },
    DrumEngineTrack {
        title: "Jeans Instability",
        url: "https://soundcloud.com/mamut_studio/jeans-instability",
        note: "Original organic take in the public SoundCloud set.",
        track_id: "2318818655",
    },
];

pub const BLOG_POSTS: &[BlogPost] = &[
    BlogPost {
        slug: "core-stance",
        series: "Direction",
        title: "Project Boundaries",
        intro: "The project keeps software, hardware, and rig work separated enough that each part can be tested on its own.",
        body: "EPM1, EPM2, and PC4 rig work have different jobs. The software runtime should stay runnable, the hardware path should stay tied to simulation and bench evidence, and related orchestration should stay in its own repo.",
        bullets: &[
            "Software behavior should be audible and testable now.",
            "Hardware work should move through simulation, capture, and measurement.",
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
        body: "EPM1 exists so patches, macros, and performance behavior can be exercised now. EPM2 exists so the physical instrument can move through circuit study, capture, and bench evidence.",
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
        body: "The software line is focused on a standalone runtime first. It exposes patch validation, dry-run behavior, device selection, live play, and PC4 rig work before plugin or editor polish.",
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
        slug: "boundaries-and-non-goals",
        series: "Direction",
        title: "Boundaries And Roles",
        intro: "The split is useful only if each line keeps a clear job.",
        body: "EPM1 stays focused on runnable software instrument behavior. EPM2 stays focused on desktop hardware study and bench evidence. PC4 rig orchestration stays separate from both.",
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
        body: "The software line is decomposed into crates for parameters, patches, identity terms, DSP, engine behavior, and standalone execution. That makes runtime behavior easier to inspect and test.",
        bullets: &[
            "Stable parameter and macro registry.",
            "Canonical TOML patch model and validation.",
            "Shared DSP, engine logic, and standalone runtime.",
        ],
    },
    BlogPost {
        slug: "pc4-controller-map-validated",
        series: "System",
        title: "PC4 Controller Map Validated",
        intro: "A real Kurzweil PC4 session now exercises EPM1 as a one-way MIDI-controlled software instrument.",
        body: "On April 29, 2026, the EPM1 standalone runtime was run through the live rig: Kurzweil PC4 into mioXM DIN 1, then into the Rust standalone synth, with audio out through the Yamaha AG03 on hw:1,0. The session validated the full PC4 controller surface that matters for one-way play into Mamut while keeping the detailed evidence in the EPM1 repo.",
        bullets: &[
            "Clean idle held after PC4 Global MIDI cleanup; startup trace stayed quiet across note, CC, and program-change channels.",
            "Sustain, K1-K9, S1-S9, SW1-SW9, mod wheel, pitch bend, and channel aftertouch were observed through the standalone MIDI trace.",
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
        body: "The current study loop treats NGSpice as the primary daily tool for learning behavior before expensive hardware commitments are made.",
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
        body: "The repo now carries electrically meaningful capture for P1 blocks, while intentionally leaving some areas provisional until they need more fidelity.",
        bullets: &[
            "Input tune, power reference, current integrator, threshold comparator, and reset output are already named as meaningful capture targets.",
            "Hardware organization follows bench relevance and concrete review questions.",
            "The repo layout preserves room for later board and subsystem growth.",
        ],
    },
    BlogPost {
        slug: "bench-evidence",
        series: "Build Path",
        title: "Bench Evidence",
        intro: "Simulation and capture need measurement feedback once prototypes exist.",
        body: "Bench, notes, and expected-behavior folders are in place so measurements can tighten the loop between simulation and physical reality once the next prototypes land.",
        bullets: &[
            "Bench artifacts have a dedicated home.",
            "Expected behavior lives beside measurement notes.",
            "Study logs and promoted docs can stay in sync.",
        ],
    },
    BlogPost {
        slug: "pc4-microkit-studio",
        series: "Adjacent Work",
        title: "PC4 Microkit Studio",
        intro: "PC4 Microkit Studio is related rig infrastructure, so it stays outside the EPM1 and EPM2 repos.",
        body: "PC4 Microkit Studio is a host-first, spec-first standalone repo for a studio rig. It centers typed contracts, authority boundaries, session artifacts, playback control, and hardware bench work around a PC4 setup.",
        bullets: &[
            "Separate canonical repository and separate problem frame.",
            "Related to the performance setup and kept outside the EPM1/EPM2 repo split.",
            "Surfaced here with a concise summary and outbound repo context.",
        ],
    },
];

pub const DOC_CATEGORIES: &[DocCategory] = &[
    DocCategory {
        title: "EPM1 Runtime Evidence",
        body: "Hardware-backed software-runtime evidence for the current playable EPM1 instrument.",
        docs: &[DocCard {
            title: "PC4 Full Controller Map",
            summary: "Real Kurzweil PC4 hardware validation of one-way sustain, knobs, sliders, switches, wheels, and aftertouch into EPM1.",
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
                summary: "The chosen direction for the desktop analog poly and the current V1 non-goals.",
                source_path: "docs/product-direction.md",
                repo: RepoKind::Epm2,
            },
            DocCard {
                title: "Open Questions",
                summary: "Open design questions that still need decisions or evidence.",
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
                summary: "How the identity terms map into system and implementation roles.",
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
