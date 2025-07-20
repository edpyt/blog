use std::sync::Arc;

use crate::core::constants::{DEFAULT_THUMBNAIL, IMAGES_CDN};
use chrono::{DateTime, NaiveDateTime, Utc};
use include_dir::File;
use orgize::Org;

#[derive(Clone)]
pub struct OrgPost {
    org: Arc<Org>,
    pub filename: String,
    pub created: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub thumbnail: String,
}

impl<'a> TryFrom<&File<'a>> for OrgPost {
    type Error = &'static str;

    fn try_from(file: &File<'a>) -> Result<Self, Self::Error> {
        let filename = file
            .path()
            .file_name()
            .expect("can't retrieve file name")
            .to_str()
            .unwrap();
        let org = Org::parse(file.contents_utf8().unwrap());

        OrgPost::new(org, filename).ok_or("Can't retrieve org_post struct from file")
    }
}

impl OrgPost {
    fn new(org: Org, filename: &'_ str) -> Option<Self> {
        let properties = org.document().properties()?;

        // NOTE: required
        let title = properties.get("TITLE")?.to_string();
        let description = properties.get("DESCRIPTION")?.to_string();
        let created = DateTime::from_naive_utc_and_offset(
            NaiveDateTime::parse_from_str(properties.get("CREATED")?.as_ref(), "%Y-%m-%d %H:%M")
                .ok()?,
            Utc,
        );

        // NOTE: optional
        let thumbnail = match properties.get("THUMBNAIL") {
            Some(token) => token.to_string(),
            None => format!("{IMAGES_CDN}{DEFAULT_THUMBNAIL}"),
        };

        Some(OrgPost {
            org: org.into(),
            filename: filename.to_string(),
            title,
            description,
            created,
            thumbnail,
        })
    }

    pub fn content_html(&self) -> String {
        self.org.to_html()
    }
}
