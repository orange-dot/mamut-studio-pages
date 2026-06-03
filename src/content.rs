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
pub struct HomeWorkArea {
    pub kicker: &'static str,
    pub title: &'static str,
    pub body: &'static str,
    pub bullets: &'static [&'static str],
    pub primary_cta: &'static str,
    pub secondary_cta: Option<&'static str>,
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
pub struct CodeExample {
    pub label: &'static str,
    pub source_path: &'static str,
    pub language: &'static str,
    pub code: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BlogPostSection {
    pub title: &'static str,
    pub body: &'static str,
    pub bullets: &'static [&'static str],
    pub examples: &'static [CodeExample],
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
    eyebrow: "Mamut Studio",
    title: "Software, drums, and PC4 rig work.",
    body: "EPM1 plays in the browser and desktop runtime. The Reactive Programmable Drum Machine turns ADG/AIG groove intent into MIDI for the Kurzweil PC4. PC4 Microkit Studio keeps the mioXM and Yamaha AG03 rig flow close to the sessions.",
    status: "Start with EPM1 or the programmable drum-machine take, then use Notes for the rig and source notes.",
    primary_cta: "Play EPM1",
    secondary_cta: "Hear drums",
};

pub const HOME_WORK_AREAS: &[HomeWorkArea] = &[
    HomeWorkArea {
        kicker: "EPM1",
        title: "Play the software instrument.",
        body: "EPM1 is the runnable instrument: browser demo, desktop runtime, patch bank, macro controls, and PC4-oriented MIDI work.",
        bullets: &[
            "Eight live-set patches and one browser render path.",
            "Patch names and macro targets match the desktop runtime.",
            "PC4 MIDI work keeps the software line close to the rig.",
        ],
        primary_cta: "Open play",
        secondary_cta: Some("Open lines"),
    },
    HomeWorkArea {
        kicker: "Drums",
        title: "Hear the programmable drum-machine take.",
        body: "ADG/AIG groove intent becomes MIDI, mioXM routes it, the Kurzweil PC4 plays it, and Yamaha AG03 monitoring brings the take back into the session.",
        bullets: &[
            "Start with the Jeans Instability release-candidate take.",
            "143 BPM, four-bar chunks, groove-led mode, and saved player controls.",
            "Feedback and taste shape the next listening pass.",
        ],
        primary_cta: "Open drum machine",
        secondary_cta: Some("Open SoundCloud"),
    },
    HomeWorkArea {
        kicker: "PC4MS",
        title: "Follow the PC4 rig flow.",
        body: "PC4 Microkit Studio keeps playback control, session files, local rig flow, and hardware bench notes outside the instrument repos.",
        bullets: &[
            "PC4, mioXM, and Yamaha AG03 stay close to the working sessions.",
            "Session files and playback control live in their own source home.",
            "The rig notes connect EPM, drums, and hardware work.",
        ],
        primary_cta: "Read PC4MS note",
        secondary_cta: Some("Open repo"),
    },
    HomeWorkArea {
        kicker: "EPM2",
        title: "Read the hardware lab.",
        body: "EPM2 follows the hardware path for the later physical instrument: P1 VCO simulation, KiCad capture, bench expectations, and public source notes.",
        bullets: &[
            "P1 VCO study and KiCad capture are the active path.",
            "Bench notes keep ramp window, reset level, sync, and range visible.",
            "The public repo keeps docs, sim, KiCad, bench notes, and tools together.",
        ],
        primary_cta: "Open lab",
        secondary_cta: Some("Browse docs"),
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
        repo_path: "mamut-sint-sw",
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
        repo_path: "mamut-sint-hw",
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
    repo_path: "pc4-microkit-studio",
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
        body: "Selected material and live controls steer the machine toward takes that fit the track.",
        detail: "Profile-led and intake-led choices stay readable.",
    },
    LabCard {
        label: "Memory",
        title: "Feedback loop",
        body: "Correction and comparison notes keep the next pass tied to what worked while the player keeps the taste call.",
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
        note: "Featured take from the saved programmable drum-machine live set.",
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
    "reactive-programmable-drum-machine",
    "adg-aig-drum-language",
    "pc4-drum-rig-flow",
    "drum-engine-feedback-taste-memory",
];

pub const BLOG_POSTS: &[BlogPost] = &[
    BlogPost {
        slug: "reactive-programmable-drum-machine",
        series: "Programmable Drums",
        title: "Reactive Programmable Drum Machine",
        intro: "Reactive Programmable Drum Machine is the PC4 rig drum system: live PC4 intake shapes the next generated MIDI pass while ADG/AIG keeps the drum decision editable before it becomes notes, gates, ticks, and velocities.",
        body: "This note follows drum-live-1780432493944, the Jeans release-candidate take. The machine reads the PC4 performance, keeps the groove decision in ADG/AIG, lowers the part to MIDI, routes it through mioXM to the Kurzweil PC4, and returns the session through the AG03/AG06 path.",
        bullets: &[
            "Uses the Jeans release-candidate take: 6:50.805 of 96 kHz stereo 24-bit PCM audio.",
            "Starts from PC4 live MIDI intake through mioXM, including 4416 captured events for the release candidate pass.",
            "Connects the 143 BPM Groove-led Reference live preset to the Jeans 11/8 PC4 rig.",
            "Keeps the generated MIDI, AIG/ADG decision layer, PC4 playback, AG03 capture, and SoundCloud listening reference tied to one take.",
        ],
    },
    BlogPost {
        slug: "adg-aig-drum-language",
        series: "Programmable Drums",
        title: "ADG And AIG Drum Language",
        intro: "ADG/AIG holds the drum-machine decision before MIDI: groove intent, gesture role, surface, density, timing feel, and phrase relation.",
        body: "AIG is the wider gesture frame. ADG is the drum dialect used by the current reactive machine. MIDI is the transport step that makes the decision playable on the Kurzweil PC4 through mioXM and audible through the AG03/AG06 path.",
        bullets: &[
            "AIG carries articulated instrument gestures and traceable musical decisions.",
            "ADG is the drum dialect for role, kind, surface, density, contact, timing, phrase role, variation identity, relationships, and protection flags.",
            "Generated MIDI carries note, channel, tick, gate, and velocity for the PC4.",
            "The release candidate keeps the player's timing and the machine response tied to the same rig pass.",
        ],
    },
    BlogPost {
        slug: "pc4-drum-rig-flow",
        series: "Programmable Drums",
        title: "PC4 Drum Rig Flow",
        intro: "PC4 Drum Rig Flow follows the live path: ADG/AIG becomes generated MIDI, mioXM carries it to the Kurzweil PC4, and AG03/AG06 monitoring returns the take.",
        body: "This note stays on the bench chain used for the programmable drum-machine takes. The same PC4, mioXM, and AG03/AG06 route is used for listening and for judging corrections.",
        bullets: &[
            "Live controls set the response posture before the generated part is lowered to MIDI.",
            "mioXM carries both the PC4 intake and the generated drum output.",
            "The PC4 plays the generated drums; AG03/AG06 brings the audio back into the session.",
            "SoundCloud references point to takes from that chain.",
        ],
    },
    BlogPost {
        slug: "drum-engine-feedback-taste-memory",
        series: "Programmable Drums",
        title: "Feedback And Taste Memory",
        intro: "Feedback And Taste Memory follows what happens after listening: selected material, accepted takes, rejected takes, and correction notes feed the next machine pass.",
        body: "This note keeps the feedback story close to the session. The player chooses the material, hears the PC4 take, and records what should stay or change.",
        bullets: &[
            "Manual corpus selection weights chosen material for the next pass.",
            "LiveGrooveState carries locked density, energy, accent cells, coupling, and phase across chunks.",
            "Comparison records mark which take won and why.",
            "Training adjustments change priorities, density, fill pressure, surface brightness, and bass coupling.",
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

pub fn canonical_blog_slug(slug: &str) -> &str {
    match slug {
        "drum-engine-companion" => "reactive-programmable-drum-machine",
        _ => slug,
    }
}

pub fn blog_post_by_slug(slug: &str) -> Option<BlogPost> {
    let slug = canonical_blog_slug(slug);
    BLOG_POSTS.iter().copied().find(|post| post.slug == slug)
}

pub fn blog_post_sections(slug: &str) -> &'static [BlogPostSection] {
    match canonical_blog_slug(slug) {
        "reactive-programmable-drum-machine" => REACTIVE_PROGRAMMABLE_DRUM_MACHINE_SECTIONS,
        "adg-aig-drum-language" => ADG_AIG_DRUM_LANGUAGE_SECTIONS,
        "pc4-drum-rig-flow" => PC4_DRUM_RIG_FLOW_SECTIONS,
        "drum-engine-feedback-taste-memory" => DRUM_ENGINE_FEEDBACK_MEMORY_SECTIONS,
        _ => &[],
    }
}

pub const NO_CODE_EXAMPLES: &[CodeExample] = &[];

pub const DRUM_ENGINE_RELEASE_EXAMPLES: &[CodeExample] = &[CodeExample {
    label: "Take metadata excerpt",
    source_path: "session-store/workbench-session/take-metadata/drum-live-1780432493944.json",
    language: "json",
    code: r#"{
  "take_id": "drum-live-1780432493944",
  "kind": "LivePass",
  "wav_file": "drum-live-1780432493944.wav",
  "rig_target": "Kurzweil PC4",
  "midi_output": "hw:CARD=mioXM,DEV=0",
  "audio_input": "hw:CARD=AG06AG03,DEV=0",
  "summary": "Reactive programmable drum-machine live performance"
}"#,
}];

pub const DRUM_ENGINE_INTAKE_EXAMPLES: &[CodeExample] = &[CodeExample {
    label: "Live MIDI intake excerpt",
    source_path: "session-store/workbench-session/midi-performance-takes/drum-live-1780432493944.json",
    language: "json",
    code: r#"{
  "take_id": "drum-live-1780432493944",
  "midi_input": "hw:CARD=mioXM,DEV=0",
  "rig_target": "Kurzweil PC4",
  "event_count": 4416,
  "events": [
    { "offset_millis": 1, "bytes": [145, 48, 94] },
    { "offset_millis": 1, "bytes": [129, 36, 47] },
    { "offset_millis": 1, "bytes": [145, 36, 63] }
  ]
}"#,
}];

pub const JEANS_PROFILE_EXAMPLES: &[CodeExample] = &[CodeExample {
    label: "Profile definition excerpt; defaults are profile fallback values",
    source_path: "support/drum-engine/profiles/jeans_11_8.toml",
    language: "toml",
    code: r#"profile_id = "jeans_11_8"
name = "Jeans Instability 11/8 Groove-Led Drummer"
ppqn = 480
output_channel = 9

[meter]
numerator = 11
denominator = 8
grouping = [2, 2, 3, 2, 2]

[defaults]
energy = 0.58
density = 0.46
risk = 0.62
fill_pressure = 0.42
surface_brightness = 0.54

[[midi_map]]
role = "kick"
kind = "anchor"
instrument = "BassDrum1"
note = 36
min_velocity = 72
max_velocity = 116
duration_ticks = 120
priority = 100"#,
}];

pub const AIG_ADG_CHAIN_EXAMPLES: &[CodeExample] = &[CodeExample {
    label: "AIG frame to ADG drum answer",
    source_path: "curated ADG/AIG sketch before MIDI lowering",
    language: "rust",
    code: r#"let intake_phrase = AigGestureFrame {
    source: "PC4 live intake",
    beat: 6.5,
    strength: 0.84,
    density: 0.82,
    relation: "answers the long 3 group",
};

let drum_answer = AdgGesture {
    id: "ride_pressure_06".to_string(),
    tick: beat_to_tick(intake_phrase.beat, profile.ppqn),
    role: AdgRole::Ride,
    kind: AdgKind::Pressure,
    strength: intake_phrase.strength,
    duration_ticks: 120,
    body: 0.42,
    transient: 0.66,
    openness: 0.38,
    density: intake_phrase.density,
    micro_offset_ticks: -6,
    velocity_delta: 4,
    protect_timing: true,
    protect_anchor: true,
    phrase_role: Some("long-group answer".to_string()),
    variation_seed: 1780432493944,
    variation_group: Some("jeans_11_8_surface".to_string()),
    surface_touch: Some(SurfaceTouch {
        attack_hint: SurfaceAttackHint::Accent,
        connection: SurfaceConnection::Connected,
        release_velocity: 34,
    }),
    reason: "answers PC4 intake density".to_string(),
};"#,
}];

