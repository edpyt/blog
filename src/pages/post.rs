use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use crate::components::GiscusComments;
use crate::core::constants::DATETIME_FORMAT;
use crate::core::OrgPost;
use crate::POSTS_DIR;

#[derive(Params, PartialEq, Clone, Debug)]
struct PostParams {
    post_filename: Option<String>,
}

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params::<PostParams>();
    let post_filename = params
        .get_untracked()
        .map(|params| params.post_filename.unwrap_or_default())
        .expect("Can't retrieve post filename param");

    match POSTS_DIR.get_file(&post_filename) {
        None => Either::Left(view! { <p>"No post with provided filename was found."</p> }),
        Some(post_file) => {
            let result_view = match OrgPost::try_from(post_file) {
                Ok(org_post) => view! {
                    <div class="flex flex-col gap-5 ">
                        <div inner_html=org_post.content_html()></div>
                        <div>
                            <div class="flex justify-end">
                                <p class="text-sm text-base-content/70">
                                    {org_post.created.format(DATETIME_FORMAT).to_string()}
                                </p>
                            </div>

                            <GiscusComments />
                        </div>
                    </div>
                }
                .into_any(),
                Err(_) => view! {
                    "Can't parse file "
                    {post_filename}
                }
                .into_any(),
            };

            Either::Right(result_view)
        }
    }
}
