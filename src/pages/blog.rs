use leptos::prelude::*;
use leptos_router::components::A;

use crate::{
    components::PostsSort,
    core::{constants::DATETIME_FORMAT, OrgPost},
    POSTS_DIR,
};

#[component]
pub fn Blog() -> impl IntoView {
    let posts = RwSignal::new(
        POSTS_DIR
            .files()
            .filter_map(|file| file.try_into().ok())
            .collect::<Vec<OrgPost>>(),
    );

    view! {
        <PostsSort posts=posts />

        <For
            each=move || posts.read().clone()
            key=|org_post| org_post.filename.clone()
            children=move |org_post| {
                view! {
                    <div class="py-6 flex flex-row gap-6 md:gap-10 items-start">
                        <img class="h-25 w-25 object-cover" src=org_post.thumbnail />
                        <div class="w-full flex flex-col gap-4">
                            <div class="flex justify-between">
                                <h2 class="text-2xl font-bold">
                                    <A href=org_post.filename>
                                        <span class="hover:underline">{org_post.title}</span>
                                    </A>
                                </h2>
                                <span class="text-end text-sm text-base-content/70">
                                    {org_post.created.format(DATETIME_FORMAT).to_string()}
                                </span>
                            </div>
                            <p class="text-sm text-base-content/70">{org_post.description}</p>
                        </div>
                    </div>
                }
            }
        />
    }
}