pub const ADG_EVENT_EXAMPLES: &[CodeExample] = &[
    CodeExample {
        label: "ADG event before MIDI lowering",
        source_path: "derived example from the Jeans 11/8 reactive machine",
        language: "adg",
        code: r#"ADGEvent {
  beat: 6.5,
  role: "ride",
  kind: "pressure",
  strength: 0.84,
  surface: "right-hand",
  phrase_role: "long-group answer",
  protects: ["anchor", "timing"],
  relation: "answers PC4 intake density"
}"#,
    },
    CodeExample {
        label: "ADG gesture object",
        source_path: "curated Rust sketch from crates/drum-engine/src/lib.rs",
        language: "rust",
        code: r#"AdgGesture {
    id: "snare_ghost_09".to_string(),
    tick: 1440,
    role: AdgRole::Snare,
    kind: AdgKind::Ghost,
    strength: 0.36,
    duration_ticks: 90,
    body: 0.20,
    transient: 0.54,
    openness: 0.08,
    density: 0.58,
    micro_offset_ticks: 11,
    velocity_delta: -8,
    protect_timing: false,
    protect_anchor: false,
    phrase_role: Some("inside-motion".to_string()),
    variation_seed: 41,
    variation_group: Some("jeans_ghost_motion".to_string()),
    surface_touch: None,
    reason: "keeps motion under the PC4 phrase".to_string(),
}"#,
    },
];

