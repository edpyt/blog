use blog::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        view! { <App /> }
    })
}
