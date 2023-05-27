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

    /// Indicates if the sound is positional.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Positional")]
    pub positional: Option<NwValue<u8>>,

    /// Priority of the sound.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
    pub priority: Option<NwValue<i16>>,

    /// Sounds associated with the structure.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Sounds")]
    pub sounds: Option<NwValue<Vec<Sound>>>,

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
