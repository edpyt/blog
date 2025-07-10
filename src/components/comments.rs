use leptos::prelude::*;

#[component]
pub fn GiscusComments() -> impl IntoView {
    view! {
        <div class="giscus"></div>

        <script
            src="https://giscus.app/client.js"
            data-repo="edpyt/blog"
            data-repo-id="R_kgDOPA8L5Q"
            data-category="General"
            data-category-id="DIC_kwDOPA8L5c4Cr9M0"
            data-mapping="url"
            data-strict="0"
            data-reactions-enabled="1"
            data-emit-metadata="0"
            data-input-position="bottom"
            data-theme="catppuccin_mocha"
            data-lang="en"
            crossorigin="anonymous"
            async
        ></script>
    }
}
