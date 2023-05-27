//! # Palette
//! Structs for the `itp` file format

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::*;

/// Represents the ITP (Item Palette) structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct Itp {
    /// The main entry of the ITP file.
    #[serde(rename = "MAIN")]
    pub entry: Option<NwValue<Vec<Entry>>>,
}

/// Represents an entry within the ITP structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    /// The unique identifier for the entry.
    #[serde(rename = "__struct_id")]
    pub struct_id: u32,

    /// The challenge rating (CR) of the entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "CR")]
    pub cr: Option<NwValue<Decimal>>,

    /// The faction associated with the entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "FACTION")]
    pub faction: Option<NwValue<String>>,

    /// The ID of the entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ID")]
    pub id: Option<NwValue<u32>>,

    /// The list of sub-entries within the entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "LIST")]
    pub list: Option<NwValue<Vec<Entry>>>,

    /// The name of the entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "NAME")]
    pub name: Option<NwValue<String>>,

    /// The resource reference (ResRef) of the entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "RESREF")]
    pub res_ref: Option<NwValue<String>>,

    /// The string reference (StrRef) associated with the entry.
    #[serde(skip_serializing_if = "Option::is_none", rename = "STRREF")]
    pub string_ref: Option<NwValue<u32>>,
}
