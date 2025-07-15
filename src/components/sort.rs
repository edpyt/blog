use leptos::prelude::*;

use crate::core::OrgPost;

#[component]
pub fn PostsSort<'a>(posts: RwSignal<Vec<RwSignal<OrgPost<'a>>>>) -> impl IntoView {
    view! {}
}
