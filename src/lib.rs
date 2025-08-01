mod components;
mod core;
mod pages;

use include_dir::include_dir;
use include_dir::Dir;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;
use pages::about::About;
use pages::blog::Blog;
use pages::post::Post;

pub static POSTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/posts");

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/blog">
            <nav class="sticky top-0 z-50">
                <div class="navbar bg-base-100 shadow-sm">
                    <div class="flex-1">
                        <A href="posts">
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

            <main class="w-full relative">
                <div class="flex justify-center">
                    <div class="w-full max-w-3xl flex-grow p-4 pb-10">
                        <Routes fallback=|| "Not found.">
                            <Route path=path!("/posts") view=Blog />
                            <Route path=path!("/posts/:post_filename") view=Post />
                            <Route path=path!("/*any") view=About />
                        </Routes>
                    </div>
                </div>
            </main>
        </Router>
    }
}
