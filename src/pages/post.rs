use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use orgize::Org;

use crate::pages::comments::GiscusComments;

#[derive(Params, PartialEq, Clone, Debug)]
struct PostParams {
    post_title: Option<String>,
}

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params::<PostParams>();
    let post_title = params
        .get_untracked()
        .map(|params| params.post_title.unwrap_or_default())
        .ok();

    match post_title {
        None => Either::Left(view! { <p>"No post with provided title was found."</p> }),
        Some(post_title) => {
            let result = Org::parse("* title\ntest").to_html();

            Either::Right(view! {
                {post_title}

                <div inner_html=result></div>

                <GiscusComments />
            })
        }
    }
}
