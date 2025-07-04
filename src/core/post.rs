use chrono::{DateTime, NaiveDateTime, Utc};
use include_dir::File;
use orgize::Org;

pub struct OrgPost<'a> {
    org: Org,
    pub filename: &'a str,
    pub created: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub thumbnail: Option<String>,
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

        OrgPost::from_orgize_obj(org, filename).ok_or("Can't retrieve org_post struct from file")
    }
}

impl<'a> OrgPost<'a> {
    fn from_orgize_obj(org: Org, filename: &'a str) -> Option<Self> {
        let properties = org.document().properties()?;

        let title = properties.get("TITLE")?.to_string();
        let description = properties.get("DESCRIPTION")?.to_string();
        let created = DateTime::from_naive_utc_and_offset(
            NaiveDateTime::parse_from_str(properties.get("CREATED")?.as_ref(), "%Y-%m-%d %H:%M")
                .ok()?,
            Utc,
        );

        let thumbnail = properties
            .get("THUMBNAIL")
            .map(|thumbnail| thumbnail.to_string());

        Some(OrgPost {
            org,
            filename,
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
