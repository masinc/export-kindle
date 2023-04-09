use serde::Serialize;

use crate::xml;

#[derive(Debug, Serialize)]
pub struct Item {
    pub asin: String,
    pub title: Title,
    pub authors: Vec<Author>,
    pub publishers: Vec<String>,
    pub publication_date: String,
    pub purchase_date: String,
    pub content_type: String,
}

impl From<xml::Metadata> for Item {
    fn from(metadata: xml::Metadata) -> Self {
        Self {
            asin: metadata.asin,
            title: metadata.title.into(),
            authors: metadata
                .authors
                .author
                .unwrap_or_default()
                .into_iter()
                .map(|x| x.into())
                .collect(),
            publishers: metadata
                .publishers
                .into_iter()
                .filter_map(|xp| xp.publisher)
                .collect(),
            publication_date: metadata.publication_date,
            purchase_date: metadata.purchase_date,
            content_type: metadata.content_type,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Title {
    pub pronunciation: Option<String>,
    pub name: String,
}

impl From<xml::Title> for Title {
    fn from(title: xml::Title) -> Self {
        Self {
            pronunciation: title.pronunciation,
            name: title.text,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Author {
    pub pronunciation: Option<String>,
    pub name: String,
}

impl From<xml::Author> for Author {
    fn from(author: xml::Author) -> Self {
        Self {
            pronunciation: author.pronunciation,
            name: author.text,
        }
    }
}
