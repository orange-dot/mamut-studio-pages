use crate::content::{
    BLOG_INTRO, BLOG_POSTS, DOC_CATEGORIES, DRUM_ENGINE_EVIDENCE, DRUM_ENGINE_FEATURED_TRACK_ID,
    DRUM_ENGINE_NOTE_SLUGS, DRUM_ENGINE_PRESET_CONTROLS, DRUM_ENGINE_TRACKS,
    DRUM_STUDIO_ARTIFACT_CARDS, DRUM_STUDIO_AUTHORITY_CARDS, DRUM_STUDIO_INTRO,
    DRUM_STUDIO_NOTE_SLUGS, EPM2_PUBLIC_REPO_URL, HERO, HOME_WORK_AREAS, LAB_INTRO, LAB_NEXT_STEPS,
    LAB_RESULTS, LAB_STAGES, PRODUCT_LINES, PRODUCTS_INTRO, RepoKind, blog_post_by_slug,
    blog_post_sections, repo_root_url, source_url,
};
use crate::play::PlayPage;
use dioxus::prelude::*;

const SITE_NAME: &str = "Mamut EPM";
const SITE_DESCRIPTION: &str = "Public home for Mamut Studio: play EPM1, hear programmable drums, follow Drum Studio, and read the EPM2 hardware path.";
const DEFAULT_SITE_BASE_URL: &str = "https://mamut-studio.com";
const PREVIEW_IMAGE_PATH: &str = "/og-default.png";
const PREVIEW_IMAGE_WIDTH: &str = "3000";
const PREVIEW_IMAGE_HEIGHT: &str = "3000";
const PREVIEW_IMAGE_ALT: &str = "Mamut Studio circular signal artwork";
#[cfg(target_arch = "wasm32")]
const THEME_STORAGE_KEY: &str = "mamut-theme";
const HERO_DRUM_FLOW_STEPS: [HeroDrumFlowStep; 12] = [
    HeroDrumFlowStep::active("MIDI intake", "Live input"),
    HeroDrumFlowStep::active("Played part", "Your timing"),
    HeroDrumFlowStep::active("Groove-led", "Lock + adapt"),
    HeroDrumFlowStep::inactive("AIG frame", "Gesture intent"),
    HeroDrumFlowStep::active("ADG dialect", "Drum decision"),
    HeroDrumFlowStep::inactive("Surface", "Density + fills"),
    HeroDrumFlowStep::inactive("Timing feel", "Humanize"),
    HeroDrumFlowStep::active("Reactive state", "Next chunk"),
    HeroDrumFlowStep::active("Bundle", "Trace + export"),
    HeroDrumFlowStep::active("MIDI lower", "Notes + velocity"),
    HeroDrumFlowStep::active("MIDI out", "Target route"),
    HeroDrumFlowStep::active("Synth + capture", "Play + record"),
];
const HERO_DRUM_CHAIN_CARDS: [HeroDrumChainCard; 4] = [
    HeroDrumChainCard {
        label: "Input",
        value: "MIDI live intake",
    },
    HeroDrumChainCard {
        label: "Engine",
        value: "Groove-led machine",
    },
    HeroDrumChainCard {
        label: "Files",
        value: "ADG bundle + AIG export",
    },
    HeroDrumChainCard {
        label: "Output",
        value: "MIDI out -> synth target -> capture",
    },
];

#[derive(Clone, Copy)]
struct HeroDrumFlowStep {
    label: &'static str,
    value: &'static str,
    active: bool,
}

impl HeroDrumFlowStep {
    const fn active(label: &'static str, value: &'static str) -> Self {
        Self {
            label,
            value,
            active: true,
        }
    }

    const fn inactive(label: &'static str, value: &'static str) -> Self {
        Self {
            label,
            value,
            active: false,
        }
    }
}

#[derive(Clone, Copy)]
struct HeroDrumChainCard {
    label: &'static str,
    value: &'static str,
}

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[redirect("/control-plane", || Route::Home {})]
    #[route("/")]
    Home {},
    #[redirect("/blog", || Route::Notes {})]
    #[route("/notes")]
    Notes {},
    #[redirect("/blog/:slug", |slug: String| Route::NotePost { slug })]
    #[route("/notes/:slug")]
    NotePost { slug: String },
    #[redirect("/products", || Route::Lines {})]
    #[route("/lines")]
    Lines {},
    #[route("/lab")]
    Lab {},
    #[route("/docs")]
    Docs {},
    #[route("/play")]
    Play {},
    #[redirect("/drum-engine", || Route::DrumEngine {})]
    #[route("/programmable-drum-machine")]
    DrumEngine {},
    #[route("/drum-studio")]
    DrumStudio {},
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ThemeMode {
    Dark,
    Light,
}

impl ThemeMode {
    #[cfg(target_arch = "wasm32")]
    fn from_storage_value(value: &str) -> Option<Self> {
        match value {
            "dark" => Some(Self::Dark),
            "light" => Some(Self::Light),
            _ => None,
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Dark => "Dark",
            Self::Light => "Light",
        }
    }

    const fn shell_class(self) -> &'static str {
        match self {
            Self::Dark => "theme-dark",
            Self::Light => "theme-light",
        }
    }

    #[cfg(target_arch = "wasm32")]
    const fn storage_value(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }

    #[cfg(target_arch = "wasm32")]
    const fn theme_color(self) -> &'static str {
        match self {
            Self::Dark => "#090b10",
            Self::Light => "#f4f5f1",
        }
    }

    const fn toggled(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }
}

