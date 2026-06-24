use crate::content::{
    BLOG_INTRO, DRUM_ENGINE_FEATURED_TRACK_ID, DRUM_ENGINE_PRESET_CONTROLS, DRUM_ENGINE_TRACKS,
    EPM2_PUBLIC_REPO_URL, HERO, HOME_WORK_AREAS, NOTES_INDEX_SLUGS, WHAT_RUNS_TODAY,
    WHERE_THIS_IS_GOING, blog_post_by_slug, blog_post_sections,
};
use crate::play::PlayPage;
use dioxus::prelude::*;

const SITE_NAME: &str = "Mamut Studio";
const SITE_DESCRIPTION: &str = "Mamut Studio is a musical system that learns your taste, on your own machine. Drums play today; play EPM1 in the browser or hear the current takes.";
const DEFAULT_SITE_BASE_URL: &str = "https://mamut-studio.com";
const PREVIEW_IMAGE_PATH: &str = "/og-default.png";
const PREVIEW_IMAGE_WIDTH: &str = "3000";
const PREVIEW_IMAGE_HEIGHT: &str = "3000";
const PREVIEW_IMAGE_ALT: &str = "Mamut Studio circular signal artwork";
#[cfg(target_arch = "wasm32")]
const THEME_STORAGE_KEY: &str = "mamut-theme";
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[redirect("/control-plane", || Route::Home {})]
    #[redirect("/lines", || Route::Home {})]
    #[redirect("/products", || Route::Home {})]
    #[redirect("/lab", || Route::Home {})]
    #[redirect("/docs", || Route::Home {})]
    #[route("/")]
    Home {},
    #[redirect("/blog", || Route::Notes {})]
    #[route("/notes")]
    Notes {},
    #[redirect("/blog/:slug", |slug: String| Route::NotePost { slug })]
    #[route("/notes/:slug")]
    NotePost { slug: String },
    #[route("/play")]
    Play {},
    #[redirect("/programmable-drum-machine", || Route::Listen {})]
    #[redirect("/drum-engine", || Route::Listen {})]
    #[redirect("/drum-studio", || Route::Listen {})]
    #[redirect("/drums", || Route::Listen {})]
    #[route("/listen")]
    Listen {},
    #[route("/how-it-works")]
    HowItWorks {},
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
                    span { class: "section-kicker", "Where to start" }
                    h2 { "Play it, hear it, see where it goes." }
                    p { "EPM1 plays in the browser today. The drum engine already turns a groove decision into MIDI the rig plays, and the same taste loop is built to carry more instruments next." }
                }
                div { class: "feature-grid",
                    for area in HOME_WORK_AREAS {
                        {home_work_card(area)}
                    }
                }
            }
            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "Honest status" }
                    h2 { "What runs today, and where it goes." }
                    p { "Drums are the first voice. Everything here is either playing now or named as the next step." }
                }
                div { class: "products-grid",
                    article { class: "detail-panel",
                        div { class: "card-topline", "What runs today" }
                        ul {
                            for item in WHAT_RUNS_TODAY {
                                li { "{item}" }
                            }
                        }
                    }
                    article { class: "detail-panel",
                        div { class: "card-topline", "Where this is going" }
                        ul {
                            for item in WHERE_THIS_IS_GOING {
                                li { "{item}" }
                            }
                        }
                    }
                }
            }
            BackgroundSection {}
        }
    }
}

#[component]
fn BackgroundSection() -> Element {
    rsx! {
        section { class: "utility-band",
            div { class: "section-copy",
                span { class: "section-kicker", "Background" }
                h2 { "Where this came from." }
                p { "Mamut Studio grew out of a long line of instrument and rig work: the EPM1 software runtime, the PC4 live setup, and an EPM2 hardware study. The taste system is what that work converged on." }
            }
            div { class: "utility-links",
                Link { class: "button button-primary", to: Route::HowItWorks {}, "How it works" }
                a {
                    class: "button button-secondary",
                    href: EPM2_PUBLIC_REPO_URL,
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "EPM2 hardware source"
                }
            }
        }
    }
}

