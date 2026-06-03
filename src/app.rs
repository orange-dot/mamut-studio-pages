use crate::content::{
    BLOG_INTRO, BLOG_POSTS, DOC_CATEGORIES, DRUM_ENGINE_EVIDENCE, DRUM_ENGINE_FEATURED_TRACK_ID,
    DRUM_ENGINE_NOTE_SLUGS, DRUM_ENGINE_PRESET_CONTROLS, DRUM_ENGINE_TRACKS, EPM2_PUBLIC_REPO_URL,
    HERO, HOME_FEATURES, LAB_INTRO, LAB_NEXT_STEPS, LAB_RESULTS, LAB_STAGES, PC4_BRIDGE,
    PRODUCT_LINES, PRODUCTS_INTRO, RepoKind, STATS, blog_post_by_slug, blog_post_sections,
    pc4_microkit_studio_url, repo_root_url, source_url,
};
use crate::play::PlayPage;
use dioxus::prelude::*;

const SITE_NAME: &str = "Mamut EPM";
const SITE_DESCRIPTION: &str = "Public home for Mamut EPM: play the current software instrument, read working notes, follow the Drum Engine case study, and track the hardware study path.";
const DEFAULT_SITE_BASE_URL: &str = "https://mamut-studio.com";
const PREVIEW_IMAGE_PATH: &str = "/background-clean-final.png";
const HERO_PREVIEW_STEPS: [(&str, bool); 16] = [
    ("C2", true),
    ("G2", false),
    ("C3", true),
    ("G2", false),
    ("G3", true),
    ("D3", true),
    ("C3", false),
    ("C4", true),
    ("C3", true),
    ("G2", false),
    ("G3", true),
    ("C4", false),
    ("D4", true),
    ("G3", false),
    ("G4", true),
    ("C4", true),
];

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
    #[route("/drum-engine")]
    DrumEngine {},
}

#[component]
pub fn App() -> Element {
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
                    span { class: "section-kicker", "Current build" }
                    h2 { "Software now, hardware in study." }
                    p { "EPM1 is the runnable software instrument. EPM2 is the hardware track for simulation, capture, and bench work. They are kept separate so the current state stays clear." }
                }
                div { class: "feature-grid",
                    for section in HOME_FEATURES {
                        {feature_card(section)}
                    }
                }
            }
            HomeUtilitySection {}
            DrumEngineCaseSection {}
            AdjacentProjectSection {}
        }
    }
}

