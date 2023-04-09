use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Response {
    pub sync_time: String,
    pub cache_metadata: CacheMetadata,
    pub add_update_list: AddUpdatelist,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CacheMetadata {
    pub version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AddUpdatelist {
    pub meta_data: Vec<Metadata>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Metadata {
    #[serde(rename = "ASIN")]
    pub asin: String,
    pub title: Title,
    pub authors: Authors,
    pub publishers: Vec<Publisher>,
    pub publication_date: String,
    pub purchase_date: String,
    // pub textbook_type: Option<String>,
    pub cde_contenttype: String,
    pub content_type: String,
    // pub origins: Vec<Origin>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Title {
    #[serde(rename = "@pronunciation")]
    pub pronunciation: Option<String>,

    #[serde(rename = "$text")]
    pub text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Authors {
    pub author: Option<Vec<Author>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Author {
    #[serde(rename = "@pronunciation")]
    pub pronunciation: Option<String>,

    #[serde(rename = "$text")]
    pub text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Publisher {
    pub publisher: Option<String>,
}
