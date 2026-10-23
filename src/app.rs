use crate::archive::{BlogPost, NOTES_INDEX_SLUGS, blog_post_by_slug, blog_post_sections};
use crate::content::{
    ASSEMBLY_POLICY, CLAIM_LEVELS, HERO_BODY, HERO_TITLE, INSTRUMENTS, INSTRUMENTS_INTRO, Item,
    KEYBOARD_INTRO, KEYBOARD_STAGES, Module, NOTES, PHYSICAL_NOTE, SOURCES, SOURCES_INTRO, Stage,
    TONEWHEEL91_REPO_URL, VOICE_INTRO, VOICE_MODULES, VOICE_SIGNAL_PATH, module_by_slug,
};
use dioxus::prelude::*;

const SITE_NAME: &str = "Mamut Studio";
const SITE_DESCRIPTION: &str = "Mamut: a keyboard instrument built from discrete analog circuits, a sensing key surface and C instrument models. Designs, model runs and their limits.";
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
    #[route("/voice")]
    Voice {},
    #[route("/voice/:slug")]
    VoiceModule { slug: String },
    #[route("/keyboard")]
    Keyboard {},
    #[route("/sources")]
    Sources {},
    #[redirect("/play", || Route::Instruments {})]
    #[redirect("/listen", || Route::Instruments {})]
    #[redirect("/programmable-drum-machine", || Route::Instruments {})]
    #[redirect("/drum-engine", || Route::Instruments {})]
    #[redirect("/drum-studio", || Route::Instruments {})]
    #[redirect("/drums", || Route::Instruments {})]
    #[route("/instruments")]
    Instruments {},
    #[redirect("/blog", || Route::Notes {})]
    #[route("/notes")]
    Notes {},
    #[redirect("/blog/:slug", |slug: String| Route::NotePost { slug })]
    #[route("/notes/:slug")]
    NotePost { slug: String },
    #[redirect("/how-it-works", || Route::About {})]
    #[route("/about")]
    About {},
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
            section { class: "hero",
                div { class: "hero-copy",
                    span { class: "eyebrow", "Mamut" }
                    h1 { "{HERO_TITLE}" }
                    p { class: "hero-body", "{HERO_BODY}" }
                    div { class: "hero-actions",
                        Link { class: "button button-primary", to: Route::Voice {}, "The voice boards" }
                        Link { class: "button button-secondary", to: Route::Instruments {}, "Instruments that play" }
                    }
                }
                div { class: "hero-panel",
                    div { class: "card-topline", "State on 2026-10-23" }
                    ul { class: "hero-today",
                        li { "Oscillator and filter boards: CAD released, model runs pass." }
                        li { "Amplifier, control, drive and supply: planned or in progress." }
                        li { "Sensing key surface: receiver boards routed, sensor stack not validated." }
                        li { "Organ, electric piano and analog voice model play live in C." }
                    }
                    div { class: "hero-panel-divider" }
                    p { class: "hero-panel-note", "{PHYSICAL_NOTE} for any hardware item." }
                }
            }
            section { class: "signal-section",
                div { class: "section-copy",
                    span { class: "section-kicker", "Four lines of work" }
                    h2 { "Hardware, touch, sources, and instruments you can play now." }
                }
                div { class: "feature-grid",
                    {area_card("Voice-1", "Discrete analog voice", VOICE_INTRO, Route::Voice {}, "Open the voice")}
                    {area_card("Keyboard", "Sensing key surface", KEYBOARD_INTRO, Route::Keyboard {}, "Open the keyboard")}
                    {area_card("Sources", "Physical sound sources", SOURCES_INTRO, Route::Sources {}, "Open the sources")}
                    {area_card("tonewheel91", "C instrument models", INSTRUMENTS_INTRO, Route::Instruments {}, "Open the instruments")}
                }
            }
        }
    }
}

fn area_card(
    kicker: &'static str,
    title: &'static str,
    body: &'static str,
    to: Route,
    cta: &'static str,
) -> Element {
    rsx! {
        article { class: "feature-card",
            div { class: "card-topline", "{kicker}" }
            h3 { "{title}" }
            p { "{body}" }
            div { class: "hero-actions",
                Link { class: "source-link", to: to, "{cta}" }
            }
        }
    }
}

#[component]
fn Voice() -> Element {
    rsx! {
        PageFrame {
            title: "Voice-1".to_string(),
            description: VOICE_INTRO.to_string(),
            current: Route::Voice {},
            section { class: "page-intro",
                span { class: "eyebrow", "Voice-1" }
                h1 { "A discrete analog voice, board by board." }
                p { "{VOICE_INTRO}" }
            }
            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "Signal path" }
                    code { class: "signal-path", "{VOICE_SIGNAL_PATH}" }
                    p { "{ASSEMBLY_POLICY}" }
                }
                div { class: "products-grid",
                    for module in VOICE_MODULES {
                        {module_card(module)}
                    }
                }
            }
        }
    }
}

