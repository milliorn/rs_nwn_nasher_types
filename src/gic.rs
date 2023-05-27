//! # Area comments
//! Structs for the `gic` file format

use super::*;
use serde::{Deserialize, Serialize};

/// Gic is a struct that represents area comments
#[derive(Debug, Serialize, Deserialize)]
pub struct Gic {
    /// Creature List is a list of creatures
    #[serde(skip_serializing_if = "Option::is_none", rename = "Creature List")]
    pub creature_list: Option<NwValue<Vec<Comment>>>,
    /// Door List is a list of doors
    #[serde(skip_serializing_if = "Option::is_none", rename = "Door List")]
    pub door_list: Option<NwValue<Vec<Comment>>>,
    /// Encounter List is a list of encounters
    #[serde(skip_serializing_if = "Option::is_none", rename = "Encounter List")]
    pub encounter_list: Option<NwValue<Vec<Comment>>>,
    /// List is a generic list
    #[serde(skip_serializing_if = "Option::is_none", rename = "List")]
    pub list: Option<NwValue<Vec<Comment>>>,
    /// Placeable List is a list of placeables
    #[serde(skip_serializing_if = "Option::is_none", rename = "Placeable List")]
    pub placeable_list: Option<NwValue<Vec<Comment>>>,
    /// SoundList is a list of sounds
    #[serde(skip_serializing_if = "Option::is_none", rename = "SoundList")]
    pub sound_list: Option<NwValue<Vec<Comment>>>,
    /// StoreList is a list of stores
    #[serde(skip_serializing_if = "Option::is_none", rename = "StoreList")]
    pub store_list: Option<NwValue<Vec<Comment>>>,
    /// TriggerList is a list of triggers
    #[serde(skip_serializing_if = "Option::is_none", rename = "TriggerList")]
    pub trigger_list: Option<NwValue<Vec<Comment>>>,
    /// WaypointList is a list of waypoints
    #[serde(skip_serializing_if = "Option::is_none", rename = "WaypointList")]
    pub waypoint_list: Option<NwValue<Vec<Comment>>>,
}

/// Represents a comment
#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    /// Field representing the struct ID
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    /// Field representing the comment content
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    /// Field indicating whether the comment can be played in the toolset
    #[serde(skip_serializing_if = "Option::is_none", rename = "PlayInToolset")]
    pub play_in_toolset: Option<NwValue<u8>>,
}
