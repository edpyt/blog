use leptos::prelude::*;
use leptos_router::components::A;

use crate::{core::OrgPost, POSTS_DIR};

#[component]
pub fn Blog() -> impl IntoView {
    let mut posts: Vec<OrgPost> = POSTS_DIR
        .files()
        .filter_map(|file| file.try_into().ok())
        .collect();
    posts.sort_by(|a, b| b.created.cmp(&a.created));

    view! {
        {posts
            .into_iter()
            .map(|org_post| {
                view! {
                    <div class="py-6 flex flex-row gap-6 md:gap-10 items-start">
                        <img
                            class="h-25 w-25 object-cover"
                            src=move || {
                                org_post
                                    .thumbnail
                                    .clone()
                                    .unwrap_or(
                                        "https://raw.githubusercontent.com/edpyt/blog/refs/heads/main/assets/images/pride-trans-thumb.png"
                                            .to_string(),
                                    )
                            }
                        />
                        <div class="w-full flex flex-col gap-4">
                            <div class="flex justify-between">
                                <h2 class="text-2xl font-bold">
                                    <A href=org_post.filename>
                                        <span class="hover:underline">{org_post.title}</span>
                                    </A>
                                </h2>
                                <span class="text-end text-sm text-base-content/70">
                                    {org_post.created.format("%Y-%m-%d %H:%M:%S").to_string()}
                                </span>
                            </div>
                            <p class="text-sm text-base-content/70">{org_post.description}</p>
                        </div>
                    </div>
                }
            })
            .collect::<Vec<_>>()}
    }
}
