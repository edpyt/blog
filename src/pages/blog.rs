use leptos::prelude::*;
use orgize::Org;

#[component]
pub fn Blog() -> impl IntoView {
    let mut writer = Vec::new();
    Org::parse("* title\ntest").write_html(&mut writer).unwrap();
    let result = String::from_utf8(writer).unwrap();
    view! { <div inner_html=result></div> }
}
