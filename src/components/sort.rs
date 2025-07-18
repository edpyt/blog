use leptos::prelude::*;

use crate::core::OrgPost;

#[component]
pub fn PostsSort(posts: RwSignal<Vec<OrgPost>>) -> impl IntoView {
    posts.write().sort_by(|a, b| b.created.cmp(&a.created));
    // TODO: add sort menu
    view! {}
}
