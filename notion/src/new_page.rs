use std::str::FromStr;

use rusticnotion::{
    ids::PropertyId,
    models::{
        properties::{Color, External, FileReference, PropertyValue, SelectedValue},
        text::{RichText, RichTextCommon, Text},
        Database,
    },
};

pub struct NewPage {
    pub parent_database: Database,
    pub name: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl NewPage {
    fn empty_id(&self) -> PropertyId {
        PropertyId::from_str("").unwrap()
    }

    pub fn get_name_property(&self) -> Option<PropertyValue> {
        if let Some(name) = &self.name {
            let name_property: PropertyValue = PropertyValue::Title {
                id: self.empty_id(),
                title: [RichText::Text {
                    text: Text {
                        content: name.clone(),
                        link: None,
                    },
                    rich_text: RichTextCommon {
                        plain_text: name.clone(),
                        href: None,
                        annotations: None,
                    },
                }]
                .to_vec(),
            };

            Some(name_property)
        } else {
            None
        }
    }

    pub fn get_url_property(&self) -> Option<PropertyValue> {
        if let Some(url) = &self.url {
            let url_property: PropertyValue = PropertyValue::Url {
                id: self.empty_id(),
                url: Some(url.to_string()),
            };

            Some(url_property)
        } else {
            None
        }
    }

    pub fn get_image_property(&self) -> Option<PropertyValue> {
        if let Some(image_url) = &self.image_url {
            let image_property: PropertyValue = PropertyValue::Files {
                id: self.empty_id(),
                files: Some(
                    [FileReference::External {
                        name: "Image".to_string(),
                        external: External {
                            url: image_url.to_string(),
                        },
                    }]
                    .to_vec(),
                ),
            };

            Some(image_property)
        } else {
            None
        }
    }

    pub fn get_tags_property(&self) -> Option<PropertyValue> {
        if let Some(tags) = &self.tags {
            let tags_property: PropertyValue = PropertyValue::MultiSelect {
                id: self.empty_id(),
                multi_select: Some(
                    tags.iter()
                        .map(|tag| SelectedValue {
                            id: None,
                            name: Some(tag.to_string()),
                            color: Color::Default,
                        })
                        .collect(),
                ),
            };

            Some(tags_property)
        } else {
            None
        }
    }
}
