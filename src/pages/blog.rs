use std::time::SystemTime;

use include_dir::File;
use leptos::prelude::*;
use leptos_router::components::A;
use orgize::Org;

use crate::POSTS_DIR;

#[component]
pub fn Blog() -> impl IntoView {
    let mut posts: Vec<OrgPost> = POSTS_DIR
        .files()
        .filter_map(|file| file.try_into().ok())
        .collect();
    posts.sort_by(|a, b| b.created.cmp(&a.created));

    view! {
        {posts
            .into_iter()
            .map(|org_post| {
                view! {
                    <div class="py-6 flex flex-row gap-6 md:gap-10 items-start">
                        <div class="flex flex-col gap-4">
                            <h2 class="text-2xl font-bold">
                                <A href=org_post.filename>
                                    <span class="hover:underline">{org_post.title}</span>
                                </A>
                            </h2>
                            <p class="text-sm text-base-content/70">{org_post.description}</p>
                        </div>
                    </div>
                }
            })
            .collect::<Vec<_>>()}
    }
}

struct OrgPost<'a> {
    filename: &'a str,
    created: SystemTime,
    title: String,
    description: String,
    content_html: String,
}

impl<'a> TryFrom<&File<'a>> for OrgPost<'a> {
    type Error = &'static str;

    fn try_from(file: &File<'a>) -> Result<Self, Self::Error> {
        let filename = file
            .path()
            .file_name()
            .expect("can't retrieve file name")
            .to_str()
            .unwrap();
        let org = Org::parse(file.contents_utf8().unwrap());
        let created = file.metadata().unwrap().created();

        Ok(OrgPost::from_orgize_obj(org, filename, created).unwrap())
    }
}

impl<'a> OrgPost<'a> {
    fn from_orgize_obj(org: Org, filename: &'a str, created: SystemTime) -> Option<Self> {
        let properties = org.document().properties()?;

        let title = properties.get("TITLE")?.to_string();
        let description = properties.get("DESCRIPTION")?.to_string();
        let content_html = org.to_html();

        Some(OrgPost {
            filename,
            title,
            description,
            content_html,
            created,
        })
    }
}
