mod pages;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;
use pages::about::About;
use pages::blog::Blog;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/blog/">
            <nav>
                <div class="navbar bg-base-100 shadow-sm">
                    <div class="flex-1">
                        <span class="text-xl font-bold">thoughts</span>
                    </div>
                    <div class="flex-none">
                        <ul class="menu menu-horizontal px-1">
                            <li>
                                <A href="/blog/articles">"Blog"</A>
                            </li>
                            <li>
                                <A href="/blog/">"About"</A>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>

            <main class="w-full text-center">
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/articles") view=Blog />
                    <Route path=path!("/") view=About />
                // TODO: <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> } />
                </Routes>
            </main>
        </Router>
    }
}
