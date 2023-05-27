//! # Sound
//! Structs for the `uts` file format

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::*;

/// Represents a `Uts` structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct Uts {
    /// Indicates if the sound is active.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
    pub active: Option<NwValue<u8>>,

    /// Comment associated with the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,

    /// Indicates if the sound is continuous.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Continuous")]
    pub continuous: Option<NwValue<u8>>,

    /// Elevation of the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Elevation")]
    pub elevation: Option<NwValue<Decimal>>,

    /// Hours associated with the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Hours")]
    pub hours: Option<NwValue<u8>>,

    /// Interval of the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Interval")]
    pub interval: Option<NwValue<i32>>,

    /// Interval variation of the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "IntervalVrtn")]
    pub interval_vrtn: Option<NwValue<i32>>,

    /// Localized name of the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocName")]
    pub loc_name: Option<NwValue<LocalizedText>>,

    /// Indicates if the sound is looping.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Looping")]
    pub looping: Option<NwValue<u8>>,

    /// Maximum distance of the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "MaxDistance")]
    pub max_distance: Option<NwValue<Decimal>>,

    /// Minimum distance of the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "MinDistance")]
    pub min_distance: Option<NwValue<Decimal>>,

    /// Palette ID of the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "PaletteID")]
    pub palette_id: Option<NwValue<i16>>,

    /// Pitch variation of the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "PitchVariation")]
    pub pitch_variation: Option<NwValue<Decimal>>,

    /// Indicates if the sound is positional.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Positional")]
    pub positional: Option<NwValue<u8>>,

    /// Priority of the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
    pub priority: Option<NwValue<i16>>,

    /// Indicates if the sound is random.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Random")]
    pub random: Option<NwValue<u8>>,

    /// Indicates if the sound position is random.
    #[serde(skip_serializing_if = "Option::is_none", rename = "RandomPosition")]
    pub random_position: Option<NwValue<u8>>,

    /// Random range on the X-axis.
    #[serde(skip_serializing_if = "Option::is_none", rename = "RandomRangeX")]
    pub random_range_x: Option<NwValue<Decimal>>,

    /// Random range on the Y-axis.
    #[serde(skip_serializing_if = "Option::is_none", rename = "RandomRangeY")]
    pub random_range_y: Option<NwValue<Decimal>>,

    /// Sounds associated with the structure.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Sounds")]
    pub sounds: Option<NwValue<Vec<Sound>>>,

    /// Tag associated with the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,

    /// Template ResRef of the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,

    /// Times associated with the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Times")]
    pub times: Option<NwValue<u8>>,

    /// Volume of the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Volume")]
    pub volume: Option<NwValue<u16>>,

    /// Volume variation of the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "VolumeVrtn")]
    pub volume_vrtn: Option<NwValue<u32>>,
}

/// Represents a `Sound` structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct Sound {
    /// The structure ID.
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// The sound associated with the structure.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Sound")]
    pub sound: Option<NwValue<String>>,
}
