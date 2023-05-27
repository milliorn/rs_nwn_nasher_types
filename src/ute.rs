//! # Encounter
//! Structs for the `ute` file format

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::*;

/// Represents an Encounter in the `ute` file format.
#[derive(Debug, Serialize, Deserialize)]
pub struct Ute {
    /// Indicates if the Encounter is active.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
    pub active: Option<NwValue<u8>>,

    /// Comment for the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,

    /// List of creatures in the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "CreatureList")]
    pub creature_list: Option<NwValue<Vec<Creature>>>,

    /// The difficulty level of the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Difficulty")]
    pub difficulty: Option<NwValue<u8>>,

    /// The difficulty index of the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "DifficultyIndex")]
    pub difficulty_index: Option<NwValue<u16>>,

    /// The faction of the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Faction")]
    pub faction: Option<NwValue<u16>>,

    /// The localized name of the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocalizedName")]
    pub localized_name: Option<NwValue<LocalizedText>>,

    /// The maximum number of creatures in the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "MaxCreatures")]
    pub max_creatures: Option<NwValue<u8>>,

    /// The script to run when a creature enters the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnEntered")]
    pub on_entered: Option<NwValue<String>>,

    /// The script to run when the Encounter is exhausted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnExhausted")]
    pub on_exhausted: Option<NwValue<String>>,

    /// The script to run when a creature exits the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnExit")]
    pub on_exit: Option<NwValue<String>>,

    /// The script to run on the Encounter's heartbeat.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnHeartbeat")]
    pub on_heartbeat: Option<NwValue<String>>,

    /// The script to run when a user-defined event occurs in the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUserDefined")]
    pub on_user_defined: Option<NwValue<String>>,

    /// The palette ID of the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "PaletteID")]
    pub palette_id: Option<NwValue<i32>>,

    /// Indicates if the Encounter is limited to players only.
    #[serde(skip_serializing_if = "Option::is_none", rename = "PlayerOnly")]
    pub player_only: Option<NwValue<u8>>,

    /// The recommended number of creatures in the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "RecCreatures")]
    pub rec_creatures: Option<NwValue<u8>>,

    /// Indicates if the Encounter should be reset.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Reset")]
    pub reset: Option<NwValue<u8>>,

    /// The reset time of the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ResetTime")]
    pub reset_time: Option<NwValue<i32>>,

    /// The number of times the Encounter respawns.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Respawns")]
    pub respawns: Option<NwValue<i16>>,

    /// The spawn option of the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "SpawnOption")]
    pub spawn_option: Option<NwValue<i32>>,

    /// The tag of the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,

    /// The template resource reference (ResRef) of the Encounter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,
}

/// Represents a creature in an Encounter.
#[derive(Debug, Serialize, Deserialize)]
pub struct Creature {
    /// The unique identifier of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// The appearance of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Appearance")]
    pub appearance: Option<NwValue<u32>>,

    /// The challenge rating (CR) of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "CR")]
    pub challenge_rating: Option<NwValue<Decimal>>,

    /// The resource reference (ResRef) of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ResRef")]
    pub res_ref: Option<NwValue<String>>,

    /// Indicates if only a single instance of the creature should be spawned.
    #[serde(skip_serializing_if = "Option::is_none", rename = "SingleSpawn")]
    pub single_spawn: Option<NwValue<u8>>,
}
