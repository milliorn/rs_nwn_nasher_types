//! # Placeable
//! Structs for the `utp` file format

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Utp {
    // Animation state of the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "AnimationState")]
    pub animation_state: Option<NwValue<u8>>,

    // Appearance of the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "Appearance")]
    pub appearance: Option<NwValue<u32>>,

    // Key required to automatically remove the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "AutoRemoveKey")]
    pub auto_remove_key: Option<NwValue<u8>>,

    // Whether the placeable is a body bag
    #[serde(skip_serializing_if = "Option::is_none", rename = "BodyBag")]
    pub body_bag: Option<NwValue<u8>>,

    // Difficulty class for closing and locking the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "CloseLockDC")]
    pub close_lock_dc: Option<NwValue<u8>>,

    // Comment associated with the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,

    // Conversation script associated with the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "Conversation")]
    pub conversation: Option<NwValue<String>>,

    // Current hit points of the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentHP")]
    pub current_hp: Option<NwValue<u32>>,

    // Description of the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
    pub description: Option<NwValue<LocalizedText>>,

    // Difficulty class for disarming the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "DisarmDC")]
    pub disarm_dc: Option<NwValue<u8>>,

    // Faction of the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "Faction")]
    pub faction: Option<NwValue<u32>>,

    // Fortitude saving throw modifier for the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "Fort")]
    pub fortitude: Option<NwValue<u8>>,

    // Hardness of the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "Hardness")]
    pub hardness: Option<NwValue<u8>>,

    // Whether the placeable has inventory
    #[serde(skip_serializing_if = "Option::is_none", rename = "HasInventory")]
    pub has_inventory: Option<NwValue<u8>>,

    // Hit points of the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "HP")]
    pub hp: Option<NwValue<u32>>,

    // Whether the placeable is interruptable
    #[serde(skip_serializing_if = "Option::is_none", rename = "Interruptable")]
    pub interruptable: Option<NwValue<u8>>,

    // Key name required to interact with the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "KeyName")]
    pub key_name: Option<NwValue<String>>,

    // Whether a key is required to interact with the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "KeyRequired")]
    pub key_required: Option<NwValue<u8>>,

    // Whether the placeable is lockable
    #[serde(skip_serializing_if = "Option::is_none", rename = "Lockable")]
    pub lockable: Option<NwValue<u8>>,

    // Whether the placeable is locked
    #[serde(skip_serializing_if = "Option::is_none", rename = "Locked")]
    pub locked: Option<NwValue<u8>>,

    // Localized name of the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocName")]
    pub loc_name: Option<NwValue<LocalizedText>>,

    // Script to run when the placeable is closed
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnClosed")]
    pub on_closed: Option<NwValue<String>>,

    // Script to run when the placeable is damaged
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDamaged")]
    pub on_damaged: Option<NwValue<String>>,

    // Script to run when the placeable is destroyed
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDeath")]
    pub on_death: Option<NwValue<String>>,

    // Script to run when the placeable is disarmed
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDisarm")]
    pub on_disarm: Option<NwValue<String>>,

    // Script to run on the heartbeat of the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnHeartbeat")]
    pub on_heartbeat: Option<NwValue<String>>,

    // Script to run when the placeable's inventory is disturbed
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnInvDisturbed")]
    pub on_inv_disturbed: Option<NwValue<String>>,

    // Script to run when the placeable is locked
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnLock")]
    pub on_lock: Option<NwValue<String>>,

    // Script to run when the placeable is attacked in melee
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnMeleeAttacked")]
    pub on_melee_attacked: Option<NwValue<String>>,

    // Script to run when the placeable is opened
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnOpen")]
    pub on_open: Option<NwValue<String>>,

    // Script to run when a spell is cast at the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnSpellCastAt")]
    pub on_spell_cast_at: Option<NwValue<String>>,

    // Script to run when a trap is triggered on the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnTrapTriggered")]
    pub on_trap_triggered: Option<NwValue<String>>,

    // Script to run when the placeable is unlocked
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUnlock")]
    pub on_unlock: Option<NwValue<String>>,

    // Script to run when the placeable is used
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUsed")]
    pub on_used: Option<NwValue<String>>,

    // Script to run when the placeable has a user-defined event
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUserDefined")]
    pub on_user_defined: Option<NwValue<String>>,

    // Difficulty class for opening the lock on the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "OpenLockDC")]
    pub open_lock_dc: Option<NwValue<u8>>,

    // ID of the palette for the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "PaletteID")]
    pub palette_id: Option<NwValue<u8>>,

    // Whether the placeable is part of a plot
    #[serde(skip_serializing_if = "Option::is_none", rename = "Plot")]
    pub plot: Option<NwValue<u8>>,

    // ID of the portrait for the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "PortraitId")]
    pub portrait_id: Option<NwValue<u16>>,

    // Reflex save difficulty class for the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "Ref")]
    pub reflex: Option<NwValue<u8>>,

    // Whether the placeable is static
    #[serde(skip_serializing_if = "Option::is_none", rename = "Static")]
    pub _static: Option<NwValue<u8>>,

    // Tag of the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,

    // Resref of the template for the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,

    // Whether the trap on the placeable is detectable
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDetectable")]
    pub trap_detectable: Option<NwValue<u8>>,

    // Difficulty class for detecting the trap on the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDetectDC")]
    pub trap_detect_dc: Option<NwValue<u8>>,

    // Whether the trap on the placeable is disarmable
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDisarmable")]
    pub trap_disarmable: Option<NwValue<u8>>,

    // Flag indicating the trap status of the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapFlag")]
    pub trap_flag: Option<NwValue<u8>>,

    // Whether the trap on the placeable is one-shot
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapOneShot")]
    pub trap_one_shot: Option<NwValue<u8>>,

    // Type of the trap on the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapType")]
    pub trap_type: Option<NwValue<u8>>,

    // Type of the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
    pub _type: Option<NwValue<u8>>,

    // Whether the placeable is usable
    #[serde(skip_serializing_if = "Option::is_none", rename = "Usable")]
    pub usable: Option<NwValue<u8>>,

    // Will save difficulty class for the placeable
    #[serde(skip_serializing_if = "Option::is_none", rename = "Will")]
    pub will: Option<NwValue<u8>>,
}