pub const ADG_RELATION_EXAMPLES: &[CodeExample] = &[CodeExample {
    label: "Intake pressure to ADG relation",
    source_path: "curated Rust sketch from the reactive machine",
    language: "rust",
    code: r#"let kind = if intake.density > groove.locked_density + 0.12 {
    AdgKind::Pressure
} else {
    AdgKind::Breath
};

let role = match intake.dominant_register {
    RegisterBand::Low => AdgRole::Tom,
    RegisterBand::High => AdgRole::Ride,
    _ => AdgRole::Hat,
};

let response = AdgGesture {
    role,
    kind,
    strength: intake.velocity_energy.clamp(0.34, 0.92),
    density: intake.density,
    protect_anchor: groove.phase == LiveGroovePhase::Locked,
    phrase_role: Some("answers intake pressure".to_string()),
    reason: "player pushed density in the PC4 intake".to_string(),
    ..adg_defaults("intake_answer")
};"#,
}];

pub const ADG_CORRECTION_EXAMPLES: &[CodeExample] = &[CodeExample {
    label: "Player correction before MIDI",
    source_path: "curated Rust sketch from feedback controls",
    language: "rust",
    code: r#"match correction {
    PlayerCorrection::LessFillPressure => {
        for event in adg_events.iter_mut() {
            if matches!(event.role, AdgRole::Tom | AdgRole::Crash) {
                event.strength *= 0.72;
                event.density *= 0.68;
                event.reason.push_str("; fill pressure reduced after listening");
            }
        }
    }
    PlayerCorrection::TighterAnchor => {
        for event in adg_events.iter_mut() {
            if matches!(
                (event.role, event.kind),
                (AdgRole::Kick, AdgKind::Anchor)
                    | (AdgRole::Snare, AdgKind::Backbeat)
            ) {
                event.protect_anchor = true;
                event.protect_timing = true;
                event.micro_offset_ticks = 0;
            }
        }
    }
}"#,
}];

