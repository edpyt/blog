use leptos::prelude::*;
use orgize::Org;

#[component]
pub fn Blog() -> impl IntoView {
    let (orgmode_file,_) = signal(read_orgmode_file());
    view! { "Here is a my simple blog" }
}

async fn read_orgmode_file() -> String {
    let resp = reqwest::get("assets/blog/test.org").await.map_err(|_| "Failed to load file").unwrap();
    let text = resp.text().await.map_err(|_| "Invalid file content").unwrap();
    let mut result=Vec::new();
    Org::parse(&text).write_html(&mut result).unwrap();
    String::from_utf8(result).unwrap()
}
