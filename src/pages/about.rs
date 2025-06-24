use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    let increment = 5;
    let (count, set_count) = signal(0);

    view! {
        <h1>"Welcome to Leptos"</h1>
        <h2>
            <i>"On Github Pages"</i>
        </h2>
        <button class="btn" on:click=move |_| { set_count.set(count.get() + increment) }>
            {move || count.get()}
        </button>

        <h1 class="m-8 text-4xl">"Welcome to Leptos!"</h1>
        <button class="mx-8 h-auto p-4 border rounded-md  bg-purple-400 hover:bg-purple-700">
            BOB
        </button>
    }
}
