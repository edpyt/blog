use leptos::prelude::*;
use leptos_router::components::A;
use orgize::Org;
use uuid::Uuid;

#[component]
pub fn Blog() -> impl IntoView {
    let mut writer = Vec::new();
    Org::parse("* title\ntest").write_html(&mut writer).unwrap();
    let result = String::from_utf8(writer).unwrap();

    view! {
        <div inner_html=result></div>

        <A href=move || {
            let buf: [u8; 16] = *b"abcdefghijklmnop";
            let uuid = Uuid::new_v8(buf);
            uuid.to_string()
        }>
            <span class="btn">"click this post"</span>
        </A>
    }
}
