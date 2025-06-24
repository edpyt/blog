mod pages;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;
use pages::about::About;
use pages::blog::Blog;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/blog">
            <nav>
                <div class="navbar bg-base-100 shadow-sm">
                    <div class="flex-1">
                        <span class="text-xl font-bold">thoughts</span>
                    </div>
                    <div class="flex-none">
                        <ul class="menu menu-horizontal px-1">
                            <li>
                                <A href="articles">"Blog"</A>
                            </li>
                            <li>
                                <A href="">"About"</A>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>

            <main class="w-full">
                <div class="flex justify-center">
                    <div class="py-6 prose lg:prose-lg">
                        <Routes fallback=|| "Not found.">
                            <Route path=path!("/articles") view=Blog />
                            <Route path=path!("/*any") view=About />
                        </Routes>
                    </div>
                </div>
            </main>
        </Router>
    }
}
