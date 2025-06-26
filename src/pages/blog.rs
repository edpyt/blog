use anyhow::Result;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Blog() -> impl IntoView {
    let posts_fps: RwSignal<Vec<String>> = RwSignal::new(vec![]);

    LocalResource::new(
        move || load_data(posts_fps)
    );

    view! {
        <A href="first-test-post">
            <span class="btn">"click this post"</span>
        </A>
    }
}

async fn load_data(posts_fps: RwSignal<Vec<String>>) -> Result<()> {
    reqwest::get("/posts/").await?;
    posts_fps.update(vec![]);
    Ok(())
}
