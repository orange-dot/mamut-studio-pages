#![allow(non_snake_case)]

mod app;
mod archive;
mod content;
mod research_assets;

use dioxus::prelude::*;
use dioxus_logger::tracing;

fn main() {
    let _ = dioxus_logger::init(tracing::Level::INFO);
    launch(app::App);
}