pub const ADG_MIDI_MAP_EXAMPLES: &[CodeExample] = &[CodeExample {
    label: "PC4 lowering map excerpt",
    source_path: "support/drum-engine/profiles/jeans_11_8.toml",
    language: "toml",
    code: r#"[[midi_map]]
role = "snare"
kind = "ghost"
instrument = "AcousticSnare"
note = 38
min_velocity = 34
max_velocity = 78
duration_ticks = 90
priority = 82

[[midi_map]]
role = "hat"
kind = "open"
instrument = "OpenHiHat"
note = 46
min_velocity = 42
max_velocity = 92
duration_ticks = 180
right_hand_surface = true
priority = 64

[[midi_map]]
role = "ride"
kind = "breath"
instrument = "RideCymbal1"
note = 51
min_velocity = 38
max_velocity = 88
duration_ticks = 120
right_hand_surface = true
priority = 66"#,
}];

pub const LOWERED_MIDI_EXAMPLES: &[CodeExample] = &[CodeExample {
    label: "Lowered MIDI event shape",
    source_path: "generated-live MIDI shape for PC4 channel 9",
    language: "yaml",
    code: r#"channel: 9
events:
  - tick: 960
    note: 51
    gate_ticks: 120
    velocity: 92
    source: "ride/pressure ADG response"
  - tick: 1080
    note: 38
    gate_ticks: 90
    velocity: 54
    source: "snare/ghost ADG response""#,
}];

pub const ADG_TO_MIDI_RUST_EXAMPLES: &[CodeExample] = &[
    CodeExample {
        label: "ADG gesture to MIDI",
        source_path: "curated Rust sketch from crates/drum-engine/src/lib.rs",
        language: "rust",
        code: r#"for gesture in adg_events {
    let Some(map) = find_midi_map(profile, gesture.role, gesture.kind) else {
        continue;
    };

    let velocity = apply_velocity_delta(
        lerp_u8(map.min_velocity, map.max_velocity, gesture.strength),
        gesture.velocity_delta,
    );
    let tick = resolved_gesture_tick(gesture);

    midi_events.push(GeneratedMidiEvent {
        tick,
        channel: profile.output_channel,
        kind: MidiMessageKind::NoteOn,
        data_1: map.note,
        data_2: velocity,
    });

    midi_events.push(GeneratedMidiEvent {
        tick: tick.saturating_add(map.duration_ticks),
        channel: profile.output_channel,
        kind: MidiMessageKind::NoteOff,
        data_1: map.note,
        data_2: 0,
    });
}

midi_events.sort_by(compare_midi_events);"#,
    },
    CodeExample {
        label: "Generated MIDI event shape",
        source_path: "curated Rust sketch for PC4 channel 9",
        language: "rust",
        code: r#"let adg = AdgGesture {
    tick: 960,
    role: AdgRole::Ride,
    kind: AdgKind::Pressure,
    strength: 0.84,
    duration_ticks: 120,
    micro_offset_ticks: -6,
    velocity_delta: 4,
    phrase_role: Some("long-group answer".to_string()),
    reason: "answers PC4 intake density".to_string(),
    ..adg_defaults("ride_pressure_06")
};

let pc4_midi = [
    GeneratedMidiEvent {
        tick: 954,
        channel: 9,
        kind: MidiMessageKind::NoteOn,
        data_1: 53,
        data_2: 92,
    },
    GeneratedMidiEvent {
        tick: 1074,
        channel: 9,
        kind: MidiMessageKind::NoteOff,
        data_1: 53,
        data_2: 0,
    },
];"#,
    },
    CodeExample {
        label: "Generated drum MIDI file excerpt",
        source_path: "session-store/workbench-session/generated-drum-midi-takes/drum-live-1780432493944.generated-live.mid",
        language: "midi",
        code: r#"file: Standard MIDI data (format 0), 1 track, 480 PPQN
track_name: "jeans_11_8 live generated drums"

header_hex:
  4d 54 68 64 00 00 00 06 00 00 00 01 01 e0  # MThd, format 0, 1 track, 480 PPQN
  4d 54 72 6b 00 00 87 29                    # MTrk, generated drum track

meta_events:
  ff 51 03 06 66 fc        # tempo meta event
  ff 58 04 0b 03 18 08     # 11/8 meter marker

first_note_on_bytes:
  99 24 6a  # note 36, velocity 106
  99 26 35  # note 38, velocity 53
  99 3b 51  # note 59, velocity 81"#,
    },
];

pub const RIG_CHAIN_RUST_EXAMPLES: &[CodeExample] = &[CodeExample {
    label: "mioXM output loop",
    source_path: "curated Rust sketch from binaries/pc4ms-workbench/src/runtime.rs",
    language: "rust",
    code: r#"let midi_input = "hw:CARD=mioXM,DEV=0";
let midi_output = "hw:CARD=mioXM,DEV=0";
let audio_input = "hw:CARD=AG06AG03,DEV=0";

let input = Rawmidi::new(midi_input, Direction::Capture, true)?;
let output = Rawmidi::new(midi_output, Direction::Playback, false)?;
let mut input_io = input.io();
let mut output_io = output.io();

let recording = audio_capture.start_take(take_id, audio_input)?;
let mut captured_events = Vec::new();
let mut pending_events = Vec::new();

if let Some(event) = drum_engine_live_keyboard_event(live_tick, &input_bytes) {
    captured_events.push(event);
}

pending_events.extend(generated_chunk.run.midi_events);

while pending_index < pending_events.len()
    && pending_events[pending_index].tick <= live_tick
{
    let event = &pending_events[pending_index];
    let status = match event.kind {
        MidiMessageKind::NoteOn => 0x90 | event.channel.min(15),
        MidiMessageKind::NoteOff => 0x80 | event.channel.min(15),
    };
    let bytes = [status, event.data_1.min(127), event.data_2.min(127)];
    output_io.write_all(&bytes)?;
    pending_index += 1;
}

let recorded_take = audio_capture.stop_take(recording)?;"#,
}];