#[component]
fn Notes() -> Element {
    rsx! {
        PageFrame {
            title: "Notes".to_string(),
            description: "A few working notes on the drum engine, the taste loop, and the language that keeps a groove decision musical before MIDI.".to_string(),
            current: Route::Notes {},
            PageIntroBlock {
                kicker: BLOG_INTRO.kicker,
                title: BLOG_INTRO.title,
                summary: BLOG_INTRO.summary,
            }
            section { class: "blog-grid",
                for slug in NOTES_INDEX_SLUGS {
                    if let Some(post) = blog_post_by_slug(slug) {
                        BlogCardView { post }
                    }
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
                                Link { class: "button button-primary", to: Route::Listen {}, "Hear the drums" }
                                Link { class: "button button-secondary", to: Route::HowItWorks {}, "How it works" }
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
fn HowItWorks() -> Element {
    rsx! {
        PageFrame {
            title: "How it works".to_string(),
            description: "How Mamut Studio learns your taste: it keeps the takes you choose, leans that way next time, and stays local and undoable.".to_string(),
            current: Route::HowItWorks {},
            section { class: "page-intro",
                span { class: "eyebrow", "How it works" }
                h1 { "It learns your taste from the takes you keep." }
                p { "You play, you listen, you keep some takes and pass over others. Mamut Studio remembers those choices and leans that way on the next pass, on your own machine, with every step one you can undo." }
            }
            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "The loop" }
                    h2 { "Choose, learn, lean." }
                    p { "Nothing is retrained in the cloud and nothing leaves your machine. The system updates from a single choice, and you can roll it back." }
                }
                div { class: "products-grid",
                    article { class: "detail-panel",
                        div { class: "card-topline", "Choose" }
                        h3 { "You keep what sounds like you." }
                        p { "Pick the take that fits, pass over the one that does not. That choice is the only input the loop needs." }
                    }
                    article { class: "detail-panel",
                        div { class: "card-topline", "Learn" }
                        h3 { "It updates on the spot." }
                        p { "The taste loop shifts toward what you kept, locally and right away, without a training run or a model upload." }
                    }
                    article { class: "detail-panel",
                        div { class: "card-topline", "Lean" }
                        h3 { "The next pass leans your way." }
                        p { "The following set is re-ranked toward your taste. If it drifts, you undo the step and it returns." }
                    }
                }
            }
            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "The learning" }
                    h2 { "It learns in memory, as you go." }
                    p { "The taste lives in memory, in a small map that a single choice nudges on the spot. The next note builds on the last, and the earlier taste carries forward as new taste settles in. That is what continual learning means here: each choice adds, and the past stays. This is early, local work, but the shape is deliberate." }
                }
                div { class: "products-grid",
                    article { class: "detail-panel",
                        div { class: "card-topline", "In memory" }
                        h3 { "No training run." }
                        p { "The system updates a small in-memory map from one choice at a time. There is no training pass and nothing uploaded; the change lands where you are, in milliseconds, while you keep playing." }
                    }
                    article { class: "detail-panel",
                        div { class: "card-topline", "Continual" }
                        h3 { "It keeps what it learned." }
                        p { "Each choice adds to the taste and leaves the last in place. New preferences settle in beside the old ones, so the system grows with you instead of forgetting the earlier feel every time it learns." }
                    }
                    article { class: "detail-panel",
                        div { class: "card-topline", "Bounded and local" }
                        h3 { "Small enough to stay with you." }
                        p { "The map stays bounded and runs on a regular laptop, with the same shape built to fit small hardware later. Nothing leaves the room, and every step is written down, so you can roll any of it back." }
                    }
                }
                p { class: "learning-note",
                    "The map is laid out the way a CPU likes it. The hot core — the few cells a choice touches — stays small and packed, small enough to sit in L1 and L2 cache, while the colder bulk of the map rests in L3 and main memory. A single choice moves only a handful of cache lines, so the update lands in the gap between two notes, with nothing waiting on disk or a network."
                }
            }
            section { class: "utility-band",
                div { class: "section-copy",
                    span { class: "section-kicker", "First voice, then more" }
                    h2 { "Drums today; the same loop for other instruments." }
                    p { "Drums are where the loop runs now. The same approach is built to carry piano and other instruments next, so the taste you teach in one place can carry across the rest." }
                }
                div { class: "utility-links",
                    Link { class: "button button-primary", to: Route::Listen {}, "Hear the drums" }
                    Link { class: "button button-secondary", to: Route::Play {}, "Play EPM1" }
                }
            }
            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "The language underneath" }
                    h2 { "ADG/AIG keeps a groove decision musical before MIDI." }
                    p { "Under the hood, a small drum language (ADG, inside the wider AIG frame) holds the groove decision, gesture, surface, density, and timing feel before any of it becomes MIDI. It is what lets the system learn taste in musical terms instead of raw notes. The notes go deeper for anyone who wants the detail." }
                }
                div { class: "utility-links",
                    Link { class: "button button-secondary", to: Route::Notes {}, "Read the notes" }
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
            description: "The EPM1 software instrument, playing in the browser: eight patches, macro controls, one render path.".to_string(),
            current: Route::Play {},
            PlayPage {}
        }
    }
}