fn module_card(module: &'static Module) -> Element {
    rsx! {
        article { class: "detail-panel",
            div { class: "card-topline",
                span { "{module.role}" }
                StageBadge { stage: module.stage }
            }
            h3 { "{module.code}" }
            p { "{module.summary}" }
            Link {
                class: "source-link",
                to: Route::VoiceModule { slug: module.slug.to_string() },
                "Details"
            }
        }
    }
}

#[component]
fn VoiceModule(slug: String) -> Element {
    let Some(module) = module_by_slug(&slug) else {
        return rsx! {
            PageFrame {
                title: "Module".to_string(),
                description: "Requested module was not found.".to_string(),
                current: Route::Voice {},
                section { class: "page-intro",
                    span { class: "eyebrow", "Voice-1" }
                    h1 { "Requested module was not found." }
                    Link { class: "button button-secondary", to: Route::Voice {}, "All modules" }
                }
            }
        };
    };

    rsx! {
        PageFrame {
            title: module.code.to_string(),
            description: module.summary.to_string(),
            current: Route::VoiceModule { slug: module.slug.to_string() },
            section { class: "page-intro",
                Link { class: "source-link post-back", to: Route::Voice {}, "Voice-1" }
                span { class: "eyebrow", "{module.role}" }
                h1 { "{module.code}" }
                p { "{module.summary}" }
                StageBadge { stage: module.stage }
            }
            section { class: "products-grid",
                article { class: "detail-panel",
                    div { class: "card-topline", "Recorded" }
                    dl { class: "fact-list",
                        for fact in module.facts {
                            dt { "{fact.label}" }
                            dd { "{fact.value}" }
                        }
                    }
                }
                article { class: "detail-panel",
                    div { class: "card-topline", "Open" }
                    ul {
                        for item in module.open {
                            li { "{item}" }
                        }
                        li { "{PHYSICAL_NOTE}." }
                    }
                }
            }
        }
    }
}

#[component]
fn Keyboard() -> Element {
    rsx! {
        PageFrame {
            title: "Sensing keyboard".to_string(),
            description: KEYBOARD_INTRO.to_string(),
            current: Route::Keyboard {},
            section { class: "page-intro",
                span { class: "eyebrow", "Keyboard" }
                h1 { "Touch as control and as sound." }
                p { "{KEYBOARD_INTRO}" }
            }
            ItemGrid { items: KEYBOARD_STAGES }
            p { class: "physical-note", "{PHYSICAL_NOTE} for any sensor or receiver board." }
        }
    }
}

#[component]
fn Sources() -> Element {
    rsx! {
        PageFrame {
            title: "Sound sources".to_string(),
            description: SOURCES_INTRO.to_string(),
            current: Route::Sources {},
            section { class: "page-intro",
                span { class: "eyebrow", "Sources" }
                h1 { "Physical mechanisms as sound sources." }
                p { "{SOURCES_INTRO}" }
            }
            ItemGrid { items: SOURCES }
            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "How claims are marked" }
                    h2 { "Five levels, from a cited effect to a measured sample." }
                }
                dl { class: "fact-list",
                    for level in CLAIM_LEVELS {
                        dt { "{level.label}" }
                        dd { "{level.value}" }
                    }
                }
            }
        }
    }
}

#[component]
fn Instruments() -> Element {
    rsx! {
        PageFrame {
            title: "Instruments".to_string(),
            description: INSTRUMENTS_INTRO.to_string(),
            current: Route::Instruments {},
            section { class: "page-intro",
                span { class: "eyebrow", "tonewheel91" }
                h1 { "Instruments that play today." }
                p { "{INSTRUMENTS_INTRO}" }
                div { class: "hero-actions",
                    a {
                        class: "button button-primary",
                        href: TONEWHEEL91_REPO_URL,
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "Source on GitHub"
                    }
                }
            }
            ItemGrid { items: INSTRUMENTS }
        }
    }
}