pub const JEANS_PROFILE_METHOD_EXAMPLES: &[CodeExample] = &[CodeExample {
    label: "Profile method map",
    source_path: "derived profile method for jeans_11_8",
    language: "yaml",
    code: r#"concept_law:
  support_scale: "space, note length, articulation, and breath"
  density_mass: "event weight, low-end occupation, and accumulation pressure"
  threshold: "support still exists, but phrase pressure begins to exceed it"
  collapse: "motion contracts toward fewer anchors"
  fragmentation: "one field splits into related competing centers"

meter_law:
  meter: "11/8"
  grouping: [2, 2, 3, 2, 2]
  unstable_cell: "central 3"
  identity_rule: "keep the aksak body readable before adding pressure"

form_phases:
  - diffuse_medium
  - pressure_support
  - threshold
  - support_failure
  - collapse_onset
  - fragmentation_core
  - compact_residue

drum_roles:
  low_anchor_roles: ["kick", "low_floor_tom", "low_tom"]
  right_hand_surface: "ride and closed hat behave as one moving surface"
  tom_constellation: "six tom voices become collapse vectors"
  rejection_rule: "reject density that feels decorative instead of structural""#,
}];

pub const ADG_PROFILE_LAW_EXAMPLES: &[CodeExample] = &[CodeExample {
    label: "Profile law carried by ADG",
    source_path: "ADG law packet before MIDI lowering",
    language: "adg",
    code: r#"ADGProfileLaw {
  meter: "11/8",
  grouping: [2, 2, 3, 2, 2],
  phase: "threshold",
  pressure: {
    support: 0.48,
    density_mass: 0.72,
    inward_pull: 0.68
  },
  role_policy: {
    anchors: ["kick", "low_tom"],
    surface: ["ride", "closed_hat"],
    flashes: ["hi_mid_tom", "high_tom"]
  },
  transformation: "support field begins to contract",
  lowering_target: "PC4 MIDI channel 9"
}"#,
}];

pub const REACTIVE_LOOP_RUST_EXAMPLES: &[CodeExample] = &[
    CodeExample {
        label: "Groove state decision",
        source_path: "curated Rust sketch from crates/drum-engine/src/lib.rs",
        language: "rust",
        code: r#"let observation = observe_live_groove(
    chunk_input,
    profile,
    bar_ticks,
    source_bar_count,
    state.locked_density,
    state.locked_velocity_energy,
)?;

let previous_bars = state.bars_observed;
state.bars_observed = state.bars_observed.saturating_add(source_bar_count);

if state.bars_observed < LIVE_GROOVE_LEARNING_BARS {
    state.phase = LiveGroovePhase::Learning;
    state.locked_density = blend_live_groove_value(
        state.locked_density,
        observation.density,
    );
    state.locked_accent_cells = merge_live_groove_accent_cells(
        &state.locked_accent_cells,
        &observation.accent_cells,
    );
    return Ok(state);
}

if previous_bars < LIVE_GROOVE_LEARNING_BARS {
    state.phase = LiveGroovePhase::Locked;
    state.locked_density = observation.density;
    state.locked_velocity_energy = observation.velocity_energy;
    state.locked_accent_cells = observation.accent_cells;
    return Ok(state);
}

for shift in observation.bar_shifts.iter().copied() {
    match (state.dynamic_shift_candidate, shift) {
        (Some(current), Some(next)) if current == next => {
            state.dynamic_shift_bars = state.dynamic_shift_bars.saturating_add(1);
        }
        (_, Some(next)) => {
            state.dynamic_shift_candidate = Some(next);
            state.dynamic_shift_bars = 1;
        }
        (_, None) => {
            state.dynamic_shift_candidate = None;
            state.dynamic_shift_bars = 0;
        }
    }

    if state.dynamic_shift_bars >= LIVE_GROOVE_DYNAMIC_CONFIRM_BARS {
        if let Some(shift) = state.dynamic_shift_candidate {
            apply_live_groove_dynamic_shift(&mut state, shift, &observation);
            state.last_dynamic_shift = Some(shift);
        }
    }
}

if observation.bravura_triggered && state.bravura_cooldown_bars == 0 {
    state.phase = LiveGroovePhase::Bravura;
}"#,
    },
    CodeExample {
        label: "Response event gate",
        source_path: "curated Rust sketch from crates/drum-engine/src/lib.rs",
        language: "rust",
        code: r#"if state.phase == LiveGroovePhase::Bravura
    || !live_groove_is_intake_mirror(event)
{
    return true;
}

if matches!(
    (event.role, event.kind),
    (AdgRole::Kick, AdgKind::Anchor)
        | (AdgRole::Snare, AdgKind::Backbeat)
) {
    return true;
}

let cell = (resolved_gesture_tick(event) % bar_ticks.max(1)) / grid_ticks.max(1);
if state.locked_accent_cells.contains(&cell)
    && !matches!(event.role, AdgRole::Tom)
    && !matches!(event.kind, AdgKind::Flash)
{
    return true;
}

let threshold = (state.coupling * 100.0).round() as u32;
live_groove_event_hash(&event.id) % 100 < threshold"#,
    },
];

