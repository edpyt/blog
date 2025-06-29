use include_dir::File;
use leptos::prelude::*;
use orgize::Org;

use crate::POSTS_DIR;

#[component]
pub fn Blog() -> impl IntoView {
    let posts = POSTS_DIR
        .files()
        .map(|file| {
            (
                file.path().file_name().expect("can't retrieve file name"),
                parse_post_file_to_orgmode_html(file),
            )
        })
        .collect::<Vec<_>>();

    view! {
        {posts.into_iter().map(|(_post_fname, post_html)| view! {

            <div inner_html=post_html></div>

        }).collect::<Vec<_>>()}
    }
}

fn parse_post_file_to_orgmode_html(file: &File) -> String {
    let mut writer = vec![];

    Org::parse(file.contents_utf8().unwrap())
        .write_html(&mut writer)
        .unwrap_or_else(|_| panic!("Can't parse {file:?} as orgmode file"));

    String::from_utf8(writer).unwrap()
}