#[component]
fn ItemGrid(items: &'static [Item]) -> Element {
    rsx! {
        section { class: "products-grid",
            for item in items {
                article { class: "detail-panel",
                    div { class: "card-topline",
                        span { "{item.code}" }
                        StageBadge { stage: item.stage }
                    }
                    h3 { "{item.title}" }
                    p { "{item.summary}" }
                    ul {
                        for point in item.points {
                            li { "{point}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StageBadge(stage: Stage) -> Element {
    rsx! {
        span { class: stage.class(), "{stage.label()}" }
    }
}

#[component]
fn Notes() -> Element {
    rsx! {
        PageFrame {
            title: "Notes".to_string(),
            description: "Working notes on the Mamut hardware and instrument models, and the archive of the earlier software line.".to_string(),
            current: Route::Notes {},
            section { class: "page-intro",
                span { class: "eyebrow", "Notes" }
                h1 { "Working notes." }
                p { "Short write-ups when a board, a model run or an instrument reaches a milestone." }
            }
            if !NOTES.is_empty() {
                section { class: "blog-grid",
                    for note in NOTES {
                        article { class: "blog-card",
                            div { class: "card-topline", "{note.date}" }
                            h3 { "{note.title}" }
                            p { "{note.summary}" }
                        }
                    }
                }
            }
            section { class: "doc-category",
                div { class: "section-copy",
                    span { class: "section-kicker", "Archive" }
                    h2 { "The earlier software line." }
                    p { "Notes from the EPM1 software instrument and the drum engine. That line is retired; the notes stay for reference." }
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
                    p { class: "archive-banner", "Archived note from the retired EPM1 / drum-engine line." }
                    section { class: "page-intro post-intro",
                        span { class: "eyebrow", "Note / {post.series}" }
                        h1 { "{post.title}" }
                        p { "{post.intro}" }
                    }
                    article { class: "detail-panel post-panel",
                        div { class: "card-topline", "{post.series}" }
                        p { "{post.body}" }
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
                        h2 { "Key points" }
                        ul {
                            for bullet in post.bullets {
                                li { "{bullet}" }
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
fn About() -> Element {
    rsx! {
        PageFrame {
            title: "About".to_string(),
            description: "Who builds Mamut, what the project is after, and how its claims are marked.".to_string(),
            current: Route::About {},
            section { class: "page-intro",
                span { class: "eyebrow", "About" }
                h1 { "One instrument, built in the open where it can be." }
                p { "Mamut Studio is an independent instrument lab. The aim is a keyboard instrument made to order, with analog electronics, digital processing and a sound source of its own." }
            }
            section { class: "products-grid",
                article { class: "detail-panel",
                    div { class: "card-topline", "What is public" }
                    p { "The tonewheel91 instrument models are open source. The hardware research is not; this site publishes its results, figures and limits." }
                    a {
                        class: "source-link",
                        href: TONEWHEEL91_REPO_URL,
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "tonewheel91 on GitHub"
                    }
                }
                article { class: "detail-panel",
                    div { class: "card-topline", "How to read the figures" }
                    p { "A passing model run means the circuit meets its stated criteria inside the model's limits. It is not a measurement. Every hardware page says what has and has not been tested." }
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
                        span { class: "brand-kicker", "Instrument lab" }
                        span { class: "brand-title", "{SITE_NAME}" }
                    }
                }
                div { class: "nav-cluster",
                    div { class: "nav-links",
                        Link { class: nav_link_class(&current, "voice"), to: Route::Voice {}, "Voice" }
                        Link { class: nav_link_class(&current, "keyboard"), to: Route::Keyboard {}, "Keyboard" }
                        Link { class: nav_link_class(&current, "sources"), to: Route::Sources {}, "Sources" }
                        Link { class: nav_link_class(&current, "instruments"), to: Route::Instruments {}, "Instruments" }
                        Link { class: nav_link_class(&current, "notes"), to: Route::Notes {}, "Notes" }
                        Link { class: nav_link_class(&current, "about"), to: Route::About {}, "About" }
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
        ("voice", Route::Voice {} | Route::VoiceModule { .. }) => true,
        ("keyboard", Route::Keyboard {}) => true,
        ("sources", Route::Sources {}) => true,
        ("instruments", Route::Instruments {}) => true,
        ("notes", Route::Notes {} | Route::NotePost { .. }) => true,
        ("about", Route::About {}) => true,
        _ => false,
    };

    if active {
        "nav-link is-active"
    } else {
        "nav-link"
    }
}

#[component]
fn BlogCardView(post: BlogPost) -> Element {
    rsx! {
        article { class: "blog-card",
            div { class: "card-topline", "Archive / {post.series}" }
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
                    p { "Discrete analog voice boards, a sensing key surface, physical sound sources and C instrument models." }
                }
                div { class: "footer-meta",
                    span { "Hardware: designed and simulated" }
                    span { "{PHYSICAL_NOTE}" }
                    a {
                        href: TONEWHEEL91_REPO_URL,
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "tonewheel91 source"
                    }
                }
            }
        }
    }
}

fn route_path(route: &Route) -> String {
    match route {
        Route::Home {} => "/".to_string(),
        Route::Voice {} => "/voice".to_string(),
        Route::VoiceModule { slug } => format!("/voice/{slug}"),
        Route::Keyboard {} => "/keyboard".to_string(),
        Route::Sources {} => "/sources".to_string(),
        Route::Instruments {} => "/instruments".to_string(),
        Route::Notes {} => "/notes".to_string(),
        Route::NotePost { slug } => format!("/notes/{slug}"),
        Route::About {} => "/about".to_string(),
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