pub const REACTIVE_PROGRAMMABLE_DRUM_MACHINE_SECTIONS: &[BlogPostSection] = &[
    BlogPostSection {
        title: "Release candidate take",
        body: "drum-live-1780432493944 is the Jeans release-candidate take. The WAV is a 6:50.805 live pass in stereo 96 kHz 24-bit PCM. The session names the rig target as Kurzweil PC4.",
        bullets: &[
            "The take metadata points to the PC4 live rig.",
            "The same take id has a generated-live MIDI file and a human MIDI intake JSON file.",
            "The SoundCloud reference stays tied to one session, one rig, and one generated drum pass.",
        ],
        examples: DRUM_ENGINE_RELEASE_EXAMPLES,
    },
    BlogPostSection {
        title: "Live MIDI intake",
        body: "The machine starts with the live intake. For this release candidate, the matching performance JSON records 4416 MIDI events from hw:CARD=mioXM,DEV=0. The PC4-side performance supplies timing, emphasis, density, and phrase pressure.",
        bullets: &[
            "The first captured events include note-on, note-off, and control traffic.",
            "Example note-on bytes such as status 145 with notes 48 and 36 carry live player timing and velocity.",
            "The player gives the shape; the next generated pass answers it.",
        ],
        examples: DRUM_ENGINE_INTAKE_EXAMPLES,
    },
    BlogPostSection {
        title: "Jeans 11/8 profile",
        body: "The release candidate keeps the Jeans 11/8 identity. The active profile is jeans_11_8, uses an 11/8 meter grouped as 2+2+3+2+2, runs at PPQN 480, and targets MIDI channel 9 for generated drums.",
        bullets: &[
            "The meter identity is Jeans-style 11/8.",
            "The MIDI map keeps the PC4 target concrete: kick anchor 36, snare 38, hats 42/44/46, ride 51/59/53, crash 49, and toms 45/48/41.",
            "The TOML defaults below are profile fallback posture.",
            "The featured live take uses the Reference live preset values listed below.",
        ],
        examples: JEANS_PROFILE_EXAMPLES,
    },
    BlogPostSection {
        title: "Profile method",
        body: "The Jeans profile starts with a threshold system. Support holds, density gathers, support fails, collapse contracts the bar, and fragmentation splits the same field into related centers.",
        bullets: &[
            "The first law is the 11/8 aksak body: 2+2+3+2+2 stays readable before pressure or variation enters.",
            "The form moves through diffuse medium, pressure support, threshold, support failure, collapse onset, fragmentation core, and compact residue.",
            "Kick and low toms carry gravity. Ride and closed hat carry the right-hand surface. The tom constellation carries collapse.",
            "The PC4 map comes last: PPQN, channel, MIDI notes, velocity ranges, durations, and fallback defaults.",
        ],
        examples: JEANS_PROFILE_METHOD_EXAMPLES,
    },
    BlogPostSection {
        title: "Reference live preset",
        body: "The featured take uses the saved startup profile from the PC4, mioXM, and Yamaha AG03 drum-machine session: 143 BPM, four-bar chunks, Groove-led mode, energy 0.88, density 0.82, risk 0.95, fill 0.60, surface 0.80, humanize 0.42, timing 0.19, velocity 0.92, anti-repeat 0.90, and loose 0.20.",
        bullets: &[
            "Tempo and chunk size set the live window: 143 BPM over four-bar chunks.",
            "Groove-led mode is the active posture for the featured take; the engine reacts while preserving the groove center.",
            "Energy, density, risk, velocity, and anti-repeat are set high for an assertive pass.",
        ],
        examples: NO_CODE_EXAMPLES,
    },
    BlogPostSection {
        title: "Reactive loop",
        body: "Groove-led mode keeps the machine centered on the live-set groove. The captured PC4 performance supplies timing, emphasis, density, and phrase pressure for the next response. The Rust sketch shows the decision point: learn the groove, lock it, accept a shift, or open a bravura response.",
        bullets: &[
            "The engine reads phrase pressure from the intake: where the player leans, repeats, leaves space, or pushes density.",
            "The Reference live preset constrains how far the machine can answer with fill, surface, risk, timing, velocity, anti-repeat, and looseness.",
            "The next chunk is a musical response first, then a MIDI file.",
        ],
        examples: REACTIVE_LOOP_RUST_EXAMPLES,
    },
    BlogPostSection {
        title: "Generated drum MIDI",
        body: "The matching generated-live MIDI file is the transport file. Its header identifies a jeans_11_8 live generated drums track, uses PPQN 480, carries an 11/8 meter marker, and produces note events that the PC4 can play. The Rust sketch shows the lowering step: ADG role and kind select the PC4 note, strength becomes velocity, and duration becomes note-off time.",
        bullets: &[
            "MIDI is where the engine becomes playable on the hardware rig.",
            "The PC4 receives notes, ticks, gates, and velocities.",
            "ADG/AIG stays beside the generated MIDI as the editable drum decision.",
        ],
        examples: ADG_TO_MIDI_RUST_EXAMPLES,
    },
    BlogPostSection {
        title: "PC4, mioXM, and AG03 chain",
        body: "The canonical bench profile is PC4 Live: MIDI input and output are hw:CARD=mioXM,DEV=0, the rig target is Kurzweil PC4, and the capture path is AG03/AG06. In practical terms, the player performs into the PC4/mioXM path, the generated drums route back to the PC4, and the audio returns through the AG03/AG06 monitoring and recording chain. The Rust sketch shows the live route: capture PC4 MIDI, queue generated drum events, encode bytes, and write them to mioXM for the PC4.",
        bullets: &[
            "mioXM is the MIDI bridge between the live performance path and the generated drum output path.",
            "The PC4 plays the drum part through the hardware rig.",
            "AG03/AG06 capture brings the take back as session audio.",
        ],
        examples: RIG_CHAIN_RUST_EXAMPLES,
    },
    BlogPostSection {
        title: "SoundCloud reference",
        body: "The SoundCloud release candidate is the public listening point for this working direction. It carries the same take id, live intake, generated MIDI, PC4 playback, and AG03/AG06 capture path.",
        bullets: &[
            "One focused take is easier to judge than a catalogue of possible features.",
            "The public take is a release candidate for this rig direction.",
            "Follow-up passes can compare accepted and rejected takes against the same intake and generated MIDI.",
        ],
        examples: NO_CODE_EXAMPLES,
    },
];