#[component]
fn Notes() -> Element {
    rsx! {
        PageFrame {
            title: "Notes".to_string(),
            description: "Working notes from the software runtime, hardware study path, and related rig work.".to_string(),
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
            current: Route::NotePost { slug: slug.clone() },
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
                                }
                            }
                        }
                        h2 { "Key points" }
                        ul {
                            for bullet in post.bullets {
                                li { "{bullet}" }
                            }
                        }
                        if post.series == "Drum Engine" {
                            div { class: "post-actions",
                                Link { class: "button button-primary", to: Route::DrumEngine {}, "Open Drum Engine case" }
                                Link { class: "button button-secondary", to: Route::Notes {}, "All notes" }
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
            description: "Current split between EPM1, the runnable software instrument, and EPM2, the hardware study track.".to_string(),
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
            description: "EPM2 hardware lab: P1 VCO simulation, KiCad capture, bench expectations, and public source.".to_string(),
            current: Route::Lab {},
            PageIntroBlock {
                kicker: LAB_INTRO.kicker,
                title: LAB_INTRO.title,
                summary: LAB_INTRO.summary,
            }
            section { class: "utility-band",
                div { class: "section-copy",
                    span { class: "section-kicker", "Public source" }
                    h2 { "Hardware evidence, kept separate from site infra." }
                    p { "The EPM2 public repo is a curated hardware-study export: docs, ngspice studies, KiCad capture, bench expectations, and helper tools. Deploy config, investor notes, and local workspace state stay outside that source trail." }
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
                    h2 { "Simulation to capture to bench." }
                    p { "The lab page shows the hardware work as a staged study path with evidence tied to each step." }
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
            description: "Utility library of current EPM2 hardware source documents.".to_string(),
            current: Route::Docs {},
            PageIntroBlock {
                kicker: "Docs",
                title: "Source library for the current hardware corpus.",
                summary: "Docs links to the current source material for EPM2 while keeping the landing page compact."
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
            title: "Authorial Drum Engine".to_string(),
            description: "A portfolio case study for the PC4-playable Drum Engine: semantic ADG/AIG intent, responsive drummer behavior, MIDI output, and public listening evidence.".to_string(),
            current: Route::DrumEngine {},
            section { class: "drum-case-hero",
                div { class: "section-copy",
                    span { class: "eyebrow", "Portfolio case" }
                    h1 { "Authorial Drum Engine" }
                    p { class: "hero-body", "A responsive drummer engine for the PC4 rig: it keeps groove intent in an ADG/AIG layer, locks a performance posture, lowers it into MIDI, routes through mioXM, and is monitored through the Yamaha AG03 path." }
                    p { class: "hero-status", "Drummer companion behavior: it proposes, remembers feedback, and keeps authorship with the person shaping the track." }
                    div { class: "hero-actions",
                        a {
                            class: "button button-primary",
                            href: "https://soundcloud.com/mamut_studio/jeans-instability-release",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "Listen"
                        }
                        Link { class: "button button-secondary", to: Route::Play {}, "Open EPM play" }
                    }
                }
                div { class: "drum-flow-panel",
                    div { class: "card-topline", "Current loop" }
                    h2 { "ADG/AIG to MIDI to PC4" }
                    p { "The operator path starts from a drummer intent layer, resolves it through the active profile, exports MIDI events, routes them through mioXM to the Kurzweil PC4, and monitors the result through the Yamaha AG03 audio path." }
                    div { class: "drum-flow-steps",
                        article { class: "drum-flow-step",
                            span { "Intent" }
                            strong { "Groove shape" }
                            p { "ADG/AIG keeps density, fill pressure, surface, timing feel, and drummer posture visible before note output." }
                        }
                        article { class: "drum-flow-step",
                            span { "Profile" }
                            strong { "Authority layer" }
                            p { "The live preset and manual corpus steer how tightly the drummer locks in, adapts, and shapes fills." }
                        }
                        article { class: "drum-flow-step",
                            span { "mioXM" }
                            strong { "MIDI bridge" }
                            p { "Generated note, velocity, timing, and fill decisions are lowered into MIDI and sent through the rig interface." }
                        }
                        article { class: "drum-flow-step",
                            span { "PC4 + AG03" }
                            strong { "Rig audition" }
                            p { "The Kurzweil PC4 plays the take locally while the Yamaha AG03 path makes the performance chain audible and recordable." }
                        }
                    }
                }
            }

            section { class: "drum-preset-section",
                div { class: "section-copy",
                    span { class: "section-kicker", "Reference live set" }
                    h2 { "Reference live preset." }
                    p { "These controls are the saved startup profile from the release-candidate flow, shown as implementation evidence for the PC4, mioXM, and Yamaha AG03 drummer workflow." }
                }
                div { class: "drum-control-grid",
                    for control in DRUM_ENGINE_PRESET_CONTROLS {
                        article { class: "drum-control-cell",
                            span { class: "stat-label", "{control.label}" }
                            strong { "{control.value}" }
                        }
                    }
                }
            }

            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "System shape" }
                    h2 { "What the case demonstrates." }
                    p { "The useful proof is vertical: semantic groove intent, hardware playback, manual authority, and feedback records all point at one drummer workflow." }
                }
                div { class: "drum-evidence-grid",
                    for card in DRUM_ENGINE_EVIDENCE {
                        article { class: "drum-evidence-card",
                            div { class: "card-topline", "{card.label}" }
                            h3 { "{card.title}" }
                            p { "{card.body}" }
                            div { class: "repo-meta",
                                span { class: "repo-label", "Proof angle" }
                                code { "{card.detail}" }
                            }
                        }
                    }
                }
            }

            DrumEngineNotesSection {}

            section { class: "listen-section",
                div { class: "section-copy",
                    span { class: "section-kicker", "Listen" }
                    h2 { "Public takes from the Drum Engine direction." }
                    p { "The release candidate is embedded as the primary listening artifact. The rest of the series stays available as linked takes so the page remains fast and focused." }
                }
                div { class: "soundcloud-frame-shell",
                    iframe {
                        class: "soundcloud-frame",
                        title: "SoundCloud player for jeans instability release candidate 1",
                        src: "{soundcloud_embed_src}",
                        allow: "autoplay"
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
fn DrumEngineNotesSection() -> Element {
    rsx! {
        section { class: "doc-category",
            div { class: "section-copy",
                span { class: "section-kicker", "Lab notes" }
                h2 { "Read the implementation trail." }
                p { "These notes expand the Drum Engine case into the product frame, ADG/AIG language, hardware rig flow, and feedback authority loop." }
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
    let stylesheet_path = public_path("site.css");
    let brand_mark_path = public_path("brand-mark.svg");

    rsx! {
        document::Title { "{full_title}" }
        document::Meta { name: "description", content: "{description_meta}" }
        document::Meta { name: "theme-color", content: "#090b10" }
        document::Meta { property: "og:site_name", content: "{SITE_NAME}" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:url", content: "{canonical_url}" }
        document::Meta { property: "og:title", content: "{og_title}" }
        document::Meta { property: "og:description", content: "{og_description}" }
        document::Meta { property: "og:image", content: "{preview_image}" }
        document::Meta { property: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:image", content: "{preview_image}" }
        document::Meta { property: "twitter:title", content: "{twitter_title}" }
        document::Meta { property: "twitter:description", content: "{twitter_description}" }

        link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "anonymous" }
        link { rel: "canonical", href: "{canonical_url}" }
        link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500;600;700&family=IBM+Plex+Sans:wght@400;500;600;700&display=swap",
        }
        link { rel: "stylesheet", href: "{stylesheet_path}" }
        link { rel: "icon", href: "{brand_mark_path}", r#type: "image/svg+xml" }

        div { class: "page-shell",
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
                div { class: "nav-links",
                    Link { class: nav_link_class(&current, "home"), to: Route::Home {}, "Home" }
                    Link { class: nav_link_class(&current, "play"), to: Route::Play {}, "Play" }
                    Link { class: nav_link_class(&current, "drums"), to: Route::DrumEngine {}, "Drums" }
                    Link { class: nav_link_class(&current, "notes"), to: Route::Notes {}, "Notes" }
                    Link { class: nav_link_class(&current, "lines"), to: Route::Lines {}, "Lines" }
                    Link { class: nav_link_class(&current, "lab"), to: Route::Lab {}, "Lab" }
                    Link { class: nav_link_class(&current, "docs"), to: Route::Docs {}, "Docs" }
                }
            }
        }
    }
}

fn nav_link_class(current: &Route, key: &str) -> &'static str {
    let active = match (key, current) {
        ("home", Route::Home {}) => true,
        ("play", Route::Play {}) => true,
        ("drums", Route::DrumEngine {}) => true,
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
                span { class: "section-kicker", "Hero case" }
                h2 { "Authorial Drum Engine for the PC4 rig." }
                p { "A separate PC4 Microkit Studio path now has a concrete drummer workflow: ADG/AIG intent, profile-led decisions, MIDI output through mioXM, PC4 playback, Yamaha AG03 monitoring, manual authority, and public listening artifacts." }
                div { class: "utility-links",
                    Link { class: "button button-primary", to: Route::DrumEngine {}, "Open Drum Engine" }
                    a {
                        class: "button button-secondary",
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
                p { "143 BPM, four-bar chunks, groove-led mode, high energy, dense surface, and deliberate humanization. The controls are saved as an operator preset for the current PC4, mioXM, and AG03 drummer workflow." }
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
                    Link { class: "button button-secondary", to: Route::Notes {}, "{HERO.secondary_cta}" }
                    Link { class: "source-link hero-link", to: Route::Lines {}, "See the work split" }
                }
            }
            div { class: "hero-panel hero-panel-proof",
                div { class: "hero-surface" }
                div { class: "hero-proof-header",
                    div { class: "card-topline", "Audible demo" }
                    h3 { "Browser instrument" }
                    p { "Sixteen steps, one macro lane, eight live-set patches, and a browser render path for the current software line." }
                }
                div { class: "hero-preview-ruler",
                    for step in 0..HERO_PREVIEW_STEPS.len() {
                        span {
                            class: if step % 4 == 0 { "ruler-step is-anchor" } else { "ruler-step" },
                            {format!("{:02}", step + 1)}
                        }
                    }
                }
                div { class: "hero-preview-grid",
                    for (index, (note, enabled)) in HERO_PREVIEW_STEPS.iter().enumerate() {
                        article { class: hero_preview_step_class(index, *enabled),
                            span { class: "hero-preview-index", {format!("Step {:02}", index + 1)} }
                            strong { "{note}" }
                        }
                    }
                }
                div { class: "stat-grid",
                    for stat in STATS {
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
                h2 { "Play first, then read the notes." }
                p { "Play is the fastest way to hear the current software runtime. Notes summarize decisions and open work. Lines keeps EPM1 and EPM2 separated. Docs links to the source material." }
            }
            div { class: "utility-links",
                Link { class: "button button-primary", to: Route::Play {}, "Open play" }
                Link { class: "button button-primary", to: Route::Notes {}, "Open notes" }
                Link { class: "button button-secondary", to: Route::Lab {}, "Open lab" }
                Link { class: "button button-secondary", to: Route::Lines {}, "Open lines" }
                Link { class: "button button-secondary", to: Route::Docs {}, "Browse docs" }
            }
        }
    }
}

#[component]
fn AdjacentProjectSection() -> Element {
    let pc4_url = pc4_microkit_studio_url();

    rsx! {
        section { class: "adjacent-band",
            div { class: "section-copy",
                span { class: "section-kicker", "{PC4_BRIDGE.kicker}" }
                h2 { "PC4 rig work lives in a separate repo." }
                p { "PC4 Microkit Studio covers playback control, session artifacts, and local orchestration for the live setup while keeping that work outside the instrument repos." }
            }
            article { class: "adjacent-card",
                div { class: "card-topline", "{PC4_BRIDGE.kicker}" }
                h3 { "{PC4_BRIDGE.title}" }
                p { "{PC4_BRIDGE.summary}" }
                ul {
                    for bullet in PC4_BRIDGE.bullets {
                        li { "{bullet}" }
                    }
                }
                div { class: "adjacent-actions",
                    Link {
                        class: "button button-primary",
                        to: Route::NotePost { slug: "pc4-microkit-studio".to_string() },
                        "Read the note"
                    }
                    if let Some(url) = pc4_url.as_ref() {
                        a {
                            class: "source-link",
                            href: "{url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "Open repo"
                        }
                    }
                }
                if pc4_url.is_none() {
                    div { class: "repo-meta",
                        span { class: "repo-label", "Repo path" }
                        code { "{PC4_BRIDGE.repo_path}" }
                    }
                }
            }
        }
    }
}

fn feature_card(section: &crate::content::DetailSection) -> Element {
    rsx! {
        article { class: "feature-card",
            div { class: "card-topline", "Current focus" }
            h3 { "{section.title}" }
            p { "{section.body}" }
            ul {
                for bullet in section.bullets {
                    li { "{bullet}" }
                }
            }
        }
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
                    p { "Play for the browser demo. Lab for the EPM2 hardware path. Notes and Docs for current decisions and source material." }
                }
                div { class: "footer-meta",
                    span { "EPM1 active" }
                    span { "PC4MS related rig work" }
                    span { "Browser demo online" }
                    span { "EPM2 hardware lab" }
                    span { "Notes for current decisions" }
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
        Route::DrumEngine {} => "/drum-engine".to_string(),
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

fn hero_preview_step_class(index: usize, enabled: bool) -> &'static str {
    match (enabled, index % 4 == 0) {
        (true, true) => "hero-preview-step is-active is-anchor",
        (true, false) => "hero-preview-step is-active",
        (false, true) => "hero-preview-step is-anchor",
        (false, false) => "hero-preview-step",
    }
}
