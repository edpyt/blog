use std::time::SystemTime;

use chrono::{DateTime, Utc};
use include_dir::File;
use orgize::Org;

pub struct OrgPost<'a> {
    org: Org,
    pub filename: &'a str,
    pub created: DateTime<Utc>,
    pub title: String,
    pub description: String,
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

        Some(OrgPost {
            org,
            filename,
            title,
            description,
            created: created.into(),
        })
    }

    pub fn content_html(&self) -> String {
        self.org.to_html()
    }
}