#[derive(Clone, Copy)]
struct ThemeController {
    mode: Signal<ThemeMode>,
}

#[component]
pub fn App() -> Element {
    let theme = use_signal(initial_theme_mode);
    use_context_provider(|| ThemeController { mode: theme });

    rsx! { Router::<Route> {} }
}

#[component]
fn Home() -> Element {
    rsx! {
        PageFrame {
            title: SITE_NAME.to_string(),
            description: SITE_DESCRIPTION.to_string(),
            current: Route::Home {},
            HeroSection {}
            section { class: "signal-section",
                div { class: "section-copy",
                    span { class: "section-kicker", "Current work" }
                    h2 { "EPM1, drums, MIDI targets, EPM2." }
                    p { "EPM1 plays in the browser. The Reactive Programmable Drum Machine turns ADG/AIG drum decisions into generated MIDI, Drum Studio keeps the session files readable, and EPM2 carries the hardware path." }
                }
                div { class: "feature-grid",
                    for area in HOME_WORK_AREAS {
                        {home_work_card(area)}
                    }
                }
            }
            DrumEngineCaseSection {}
            HomeUtilitySection {}
        }
    }
}

#[component]
fn Notes() -> Element {
    rsx! {
        PageFrame {
            title: "Notes".to_string(),
            description: "Working notes from the software runtime, hardware study path, and related source work.".to_string(),
            current: Route::Notes {},
            PageIntroBlock {
                kicker: BLOG_INTRO.kicker,
                title: BLOG_INTRO.title,
                summary: BLOG_INTRO.summary,
            }
            section { class: "blog-grid",
                for post in BLOG_POSTS {
                    BlogCardView { post: *post }
                }
            }
        }
    }
}

