//! # Waypoint
//! Structs for the `utw` file format

use serde::{Deserialize, Serialize};

use super::*;

/// Represents a `Utw` structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct Utw {
    /// Active flag of the waypoint.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
    pub active: Option<NwValue<u8>>,

    /// Appearance of the waypoint.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Appearance")]
    pub appearance: Option<NwValue<i32>>,

    /// Comment associated with the waypoint.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,

    /// Description of the waypoint.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
    pub description: Option<NwValue<LocalizedText>>,

    /// Flag indicating if the waypoint has a map note.
    #[serde(skip_serializing_if = "Option::is_none", rename = "HasMapNote")]
    pub has_map_note: Option<NwValue<u8>>,

    /// Waypoint linked to another waypoint.
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkedTo")]
    pub linked_to: Option<NwValue<String>>,

    /// Localized name of the waypoint.
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocalizedName")]
    pub localized_name: Option<NwValue<LocalizedText>>,

    /// Map note associated with the waypoint.
    #[serde(skip_serializing_if = "Option::is_none", rename = "MapNote")]
    pub map_note: Option<NwValue<LocalizedText>>,

    /// Flag indicating if the map note is enabled for the waypoint.
    #[serde(skip_serializing_if = "Option::is_none", rename = "MapNoteEnabled")]
    pub map_note_enabled: Option<NwValue<u8>>,

    /// Palette ID associated with the waypoint.
    #[serde(skip_serializing_if = "Option::is_none", rename = "PaletteID")]
    pub palette_id: Option<NwValue<i32>>,

    /// Tag associated with the waypoint.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,

    /// Template ResRef of the waypoint.
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,
}
