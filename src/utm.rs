//! # Store
//! Structs for the `utm` file format

use serde::{Deserialize, Serialize};

use super::*;

/// Represents the Utm struct, which contains fields for the `utm` file format.
#[derive(Debug, Serialize, Deserialize)]
pub struct Utm {
    /// Represents the black market field in the Utm struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "BlackMarket")]
    pub black_market: Option<NwValue<u8>>,

    /// Represents the black market mark down field in the Utm struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "BM_MarkDown")]
    pub bm_mark_down: Option<NwValue<i32>>,

    /// Represents the comment field in the Utm struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,

    /// Represents the ID field in the Utm struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ID")]
    pub id: Option<NwValue<u8>>,

    /// Represents the LocName field in the Utm struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocName")]
    pub loc_name: Option<NwValue<LocalizedText>>,

    /// Represents the MarkDown field in the Utm struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "MarkDown")]
    pub mark_down: Option<NwValue<i32>>,

    /// Represents the MarkUp field in the Utm struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "MarkUp")]
    pub mark_up: Option<NwValue<i32>>,

    /// Represents the OnOpenStore field in the Utm struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnOpenStore")]
    pub on_open_store: Option<NwValue<String>>,

    /// Represents the ResRef field in the Utm struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ResRef")]
    pub res_ref: Option<NwValue<String>>,

    /// Represents the StoreList field in the Utm struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "StoreList")]
    pub store_list: Option<NwValue<Vec<Store>>>,

    /// Represents the Tag field in the Utm struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
}

/// Represents the Store struct within the Utm struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    /// Represents the struct_id field in the Store struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// Represents the ItemList field in the Store struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ItemList")]
    pub item_list: Option<NwValue<Vec<Item>>>,
}

/// Represents the Item struct within the Store struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    /// Represents the struct_id field in the Item struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// Represents the Infinite field in the Item struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Infinite")]
    pub infinite: Option<NwValue<u8>>,

    /// Represents the InventoryRes field in the Item struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "InventoryRes")]
    pub inventory_res: Option<NwValue<String>>,

    /// Represents the Repos_PosX field in the Item struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Repos_PosX")]
    pub repos_pos_x: Option<NwValue<u32>>,

    /// Represents the Repos_Posy field in the Item struct.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Repos_Posy")]
    pub repos_posy: Option<NwValue<u32>>,
}
