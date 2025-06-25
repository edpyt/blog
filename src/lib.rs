mod pages;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;
use pages::about::About;
use pages::blog::Blog;
use pages::post::Post;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/blog">
            <nav>
                <div class="navbar bg-base-100 shadow-sm">
                    <div class="flex-1">
                        <A href="">
                            <span class="text-xl font-bold">"thoughts"</span>
                        </A>
                    </div>
                    <div class="flex-none">
                        <ul class="menu menu-horizontal px-1">
                            <li>
                                <A href="posts">"Blog"</A>
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
                            <Route path=path!("/posts") view=Blog />
                            <Route path=path!("/posts/:post_uuid") view=Post />
                            <Route path=path!("/*any") view=About />
                        </Routes>
                    </div>
                </div>
            </main>
        </Router>
    }
}
