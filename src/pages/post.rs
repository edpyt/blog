use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use orgize::Org;

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
        Some(post_uuid) => {
            let mut writer = Vec::new();
            Org::parse("* title\ntest").write_html(&mut writer).unwrap();
            let result = String::from_utf8(writer).unwrap();

            Either::Right(view! {
                {post_uuid}

                <div inner_html=result></div>

                <GiscusComments />
            })
        }
    }
}