pub const ADG_AIG_DRUM_LANGUAGE_SECTIONS: &[BlogPostSection] = &[
    BlogPostSection {
        title: "AIG, ADG, and MIDI",
        body: "AIG carries the wider gesture frame. ADG is the drum dialect inside it. MIDI sends the playable part to the PC4.",
        bullets: &[
            "AIG carries timing, strength, relationships, constraints, and trace.",
            "ADG specializes that object for drums: kick anchors, snare ghosts, hat breath, ride pressure, crash flash, tom movement, and phrase roles.",
            "MIDI keeps what the PC4 needs to play: channel, note number, tick, duration, and velocity.",
        ],
        examples: AIG_ADG_CHAIN_EXAMPLES,
    },
    BlogPostSection {
        title: "ADG event shape",
        body: "An ADG drum event names the musical job before it becomes a note. Role and kind come first, then strength, surface, timing feel, phrase role, and variation identity.",
        bullets: &[
            "Role says what part of the kit or drum-machine job is active: kick, snare, hat, ride, crash, or tom.",
            "Kind says what musical job that role is doing: anchor, backbeat, ghost, breath, open, pressure, flash, and similar dialect terms.",
            "The extra fields keep the take editable after listening.",
        ],
        examples: ADG_EVENT_EXAMPLES,
    },
    BlogPostSection {
        title: "Profile laws in ADG",
        body: "ADG keeps the Jeans profile laws available before MIDI export. The engine can name phase, pressure, surface, anchor, and transformation, then send the playable part to the PC4.",
        bullets: &[
            "Support, threshold, collapse, and fragmentation steer the machine response.",
            "Right-hand surface and tom constellation choices stay visible before they become note numbers.",
            "A later pass can tighten anchors, reduce surface pressure, or change fragmentation without hand-editing the whole MIDI file.",
        ],
        examples: ADG_PROFILE_LAW_EXAMPLES,
    },
    BlogPostSection {
        title: "From intake to response",
        body: "For drum-live-1780432493944, the live MIDI intake has 4416 events from the PC4/mioXM path. The reactive machine answers that performance, then lowers the answer to generated MIDI.",
        bullets: &[
            "The player gives timing, pressure, repetition, and velocity shape through the PC4.",
            "The reactive engine reads that shape against the Jeans 11/8 phrase law and the 143 BPM Groove-led Reference live preset.",
            "The ADG decision then describes the machine answer before the answer is reduced to PC4 MIDI.",
        ],
        examples: ADG_RELATION_EXAMPLES,
    },
    BlogPostSection {
        title: "11/8 phrase law",
        body: "The Jeans 11/8 profile uses an 11/8 grouping of 2+2+3+2+2. The machine follows phrase shape, not just equal grid cells.",
        bullets: &[
            "A kick anchor can hold the body of the phrase without making every strong point equally heavy.",
            "Hat and ride surfaces can explain the meter while leaving room for the player's own timing.",
            "Fills and flashes can answer the long group without erasing the 2+2+3+2+2 feel.",
        ],
        examples: NO_CODE_EXAMPLES,
    },
    BlogPostSection {
        title: "Kick, snare, and hat examples",
        body: "The PC4 lowering map makes the dialect concrete. A kick anchor lowers to note 36 with a strong velocity range. A snare backbeat or ghost lowers to note 38, but the ghost version uses a softer range. Hat breath lowers to note 42, hat pressure to note 44, and open hat to note 46.",
        bullets: &[
            "The same MIDI note can carry different ADG meaning before it is lowered; snare backbeat and snare ghost are both note 38 on the PC4 path.",
            "A hat breath is a surface decision, while a hat pressure or choke is a relation/contact decision.",
            "ADG lets the player ask for more breath, less pressure, or tighter ghosting without editing every event by hand.",
        ],
        examples: ADG_MIDI_MAP_EXAMPLES,
    },
    BlogPostSection {
        title: "Ride, crash, and tom examples",
        body: "The Jeans 11/8 map also gives the right-hand surface and fill system more vocabulary. Ride breath lowers to note 51, ride open to 59, ride pressure to 53, crash flash to 49, low tom pressure to 45, high-mid tom flash to 48, and low floor tom to 41.",
        bullets: &[
            "Ride choices can keep the 11/8 motion visible when hats are too narrow for the phrase.",
            "Crash flash is a phrase marker, not just a loud cymbal hit.",
            "Tom movement can carry pressure and answer behavior while staying connected to the same reactive phrase state.",
        ],
        examples: NO_CODE_EXAMPLES,
    },
    BlogPostSection {
        title: "What MIDI carries",
        body: "The generated MIDI preserves the parts needed for PC4 playback: role-to-note mapping, kind-to-note variant when the map has one, beat-to-tick timing, micro-offset-to-tick timing, velocity from strength, gate duration, and channel output.",
        bullets: &[
            "The PC4 gets the note, channel, tick, gate, and velocity.",
            "The listener hears whether the response works in the take.",
            "The ADG note keeps the machine choice readable after playback.",
        ],
        examples: LOWERED_MIDI_EXAMPLES,
    },
    BlogPostSection {
        title: "What ADG keeps",
        body: "ADG keeps the musical context around the generated MIDI: body, transient, recovery, presence, openness, density, wash, choke, timing protection, anchor protection, phrase role, variation identity, relationships, and kit model.",
        bullets: &[
            "Those fields stay readable before and after the PC4 output step.",
            "A future correction can target the ADG choice instead of treating the take as only note numbers.",
            "The generated MIDI remains the hardware path; ADG remains the working language.",
        ],
        examples: NO_CODE_EXAMPLES,
    },
    BlogPostSection {
        title: "Listening correction",
        body: "After hearing the SoundCloud release candidate, the player can ask for a tighter anchor, a more open right-hand surface, lower fill pressure, stronger ride phrase, or less anti-repeat while keeping drum-live-1780432493944 as the session reference.",
        bullets: &[
            "The edit target is musical language before individual MIDI events.",
            "The same take can keep its intake, generated MIDI, PC4 playback, and listening reference while the ADG choice changes.",
            "That is the practical difference between this reactive programmable drum machine and a simple MIDI humanizer.",
        ],
        examples: ADG_CORRECTION_EXAMPLES,
    },
];

