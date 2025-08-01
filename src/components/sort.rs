use leptos::prelude::*;

use crate::core::OrgPost;

#[component]
pub fn PostsSort(posts: RwSignal<Vec<OrgPost>>) -> impl IntoView {
    let oldest_fn = move |_| posts.write().sort_by(|a, b| a.created.cmp(&b.created));
    let newest_fn = move || posts.write().sort_by(|a, b| b.created.cmp(&a.created));
    newest_fn();
    let newest_fn = move |_| newest_fn();

    view! {
        <div class="flex items-center gap-2">
            <span class="text-sm font-medium">"Sort by:"</span>
            <div class="dropdown dropdown-hover">
                <div tabindex="0" role="button" class="btn btn-sm m-1">
                    Date
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-4 w-4 ml-1"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M19 9l-7 7-7-7"
                        />
                    </svg>
                </div>
                <ul
                    tabindex="0"
                    class="dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm"
                >
                    <li>
                        <button on:click=newest_fn onclick="document.activeElement.blur()">
                            "Newest first"
                        </button>
                    </li>
                    <li>
                        <button on:click=oldest_fn onclick="document.activeElement.blur()">
                            "Oldest first"
                        </button>
                    </li>
                </ul>
            </div>
        </div>
    }
}
