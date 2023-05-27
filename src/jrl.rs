//! # Journal
//! Structs for the `jrl` file format

use serde::{Deserialize, Serialize};

use super::*;

/// Represents the JRL (Journal) structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct Jrl {
    /// The categories within the journal.
    #[serde(rename = "Categories")]
    pub categories: Option<NwValue<Vec<Category>>>,
}

/// Represents a category within the journal.
#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    /// The unique identifier for the category.
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// The comment associated with the category.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,

    /// The list of entries within the category.
    #[serde(skip_serializing_if = "Option::is_none", rename = "EntryList")]
    pub entry_list: Option<NwValue<Vec<Entry>>>,

    /// The name of the category.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
    pub name: Option<NwValue<LocalizedText>>,

    /// The picture associated with the category.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Picture")]
    pub picture: Option<NwValue<u64>>,

    /// The priority of the category.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
    pub priority: Option<NwValue<u8>>,

    /// The tag associated with the category.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,

    /// The XP (experience points) associated with the category.
    #[serde(skip_serializing_if = "Option::is_none", rename = "XP")]
    pub xp: Option<NwValue<u32>>,
}

/// Represents an entry within a category in the journal.
#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    /// The unique identifier for the entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// The end position of the entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "End")]
    pub end: Option<NwValue<u32>>,

    /// The ID of the entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ID")]
    pub id: Option<NwValue<u32>>,

    /// The text content of the entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Text")]
    pub text: Option<NwValue<LocalizedText>>,
}
