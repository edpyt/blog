use anyhow::Result;
use leptos::{logging, prelude::*};
use leptos_router::components::A;

#[component]
pub fn Blog() -> impl IntoView {
    let posts_fps: RwSignal<Vec<String>> = RwSignal::new(vec![]);

    LocalResource::new(
        move || async move {match load_data(posts_fps).await {
            Ok(posts) => logging::log!("{:?}",posts),
            Err(e) => {
                logging::error!("Failed to get posts! {e:?}")
            }
        }}
    );

    view! {
        <A href="first-test-post">
            <span class="btn">"click this post"</span>
        </A>
    }
}

async fn load_data(_posts_fps: RwSignal<Vec<String>>) -> Result<()> {
    reqwest::get("http://localhost:8080/posts/").await?;
    Ok(())
}