pub const PC4_DRUM_RIG_FLOW_SECTIONS: &[BlogPostSection] = &[
    BlogPostSection {
        title: "Player path",
        body: "The player path starts at the PC4 and live controls. The profile shapes the answer, ADG/AIG keeps the decision readable, and generated MIDI carries it back to the rig.",
        bullets: &[
            "Live controls shape energy, density, risk, fill pressure, surface, humanization, and looseness.",
            "ADG/AIG carries role, kind, surface, and phrase relation before MIDI.",
            "Generated MIDI sends note, velocity, tick, and gate to the PC4.",
        ],
        examples: NO_CODE_EXAMPLES,
    },
    BlogPostSection {
        title: "Hardware chain",
        body: "The current chain is ADG/AIG decision to MIDI, mioXM routing, Kurzweil PC4 playback, and Yamaha AG03/AG06 monitoring and recording.",
        bullets: &[
            "mioXM routes generated MIDI into the hardware rig.",
            "The PC4 plays the audible drum performance for this workflow.",
            "AG03/AG06 gives the session a consistent monitoring and capture path.",
        ],
        examples: NO_CODE_EXAMPLES,
    },
    BlogPostSection {
        title: "Listening references",
        body: "The SoundCloud takes are the listening references for this chain. They make the PC4, mioXM, and AG03/AG06 route audible before the page gets into source notes.",
        bullets: &[
            "The release candidate stays the main take.",
            "Related takes show nearby choices and revisions.",
            "The page keeps one embed and a short track list.",
        ],
        examples: NO_CODE_EXAMPLES,
    },
    BlogPostSection {
        title: "What the flow connects",
        body: "The flow ties each ADG choice to the studio chain: ADG/AIG, generated MIDI, PC4 playback, AG03/AG06 capture, and listening reference.",
        bullets: &[
            "The generated drum choice reaches hardware.",
            "The output path stays specific: generated MIDI to mioXM to PC4.",
            "The listening reference comes back through AG03/AG06.",
        ],
        examples: NO_CODE_EXAMPLES,
    },
];

pub const DRUM_ENGINE_FEEDBACK_MEMORY_SECTIONS: &[BlogPostSection] = &[
    BlogPostSection {
        title: "Manual corpus",
        body: "Manual corpus selection is the machine's saved taste input. Chosen material can steer repeat reduction, surface choice, tom answers, and ghost notes in the next pass.",
        bullets: &[
            "Selected material can make a surface thinner or a tom answer more likely.",
            "A kept take can become the reference for the next pass.",
            "The player keeps control of what enters that memory.",
        ],
        examples: NO_CODE_EXAMPLES,
    },
    BlogPostSection {
        title: "Live groove state",
        body: "LiveGrooveState is the chunk memory: locked density, velocity energy, accent cells, coupling, confidence, and the current response phase.",
        bullets: &[
            "Learning chunks listen for density, velocity, and accent cells.",
            "Locked chunks keep the groove center stable.",
            "Bravura chunks can open the response while the locked groove remains the reference.",
        ],
        examples: NO_CODE_EXAMPLES,
    },
    BlogPostSection {
        title: "Training records",
        body: "Training records store listening decisions after a pass: what won, what lost, and what should change next.",
        bullets: &[
            "Feedback can target a whole take, one ADG event, a bass event, or a comparison side.",
            "Comparison records keep the winner, confidence, take refs, and diff summary.",
            "Training adjustments stay tied to the current profile.",
        ],
        examples: NO_CODE_EXAMPLES,
    },
    BlogPostSection {
        title: "Session loop",
        body: "A useful pass stays tied to one chain: profile, live controls, ADG events, generated MIDI, PC4 playback, AG03/AG06 capture, and the next correction.",
        bullets: &[
            "The listener hears the PC4 take first.",
            "The correction targets ADG, MIDI, or the selected material.",
            "The next pass keeps the take history close to the session.",
        ],
        examples: NO_CODE_EXAMPLES,
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
