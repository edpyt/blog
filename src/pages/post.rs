use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use crate::pages::comments::GiscusComments;

#[derive(Params, PartialEq, Clone, Debug)]
struct PostParams {
    post_uuid: Option<String>,
}

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params::<PostParams>();
    let post_uuid = params
        .get_untracked()
        .map(|params| params.post_uuid.unwrap_or_default())
        .ok();

    match post_uuid {
        None => Either::Left(view! { <p>"No post with this UUID was found."</p> }),
        Some(post_uuid) => Either::Right(view! {
            {post_uuid}
            <GiscusComments />
        }),
    }
}
