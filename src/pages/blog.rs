use std::{fs, path::PathBuf};

use leptos::prelude::*;
use leptos_router::components::A;
use orgize::Org;
use uuid::Uuid;

#[component]
pub fn Blog() -> impl IntoView {
    let mut writer = Vec::new();
    Org::parse("* title\ntest").write_html(&mut writer).unwrap();
    let result = String::from_utf8(writer).unwrap();

    let paths: Vec<PathBuf> = vec![]; // TODO: retrieve_blog_posts_files();

    view! {
        <div inner_html=result></div>

        <div>{paths.into_iter().map(|path| view! { <p>"file"</p> }).collect::<Vec<_>>()}</div>

        <A href=move || {
            let buf: [u8; 16] = *b"abcdefghijklmnop";
            let uuid = Uuid::new_v8(buf);
            uuid.to_string()
        }>
            <span class="btn">"click this post"</span>
        </A>
    }
}

#[allow(dead_code)]
fn retrieve_blog_posts_files() -> Vec<PathBuf> {
    let paths = fs::read_dir("./assets/posts/").unwrap();
    paths
        .filter_map(|path| {
            let path = path.unwrap().path();
            if path.is_file() && path.ends_with(".org") {
                return Some(path);
            }
            None
        })
        .collect()
}
