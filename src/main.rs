mod app;
mod components;
mod markdown;
mod posts;

use leptos::prelude::*;

fn main() {
    mount_to_body(app::App);
}
