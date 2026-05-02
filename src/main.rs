#![allow(non_snake_case)]

mod app;
mod content;
mod play;

use dioxus::prelude::*;
use dioxus_logger::tracing;

fn main() {
    let _ = dioxus_logger::init(tracing::Level::INFO);
    launch(app::App);
}