#[component]
fn NotePost(slug: String) -> Element {
    let post = blog_post_by_slug(&slug);
    let canonical_slug = post
        .map(|post| post.slug.to_string())
        .unwrap_or_else(|| slug.clone());
    let title = post
        .map(|post| post.title.to_string())
        .unwrap_or_else(|| "Note".to_string());
    let description = post
        .map(|post| post.body.to_string())
        .unwrap_or_else(|| "Requested note was not found.".to_string());
    let sections = post
        .map(|post| blog_post_sections(post.slug))
        .unwrap_or(&[]);

    rsx! {
        PageFrame {
            title: title,
            description: description,
            current: Route::NotePost {
                slug: canonical_slug,
            },
            if let Some(post) = post {
                section { class: "post-shell",
                    Link { class: "source-link post-back", to: Route::Notes {}, "Back to notes" }
                    section { class: "page-intro post-intro",
                        span { class: "eyebrow", "Note / {post.series}" }
                        h1 { "{post.title}" }
                        p { "{post.intro}" }
                    }
                    article { class: "detail-panel post-panel",
                        div { class: "card-topline", "{post.series}" }
                        p { "{post.body}" }
                        if !sections.is_empty() {
                            for section in sections {
                                section { class: "post-section",
                                    h2 { "{section.title}" }
                                    p { "{section.body}" }
                                    ul {
                                        for bullet in section.bullets {
                                            li { "{bullet}" }
                                        }
                                    }
                                    if post.slug == "pc4ms-touch-surface-live-rig" && section.title == "Tablet control surface" {
                                        figure { class: "note-image-panel",
                                            img {
                                                class: "note-image",
                                                src: "/pc4ms-touch-surface-live-rig-app-2026-06-03.jpg",
                                                alt: "Android tablet running the PC4MS touch-control surface on the PC4 and mioXM live rig",
                                                width: "1600",
                                                height: "720",
                                                loading: "lazy",
                                                decoding: "async",
                                            }
                                            figcaption {
                                                "Android touch-control surface running on the tablet while the PC4 and mioXM rig is active. The sliders send macro MIDI into Workbench without taking over the performer MIDI path."
                                            }
                                        }
                                    }
                                    if !section.examples.is_empty() {
                                        div { class: "code-example-stack",
                                            for example in section.examples {
                                                figure { class: "code-example",
                                                    figcaption { class: "code-example-head",
                                                        span { class: "code-example-label", "{example.label}" }
                                                        span { class: "code-example-language", "{example.language}" }
                                                    }
                                                    code { class: "code-example-path", "{example.source_path}" }
                                                    pre { class: "code-example-body",
                                                        code { "{example.code}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        h2 { "Key points" }
                        ul {
                            for bullet in post.bullets {
                                li { "{bullet}" }
                            }
                        }
                        if matches!(post.series, "Programmable Drums" | "Drum Studio" | "AIG / ADG" | "PC4MS" | "Research") {
                            div { class: "post-actions",
                                Link { class: "button button-primary", to: Route::DrumEngine {}, "Open drum-machine case" }
                                Link { class: "button button-secondary", to: Route::DrumStudio {}, "Open Drum Studio" }
                                Link { class: "source-link", to: Route::Notes {}, "All notes" }
                            }
                        }
                    }
                }
            } else {
                section { class: "post-shell",
                    section { class: "page-intro post-intro",
                        span { class: "eyebrow", "Note" }
                        h1 { "Requested note was not found." }
                        p { "The route exists, but there is no current note behind this slug." }
                    }
                    Link { class: "button button-secondary", to: Route::Notes {}, "Back to notes" }
                }
            }
        }
    }
}

#[component]
fn Lines() -> Element {
    rsx! {
        PageFrame {
            title: "Lines".to_string(),
            description: "Current split between EPM1, the runnable software instrument, and EPM2, the hardware build track.".to_string(),
            current: Route::Lines {},
            PageIntroBlock {
                kicker: PRODUCTS_INTRO.kicker,
                title: PRODUCTS_INTRO.title,
                summary: PRODUCTS_INTRO.summary,
            }
            section { class: "products-grid",
                for line in PRODUCT_LINES {
                    ProductCardView { line: *line }
                }
            }
        }
    }
}

#[component]
fn Lab() -> Element {
    rsx! {
        PageFrame {
            title: "Lab".to_string(),
            description: "EPM2 hardware lab: P1 VCO simulation, KiCad capture, bench expectations, and public source notes.".to_string(),
            current: Route::Lab {},
            PageIntroBlock {
                kicker: LAB_INTRO.kicker,
                title: LAB_INTRO.title,
                summary: LAB_INTRO.summary,
            }
            section { class: "utility-band",
                div { class: "section-copy",
                    span { class: "section-kicker", "Public source" }
                    h2 { "EPM2 hardware source." }
                    p { "The EPM2 public repo carries docs, ngspice studies, KiCad capture, bench expectations, and helper tools. Deploy config, investor notes, and local workspace state stay outside that repo." }
                }
                div { class: "utility-links",
                    a {
                        class: "button button-primary",
                        href: EPM2_PUBLIC_REPO_URL,
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "Open hardware repo"
                    }
                    Link { class: "button button-secondary", to: Route::Docs {}, "Browse docs" }
                }
            }
            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "Path" }
                    h2 { "Simulation, capture, bench." }
                    p { "The lab page follows the hardware work step by step, with source notes tied to each stage." }
                }
                div { class: "products-grid",
                    for card in LAB_STAGES {
                        LabCardView { card: *card }
                    }
                }
            }
            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "P1 VCO" }
                    h2 { "Current technical references." }
                    p { "These are the main study anchors behind the first oscillator block." }
                }
                div { class: "products-grid",
                    for card in LAB_RESULTS {
                        LabCardView { card: *card }
                    }
                }
            }
            section { class: "detail-panel",
                div { class: "card-topline", "Next bench posture" }
                h2 { "Keep the first oscillator honest." }
                ul {
                    for item in LAB_NEXT_STEPS {
                        li { "{item}" }
                    }
                }
            }
        }
    }
}

#[component]
fn Docs() -> Element {
    rsx! {
        PageFrame {
            title: "Docs".to_string(),
            description: "Current EPM2 hardware source library.".to_string(),
            current: Route::Docs {},
            PageIntroBlock {
                kicker: "Docs",
                title: "Source library for the current hardware notes.",
                summary: "Docs links to the current EPM2 source material while keeping the landing page compact."
            }
            for category in DOC_CATEGORIES {
                section { class: "doc-category",
                    div { class: "section-copy",
                        span { class: "section-kicker", "{category.title}" }
                        h2 { "{category.title}" }
                        p { "{category.body}" }
                    }
                    div { class: "doc-grid",
                        for doc in category.docs {
                            DocCardView { doc: *doc }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Play() -> Element {
    rsx! {
        PageFrame {
            title: "Play".to_string(),
            description: "Offline-render browser demo built from the current Mamut EPM software runtime.".to_string(),
            current: Route::Play {},
            PlayPage {}
        }
    }
}

#[component]
fn DrumEngine() -> Element {
    let soundcloud_embed_src = format!(
        "https://w.soundcloud.com/player/?visual=true&url=https%3A%2F%2Fapi.soundcloud.com%2Ftracks%2F{}&show_artwork=true",
        DRUM_ENGINE_FEATURED_TRACK_ID
    );
    let featured_track = DRUM_ENGINE_TRACKS
        .iter()
        .find(|track| track.track_id == DRUM_ENGINE_FEATURED_TRACK_ID);

    rsx! {
        PageFrame {
            title: "Reactive Programmable Drum Machine".to_string(),
            description: "A MIDI-playable programmable drum-machine case: ADG/AIG groove intent, reactive MIDI output, ADG bundle files, and public SoundCloud takes.".to_string(),
            current: Route::DrumEngine {},
            section { class: "drum-case-hero",
                div { class: "section-copy",
                    span { class: "eyebrow", "Portfolio case" }
                    h1 { "Reactive Programmable Drum Machine" }
                    p { class: "hero-body", "A MIDI drum workflow: ADG/AIG groove intent becomes generated MIDI, the ADG bundle stays with the take, and a synth target plus capture path make the session audible." }
                    p { class: "hero-status", "Player controls shape the take, correction feeds the next pass, and the AIG handoff keeps Drum Studio and material-import roles clear." }
                    div { class: "hero-actions",
                        a {
                            class: "button button-primary",
                            href: "https://soundcloud.com/mamut_studio/jeans-instability-release",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "Listen"
                        }
                        Link { class: "button button-secondary", to: Route::DrumStudio {}, "Open Drum Studio" }
                        Link { class: "source-link hero-link", to: Route::Play {}, "Open EPM play" }
                    }
                }
                div { class: "drum-flow-panel",
                    div { class: "card-topline", "Current loop" }
                    h2 { "ADG/AIG to bundle to MIDI" }
                    p { "The player path starts from drum intent, resolves it through the active profile, writes the generated MIDI and ADG bundle, sends MIDI out, and captures the result for review." }
                    div { class: "drum-flow-steps",
                        article { class: "drum-flow-step",
                            span { "Intent" }
                            strong { "Groove shape" }
                            p { "ADG/AIG keeps density, fill pressure, surface, timing feel, and response posture readable before note output." }
                        }
                        article { class: "drum-flow-step",
                            span { "Profile" }
                            strong { "Taste layer" }
                            p { "The live preset and manual corpus steer how tightly the machine locks in, adapts, and shapes fills." }
                        }
                        article { class: "drum-flow-step",
                            span { "Bundle" }
                            strong { "Session files" }
                            p { "Generated MIDI is kept beside ADG events, summaries, live chunks, traces, runtime snapshot, and AIG export request." }
                        }
                        article { class: "drum-flow-step",
                            span { "Target + capture" }
                            strong { "MIDI audition" }
                            p { "The generated take is sent to a synth target while the capture path makes the performance chain audible and recordable." }
                        }
                    }
                }
            }

            section { class: "drum-preset-section",
                div { class: "section-copy",
                    span { class: "section-kicker", "AIG / ADG" }
                    h2 { "What the drum language keeps before MIDI." }
                    p { "AIG means Articulated Instrument Gesture: the wider model for timing, strength, relationships, protection flags, and render-ready gesture data." }
                    p { "ADG means Articulated Drum Gesture: the drum dialect that names kick anchors, snare ghosts, hat breath, ride pressure, tom motion, cymbal flashes, surface feel, and phrase role before the take becomes MIDI." }
                    Link {
                        class: "source-link",
                        to: Route::NotePost { slug: "drum-engine-aig-adg-flow".to_string() },
                        "Read full flow note"
                    }
                }
                div { class: "drum-flow-panel",
                    div { class: "card-topline", "Language path" }
                    h2 { "From intake to ADG to AIG." }
                    p { "The Drum Engine reads live MIDI in meter-aware windows, proposes drum gestures, selects the lowerable winners, writes ADG beside generated MIDI, and sends the ADG bundle to AIG for the material pass." }
                    div { class: "drum-flow-steps",
                        article { class: "drum-flow-step",
                            span { "Sense" }
                            strong { "Windows and features" }
                            p { "Each output cell reacts to the previous intake cell: density, register, velocity, onset count, and phrase pressure shape the next response." }
                        }
                        article { class: "drum-flow-step",
                            span { "Choose" }
                            strong { "Candidate gestures" }
                            p { "Profile controls and source laws propose kick, snare, hat, ride, tom, and crash gestures; deterministic selection keeps one right-hand surface per tick." }
                        }
                        article { class: "drum-flow-step",
                            span { "Write" }
                            strong { "ADG plus MIDI" }
                            p { "ADG keeps role, kind, timing, strength, surface, phrase role, variation identity, and reason while MIDI carries notes, gates, ticks, and velocity." }
                        }
                        article { class: "drum-flow-step",
                            span { "Import" }
                            strong { "AIG material lane" }
                            p { "AIG narrows drum ADG into its beat-time dialect, runs semantic passes, builds gesture packets and atom specs, then renders reference audio for the material lane." }
                        }
                    }
                }
            }

            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "System shape" }
                    h2 { "What the case connects." }
                    p { "The case follows one working flow: ADG/AIG groove intent becomes MIDI, the session files stay readable, the take is played on a target, and feedback shapes the next pass." }
                }
                div { class: "drum-case-grid",
                    for card in DRUM_ENGINE_EVIDENCE {
                        article { class: "drum-case-card",
                            div { class: "card-topline", "{card.label}" }
                            h3 { "{card.title}" }
                            p { "{card.body}" }
                            div { class: "repo-meta",
                                span { class: "repo-label", "Reference" }
                                code { "{card.detail}" }
                            }
                        }
                    }
                }
            }

            section { class: "utility-band",
                div { class: "section-copy",
                    span { class: "section-kicker", "Since June 4" }
                    h2 { "The drum lane now keeps the take and files together." }
                    p { "The current session flow includes generated-live MIDI, ADG bundle directories, per-chunk traces, runtime snapshots, and an AIG export request that names the bridge consumer." }
                }
                div { class: "utility-links",
                    Link { class: "button button-primary", to: Route::DrumStudio {}, "Open Drum Studio" }
                    Link {
                        class: "button button-secondary",
                        to: Route::NotePost { slug: "pc4ms-drum-engine-since-june-4".to_string() },
                        "Read session note"
                    }
                    Link {
                        class: "source-link",
                        to: Route::NotePost { slug: "adg-aig-bridge-truth-boundary".to_string() },
                        "Bridge note"
                    }
                }
            }

            DrumEngineNotesSection {}

            section { class: "listen-section",
                div { class: "section-copy",
                    span { class: "section-kicker", "Listen" }
                    h2 { "SoundCloud takes from the programmable drum-machine flow." }
                    p { "Start with the release-candidate take. The other Jeans Instability tracks are linked below for nearby versions and live jams." }
                }
                div { class: "soundcloud-frame-shell",
                    iframe {
                        class: "soundcloud-frame",
                        title: "SoundCloud player for jeans instability release candidate 1",
                        src: "{soundcloud_embed_src}",
                        allow: "autoplay; encrypted-media"
                    }
                }
                div { class: "track-list",
                    for track in DRUM_ENGINE_TRACKS {
                        article { class: "track-card",
                            div { class: "card-topline",
                                if track.track_id == DRUM_ENGINE_FEATURED_TRACK_ID {
                                    "Featured"
                                } else {
                                    "SoundCloud"
                                }
                            }
                            h3 { "{track.title}" }
                            p { "{track.note}" }
                            a {
                                class: "source-link",
                                href: "{track.url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "Open track"
                            }
                        }
                    }
                }
                if let Some(featured_track) = featured_track {
                    p { class: "source-hint", "Featured source: {featured_track.url}" }
                }
            }
        }
    }
}

#[component]
fn DrumStudio() -> Element {
    rsx! {
        PageFrame {
            title: "Drum Studio".to_string(),
            description: "Current Drum Studio: ADG bundle files, generated MIDI, runtime traces, and the AIG handoff.".to_string(),
            current: Route::DrumStudio {},
            PageIntroBlock {
                kicker: DRUM_STUDIO_INTRO.kicker,
                title: DRUM_STUDIO_INTRO.title,
                summary: DRUM_STUDIO_INTRO.summary,
            }
            section { class: "utility-band",
                div { class: "section-copy",
                    span { class: "section-kicker", "Session files" }
                    h2 { "Generated MIDI with the files around it." }
                    p { "The current drum work keeps the generated-live MIDI file beside ADG bundle material: manifest, live chunks, per-chunk traces, runtime snapshot, trace, and an AIG export request." }
                }
                div { class: "utility-links",
                    Link { class: "button button-primary", to: Route::DrumEngine {}, "Open drum machine" }
                    Link {
                        class: "button button-secondary",
                        to: Route::NotePost { slug: "drum-studio-runtime-aig-export".to_string() },
                        "Read runtime note"
                    }
                    Link {
                        class: "source-link",
                        to: Route::NotePost { slug: "pc4ms-drum-engine-since-june-4".to_string() },
                        "Session note"
                    }
                }
            }
            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "Roles" }
                    h2 { "What each part does." }
                    p { "The export request keeps the roles clear: the drum engine writes the pass, Drum Studio assembles the session, AIG imports the material, and codec or neural work stays in its own lane." }
                }
                div { class: "products-grid",
                    for card in DRUM_STUDIO_AUTHORITY_CARDS {
                        LabCardView { card: *card }
                    }
                }
            }
            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "Files" }
                    h2 { "Current session files." }
                    p { "This page keeps the public map close to the session: generated MIDI, ADG bundles, trace files, runtime snapshots, and the AIG handoff." }
                }
                div { class: "products-grid",
                    for card in DRUM_STUDIO_ARTIFACT_CARDS {
                        LabCardView { card: *card }
                    }
                }
            }
            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "Notes" }
                    h2 { "The current update set." }
                    p { "These source notes cover the full AIG/ADG flow, runtime export, the ADG-to-AIG bridge, and neural preview research." }
                }
                div { class: "doc-grid",
                    for slug in DRUM_STUDIO_NOTE_SLUGS {
                        if let Some(post) = blog_post_by_slug(slug) {
                            BlogCardView { post }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DrumEngineNotesSection() -> Element {
    rsx! {
        section { class: "doc-category",
            div { class: "section-copy",
                span { class: "section-kicker", "Lab notes" }
                h2 { "Read the build notes." }
                p { "These notes open the programmable drum-machine case into the player surface, ADG/AIG language, MIDI workflow, and feedback loop." }
            }
            div { class: "doc-grid",
                for slug in DRUM_ENGINE_NOTE_SLUGS {
                    if let Some(post) = blog_post_by_slug(slug) {
                        BlogCardView { post }
                    }
                }
            }
        }
    }
}

#[component]
fn PageFrame(title: String, description: String, current: Route, children: Element) -> Element {
    let theme = use_context::<ThemeController>();
    let theme_mode = *theme.mode.read();
    use_effect(move || {
        update_theme_color_meta(*theme.mode.read());
    });

    let full_title = if title == SITE_NAME {
        SITE_NAME.to_string()
    } else {
        format!("{title} | {SITE_NAME}")
    };
    let og_title = full_title.clone();
    let twitter_title = full_title.clone();
    let description_meta = description.clone();
    let og_description = description.clone();
    let twitter_description = description;
    let canonical_url = route_url(&current);
    let preview_image = format!("{}{}", site_base_url(), PREVIEW_IMAGE_PATH);
    let page_shell_class = format!("page-shell {}", theme_mode.shell_class());

    rsx! {
        document::Title { "{full_title}" }
        document::Meta { name: "description", content: "{description_meta}" }
        document::Meta { property: "og:site_name", content: "{SITE_NAME}" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:url", content: "{canonical_url}" }
        document::Meta { property: "og:title", content: "{og_title}" }
        document::Meta { property: "og:description", content: "{og_description}" }
        document::Meta { property: "og:image", content: "{preview_image}" }
        document::Meta { property: "og:image:secure_url", content: "{preview_image}" }
        document::Meta { property: "og:image:type", content: "image/png" }
        document::Meta { property: "og:image:width", content: "{PREVIEW_IMAGE_WIDTH}" }
        document::Meta { property: "og:image:height", content: "{PREVIEW_IMAGE_HEIGHT}" }
        document::Meta { property: "og:image:alt", content: "{PREVIEW_IMAGE_ALT}" }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:image", content: "{preview_image}" }
        document::Meta { name: "twitter:image:alt", content: "{PREVIEW_IMAGE_ALT}" }
        document::Meta { name: "twitter:title", content: "{twitter_title}" }
        document::Meta { name: "twitter:description", content: "{twitter_description}" }

        link { rel: "canonical", href: "{canonical_url}" }

        div { class: "{page_shell_class}",
            a { class: "skip-link", href: "#content", "Skip to content" }
            div { class: "page-grid" }
            SiteHeader { current: current.clone() }
            main { id: "content", class: "main-shell", {children} }
            SiteFooter {}
        }
    }
}

#[component]
fn SiteHeader(current: Route) -> Element {
    let brand_mark_path = public_path("brand-mark.svg");
    let mut theme = use_context::<ThemeController>().mode;
    let theme_mode = *theme.read();
    let next_theme = theme_mode.toggled();
    let theme_label = theme_mode.label();
    let next_theme_label = next_theme.label();
    let theme_button_label = format!("Switch to {next_theme_label} theme");

    rsx! {
        header { class: "site-header",
            nav { class: "site-nav",
                Link { class: "brand", to: Route::Home {},
                    img { class: "brand-mark", src: "{brand_mark_path}", alt: "Mamut EPM mark" }
                    div { class: "brand-copy",
                        span { class: "brand-kicker", "Current build" }
                        span { class: "brand-title", "{SITE_NAME}" }
                    }
                }
                div { class: "nav-cluster",
                    div { class: "nav-links",
                        Link { class: nav_link_class(&current, "home"), to: Route::Home {}, "Home" }
                        Link { class: nav_link_class(&current, "play"), to: Route::Play {}, "Play" }
                        Link { class: nav_link_class(&current, "drums"), to: Route::DrumEngine {}, "Drums" }
                        Link { class: nav_link_class(&current, "notes"), to: Route::Notes {}, "Notes" }
                        Link { class: nav_link_class(&current, "lines"), to: Route::Lines {}, "Lines" }
                        Link { class: nav_link_class(&current, "lab"), to: Route::Lab {}, "Lab" }
                        Link { class: nav_link_class(&current, "docs"), to: Route::Docs {}, "Docs" }
                    }
                    button {
                        class: "theme-toggle",
                        r#type: "button",
                        title: "{theme_button_label}",
                        aria_label: "{theme_button_label}",
                        onclick: move |_| {
                            let next = {
                                let current = *theme.read();
                                current.toggled()
                            };
                            theme.set(next);
                            apply_theme_side_effects(next);
                        },
                        span { class: "theme-toggle-track", aria_hidden: "true",
                            span { class: "theme-toggle-dot" }
                        }
                        span { class: "theme-toggle-label", "{theme_label}" }
                    }
                }
            }
        }
    }
}

fn nav_link_class(current: &Route, key: &str) -> &'static str {
    let active = match (key, current) {
        ("home", Route::Home {}) => true,
        ("play", Route::Play {}) => true,
        ("drums", Route::DrumEngine {} | Route::DrumStudio {}) => true,
        ("notes", Route::Notes {} | Route::NotePost { .. }) => true,
        ("lines", Route::Lines {}) => true,
        ("lab", Route::Lab {}) => true,
        ("docs", Route::Docs {}) => true,
        _ => false,
    };

    if active {
        "nav-link is-active"
    } else {
        "nav-link"
    }
}

#[component]
fn DrumEngineCaseSection() -> Element {
    rsx! {
        section { class: "drum-engine-band",
            div { class: "section-copy",
                span { class: "section-kicker", "Drums" }
                h2 { "Reactive Programmable Drum Machine for MIDI targets." }
                p { "The current drum-machine flow connects ADG/AIG intent, profile taste, generated MIDI, ADG bundles, AIG export requests, target playback, capture, and public SoundCloud takes." }
                div { class: "utility-links",
                    Link { class: "button button-primary", to: Route::DrumEngine {}, "Open drum machine" }
                    Link { class: "button button-secondary", to: Route::DrumStudio {}, "Open Drum Studio" }
                    a {
                        class: "source-link",
                        href: "https://soundcloud.com/mamut_studio",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "Open SoundCloud"
                    }
                }
            }
            div { class: "drum-case-summary",
                div { class: "card-topline", "Reference profile" }
                h3 { "Jeans Instability release candidate" }
                p { "143 BPM, four-bar chunks, groove-led mode, high energy, dense surface, and deliberate humanization. The current lane also keeps generated MIDI beside ADG bundle traces and AIG export metadata." }
                div { class: "drum-mini-controls",
                    for control in DRUM_ENGINE_PRESET_CONTROLS.iter().take(6) {
                        span {
                            strong { "{control.value}" }
                            small { "{control.label}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HeroSection() -> Element {
    rsx! {
        section { class: "hero",
            div { class: "hero-copy",
                span { class: "eyebrow", "{HERO.eyebrow}" }
                h1 { "{HERO.title}" }
                p { class: "hero-body", "{HERO.body}" }
                p { class: "hero-status", "{HERO.status}" }
                div { class: "hero-actions",
                    Link { class: "button button-primary", to: Route::Play {}, "{HERO.primary_cta}" }
                    Link { class: "button button-secondary", to: Route::DrumEngine {}, "{HERO.secondary_cta}" }
                    Link {
                        class: "source-link hero-link",
                        to: Route::DrumStudio {},
                        "Open Drum Studio"
                    }
                }
            }
            div { class: "hero-panel hero-panel-demo",
                div { class: "hero-surface" }
                div { class: "hero-demo-header",
                    div { class: "card-topline", "Reactive programmable drum machine" }
                    h3 { "AIG/ADG to MIDI target" }
                    p { "MIDI live intake gives the groove context. The machine reacts groove-led, keeps the drum decision readable as ADG inside the wider AIG frame, writes bundle traces and AIG export metadata, lowers it to MIDI, sends it to a target, and captures the result." }
                }
                div { class: "hero-preview-ruler",
                    for step in 0..HERO_DRUM_FLOW_STEPS.len() {
                        span {
                            class: if step % 4 == 0 { "ruler-step is-anchor" } else { "ruler-step" },
                            {hero_drum_flow_segment(step)}
                        }
                    }
                }
                div { class: "hero-preview-grid",
                    for (index, step) in HERO_DRUM_FLOW_STEPS.iter().enumerate() {
                        article { class: hero_preview_step_class(index, step.active),
                            span { class: "hero-preview-index", "{step.label}" }
                            strong { "{step.value}" }
                        }
                    }
                }
                div { class: "stat-grid",
                    for stat in HERO_DRUM_CHAIN_CARDS {
                        article { class: "stat-card",
                            span { class: "stat-label", "{stat.label}" }
                            strong { "{stat.value}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HomeUtilitySection() -> Element {
    rsx! {
        section { class: "utility-band",
            div { class: "section-copy",
                span { class: "section-kicker", "Where to start" }
                h2 { "Choose by listening path." }
                p { "Use Play for EPM1, Drums for the MIDI drum-machine take, Drum Studio for the generated session files, and Lab or Docs for the EPM2 hardware path." }
            }
            div { class: "utility-links",
                Link { class: "button button-primary", to: Route::Play {}, "Play EPM1" }
                Link { class: "button button-primary", to: Route::DrumEngine {}, "Hear drums" }
                Link { class: "button button-secondary", to: Route::DrumStudio {}, "Drum Studio" }
                Link {
                    class: "button button-secondary",
                    to: Route::Lab {},
                    "Open lab"
                }
                Link { class: "button button-secondary", to: Route::Docs {}, "Browse docs" }
            }
        }
    }
}

fn home_work_card(area: &crate::content::HomeWorkArea) -> Element {
    rsx! {
        article { class: "feature-card",
            div { class: "card-topline", "{area.kicker}" }
            h3 { "{area.title}" }
            p { "{area.body}" }
            ul {
                for bullet in area.bullets {
                    li { "{bullet}" }
                }
            }
            div { class: "hero-actions",
                {home_primary_action(area)}
                if area.secondary_cta.is_some() {
                    {home_secondary_action(area)}
                }
            }
        }
    }
}

fn home_primary_action(area: &crate::content::HomeWorkArea) -> Element {
    match area.kicker {
        "EPM1" => rsx! {
            Link { class: "button button-primary", to: Route::Play {}, "{area.primary_cta}" }
        },
        "Drums" => rsx! {
            Link { class: "button button-primary", to: Route::DrumEngine {}, "{area.primary_cta}" }
        },
        "MIDI Targets" => rsx! {
            Link {
                class: "button button-primary",
                to: Route::DrumStudio {},
                "{area.primary_cta}"
            }
        },
        "EPM2" => rsx! {
            Link { class: "button button-primary", to: Route::Lab {}, "{area.primary_cta}" }
        },
        _ => rsx! {
            Link { class: "button button-primary", to: Route::Notes {}, "{area.primary_cta}" }
        },
    }
}

fn home_secondary_action(area: &crate::content::HomeWorkArea) -> Element {
    let Some(label) = area.secondary_cta else {
        return rsx! {};
    };

    match area.kicker {
        "EPM1" => rsx! {
            Link { class: "button button-secondary", to: Route::Lines {}, "{label}" }
        },
        "Drums" => rsx! {
            Link {
                class: "button button-secondary",
                to: Route::DrumStudio {},
                "{label}"
            }
        },
        "MIDI Targets" => rsx! {
            Link { class: "button button-secondary", to: Route::DrumEngine {}, "{label}" }
        },
        "EPM2" => rsx! {
            Link { class: "button button-secondary", to: Route::Docs {}, "{label}" }
        },
        _ => rsx! {},
    }
}

#[component]
fn PageIntroBlock(kicker: &'static str, title: &'static str, summary: &'static str) -> Element {
    rsx! {
        section { class: "page-intro",
            span { class: "eyebrow", "{kicker}" }
            h1 { "{title}" }
            p { "{summary}" }
        }
    }
}

#[component]
fn BlogCardView(post: crate::content::BlogPost) -> Element {
    rsx! {
        article { class: "blog-card",
            div { class: "card-topline", "Note / {post.series}" }
            h3 { "{post.title}" }
            p { "{post.intro}" }
            div { class: "blog-card-footer",
                Link {
                    class: "source-link",
                    to: Route::NotePost { slug: post.slug.to_string() },
                    "Read note"
                }
            }
        }
    }
}

#[component]
fn ProductCardView(line: crate::content::ProductLine) -> Element {
    let repo_url = repo_root_url(line.repo);
    let source_link = source_url(line.repo, line.source_path);
    let show_repo_meta = repo_url.is_none();
    let show_source_meta = source_link.is_none();
    let is_epm2 = line.repo == RepoKind::Epm2;

    rsx! {
        article { class: "product-card",
            div { class: "product-header",
                div { class: "product-title-block",
                    div { class: "card-topline", "{line.code}" }
                    h2 { "{line.title}" }
                }
                p { class: "product-status", "{line.status}" }
            }
            p { "{line.summary}" }
            ul {
                for bullet in line.bullets {
                    li { "{bullet}" }
                }
            }
            div { class: "product-actions",
                if is_epm2 {
                    Link {
                        class: "button button-primary",
                        to: Route::Lab {},
                        "Open lab"
                    }
                }
                if let Some(url) = repo_url.as_ref() {
                    a {
                        class: "button button-secondary",
                        href: "{url}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "Open repo"
                    }
                }
                if let Some(url) = source_link.as_ref() {
                    a {
                        class: "source-link",
                        href: "{url}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "Open source"
                    }
                }
            }
            if show_repo_meta {
                div { class: "repo-meta",
                    span { class: "repo-label", "Repo path" }
                    code { "{line.repo_path}" }
                }
            }
            if show_source_meta {
                div { class: "repo-meta",
                    span { class: "repo-label", "Source path" }
                    code { "{line.source_path}" }
                }
            }
        }
    }
}

#[component]
fn LabCardView(card: crate::content::LabCard) -> Element {
    rsx! {
        article { class: "product-card",
            div { class: "product-header",
                div { class: "product-title-block",
                    div { class: "card-topline", "{card.label}" }
                    h3 { "{card.title}" }
                }
            }
            p { "{card.body}" }
            div { class: "repo-meta",
                span { class: "repo-label", "Reference" }
                code { "{card.detail}" }
            }
        }
    }
}

#[component]
fn DocCardView(doc: crate::content::DocCard) -> Element {
    let repo_url = source_url(doc.repo, doc.source_path);

    rsx! {
        article { class: "doc-card",
            div { class: "card-topline", "Source doc" }
            h3 { "{doc.title}" }
            p { "{doc.summary}" }
            if let Some(url) = repo_url {
                a {
                    class: "source-link",
                    href: "{url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Open source"
                }
            } else {
                div { class: "repo-meta",
                    span { class: "repo-label", "Path" }
                    code { "{doc.source_path}" }
                }
            }
        }
    }
}

#[component]
fn SiteFooter() -> Element {
    rsx! {
        footer { class: "site-footer",
            div { class: "footer-inner",
                div {
                    span { class: "section-kicker", "{SITE_NAME}" }
                    p { "Play the browser demo. Use Drums and Drum Studio for MIDI drum sessions, Lab for the EPM2 hardware path, and Notes and Docs for current source material." }
                }
                div { class: "footer-meta",
                    span { "EPM1 active" }
                    span { "MIDI target workflow" }
                    span { "Drum Studio sessions" }
                    span { "Browser demo online" }
                    span { "EPM2 hardware lab" }
                    span { "Notes for current sessions" }
                }
            }
        }
    }
}

fn route_path(route: &Route) -> String {
    match route {
        Route::Home {} => "/".to_string(),
        Route::Notes {} => "/notes".to_string(),
        Route::NotePost { slug } => format!("/notes/{slug}"),
        Route::Lines {} => "/lines".to_string(),
        Route::Lab {} => "/lab".to_string(),
        Route::Docs {} => "/docs".to_string(),
        Route::Play {} => "/play".to_string(),
        Route::DrumEngine {} => "/programmable-drum-machine".to_string(),
        Route::DrumStudio {} => "/drum-studio".to_string(),
    }
}

fn route_url(route: &Route) -> String {
    format!("{}{}", site_base_url(), route_path(route))
}

fn site_base_url() -> &'static str {
    option_env!("MAMUT_SITE_BASE_URL").unwrap_or(DEFAULT_SITE_BASE_URL)
}

fn public_path(path: &str) -> String {
    let base_path = option_env!("MAMUT_PUBLIC_BASE_PATH").unwrap_or("");
    if base_path.is_empty() {
        format!("/{path}")
    } else {
        format!("{}/{path}", base_path.trim_end_matches('/'))
    }
}

fn hero_drum_flow_segment(index: usize) -> &'static str {
    match index {
        0 | 1 => "In",
        2 | 3 => "Sense",
        4..=7 => "ADG",
        8 | 9 => "Bundle",
        10 => "Route",
        _ => "Audio",
    }
}

fn initial_theme_mode() -> ThemeMode {
    stored_theme_mode().unwrap_or(ThemeMode::Dark)
}

#[cfg(target_arch = "wasm32")]
fn stored_theme_mode() -> Option<ThemeMode> {
    let storage = web_sys::window()?.local_storage().ok()??;
    let value = storage.get_item(THEME_STORAGE_KEY).ok()??;

    ThemeMode::from_storage_value(&value)
}

#[cfg(not(target_arch = "wasm32"))]
fn stored_theme_mode() -> Option<ThemeMode> {
    None
}

#[cfg(target_arch = "wasm32")]
fn persist_theme_mode(mode: ThemeMode) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let _ = storage.set_item(THEME_STORAGE_KEY, mode.storage_value());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn persist_theme_mode(_mode: ThemeMode) {}

#[cfg(target_arch = "wasm32")]
fn update_theme_color_meta(mode: ThemeMode) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Ok(Some(meta)) = document.query_selector("meta[name='theme-color']") {
            let _ = meta.set_attribute("content", mode.theme_color());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn update_theme_color_meta(_mode: ThemeMode) {}

fn apply_theme_side_effects(mode: ThemeMode) {
    persist_theme_mode(mode);
    update_theme_color_meta(mode);
}

fn hero_preview_step_class(index: usize, enabled: bool) -> &'static str {
    match (enabled, index % 4 == 0) {
        (true, true) => "hero-preview-step is-active is-anchor",
        (true, false) => "hero-preview-step is-active",
        (false, true) => "hero-preview-step is-anchor",
        (false, false) => "hero-preview-step",
    }
}