#[component]
fn Listen() -> Element {
    let soundcloud_embed_src = format!(
        "https://w.soundcloud.com/player/?visual=true&url=https%3A%2F%2Fapi.soundcloud.com%2Ftracks%2F{}&show_artwork=true",
        DRUM_ENGINE_FEATURED_TRACK_ID
    );
    let featured_track = DRUM_ENGINE_TRACKS
        .iter()
        .find(|track| track.track_id == DRUM_ENGINE_FEATURED_TRACK_ID);

    rsx! {
        PageFrame {
            title: "Listen".to_string(),
            description: "Hear the drums Mamut Studio plays today: the saved 143 BPM live set on SoundCloud, release-candidate take first.".to_string(),
            current: Route::Listen {},
            section { class: "page-intro",
                span { class: "eyebrow", "Listen" }
                h1 { "The drums it plays today." }
                p { "The drum engine turns a groove decision into MIDI, the PC4 plays it, and the takes land here. Start with the release-candidate take from the saved 143 BPM live set." }
            }
            section { class: "listen-section",
                div { class: "soundcloud-frame-shell",
                    iframe {
                        class: "soundcloud-frame",
                        title: "SoundCloud player for the jeans instability release-candidate take",
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
            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "Reference take" }
                    h2 { "The live set behind the featured take." }
                    p { "The release-candidate take comes from a saved 143 BPM live set: four-bar chunks, groove-led, played on the rig and captured." }
                }
                div { class: "drum-mini-controls",
                    for control in DRUM_ENGINE_PRESET_CONTROLS.iter().take(6) {
                        span {
                            strong { "{control.value}" }
                            small { "{control.label}" }
                        }
                    }
                }
            }
            section { class: "utility-band",
                div { class: "section-copy",
                    span { class: "section-kicker", "How it gets here" }
                    h2 { "From a groove decision to a take you can keep." }
                    p { "How it works walks the loop: you keep the takes that sound like you, and the next pass leans that way." }
                }
                div { class: "utility-links",
                    Link { class: "button button-primary", to: Route::HowItWorks {}, "How it works" }
                    Link { class: "button button-secondary", to: Route::Notes {}, "Read the notes" }
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
                    img { class: "brand-mark", src: "{brand_mark_path}", alt: "Mamut Studio mark" }
                    div { class: "brand-copy",
                        span { class: "brand-kicker", "Learns your taste" }
                        span { class: "brand-title", "{SITE_NAME}" }
                    }
                }
                div { class: "nav-cluster",
                    div { class: "nav-links",
                        Link { class: nav_link_class(&current, "home"), to: Route::Home {}, "Home" }
                        Link { class: nav_link_class(&current, "play"), to: Route::Play {}, "Play" }
                        Link { class: nav_link_class(&current, "listen"), to: Route::Listen {}, "Listen" }
                        Link { class: nav_link_class(&current, "how"), to: Route::HowItWorks {}, "How it works" }
                        Link { class: nav_link_class(&current, "notes"), to: Route::Notes {}, "Notes" }
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
        ("listen", Route::Listen {}) => true,
        ("how", Route::HowItWorks {}) => true,
        ("notes", Route::Notes {} | Route::NotePost { .. }) => true,
        _ => false,
    };

    if active {
        "nav-link is-active"
    } else {
        "nav-link"
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
                    Link { class: "button button-secondary", to: Route::Listen {}, "{HERO.secondary_cta}" }
                    Link {
                        class: "source-link hero-link",
                        to: Route::HowItWorks {},
                        "How it works"
                    }
                }
            }
            div { class: "hero-panel",
                div { class: "card-topline", "What runs today" }
                ul { class: "hero-today",
                    for item in WHAT_RUNS_TODAY {
                        li { "{item}" }
                    }
                }
                div { class: "hero-panel-divider" }
                div { class: "card-topline", "The shape" }
                ul { class: "hero-today",
                    li { "Local — it runs on your machine, nothing uploaded." }
                    li { "Reversible — every step is one you can undo." }
                    li { "Continual — it keeps learning as you go, without retraining." }
                }
                div { class: "hero-panel-divider" }
                p { class: "hero-panel-note",
                    "Drums are the first voice. Piano and other instruments are the next, on the same loop."
                }
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
        "Play" => rsx! {
            Link { class: "button button-primary", to: Route::Play {}, "{area.primary_cta}" }
        },
        "Drums" => rsx! {
            Link { class: "button button-primary", to: Route::Listen {}, "{area.primary_cta}" }
        },
        _ => rsx! {
            Link { class: "button button-primary", to: Route::HowItWorks {}, "{area.primary_cta}" }
        },
    }
}

fn home_secondary_action(area: &crate::content::HomeWorkArea) -> Element {
    let Some(label) = area.secondary_cta else {
        return rsx! {};
    };

    rsx! {
        Link { class: "button button-secondary", to: Route::HowItWorks {}, "{label}" }
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
fn SiteFooter() -> Element {
    rsx! {
        footer { class: "site-footer",
            div { class: "footer-inner",
                div {
                    span { class: "section-kicker", "{SITE_NAME}" }
                    p { "A musical system that learns your taste, on your own machine. Play EPM1, hear the drums, or read how the taste loop works." }
                }
                div { class: "footer-meta",
                    span { "EPM1 plays in the browser" }
                    span { "Drum takes on SoundCloud" }
                    span { "Local and undoable" }
                    span { "Drums first, more next" }
                    a {
                        href: EPM2_PUBLIC_REPO_URL,
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "EPM2 hardware source"
                    }
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
        Route::Play {} => "/play".to_string(),
        Route::Listen {} => "/listen".to_string(),
        Route::HowItWorks {} => "/how-it-works".to_string(),
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

fn initial_theme_mode() -> ThemeMode {
    stored_theme_mode().unwrap_or(ThemeMode::Light)
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
