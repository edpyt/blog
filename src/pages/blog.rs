use include_dir::File;
use leptos::prelude::*;
use leptos_router::components::A;
use orgize::Org;

use crate::POSTS_DIR;

#[component]
pub fn Blog() -> impl IntoView {
    let posts = POSTS_DIR
        .files()
        .map(|file| {
            (
                file.path()
                    .file_name()
                    .expect("can't retrieve file name")
                    .to_string_lossy(),
                parse_post_file_to_orgmode_html(file),
            )
        })
        .collect::<Vec<_>>();

    view! {
        {posts.into_iter().map(|(post_fname, _post_html)| view! {
        <div class="py-6 flex flex-row gap-6 md:gap-10 items-start">
            <div class="flex flex-col gap-4">
                <h2 class="text-2xl font-bold">
                    <A href=post_fname.clone()>
                        <span class="hover:underline">
                            {post_fname}
                        </span>
                    </A>
                </h2>
                <p class="text-sm text-base-content/70">
                    "Lorum Ipsum Dolar Sit Amet"
                </p>
            </div>
        </div>

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
