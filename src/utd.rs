//! # Door
//! Structs for the `utd` file format

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Utd {
    // Represents the animation state of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "AnimationState")]
    pub animation_state: Option<NwValue<i32>>,

    // Represents the appearance of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Appearance")]
    pub appearance: Option<NwValue<i32>>,

    // Indicates whether the key is automatically removed when used to open the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "AutoRemoveKey")]
    pub auto_remove_key: Option<NwValue<u8>>,

    // Represents the difficulty class (DC) for closing and locking the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "CloseLockDC")]
    pub close_lock_dc: Option<NwValue<i32>>,

    // Additional comments or notes about the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,

    // The conversation associated with the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Conversation")]
    pub conversation: Option<NwValue<String>>,

    // Represents the current hit points (HP) of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentHP")]
    pub current_hp: Option<NwValue<i32>>,

    // The description of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
    pub description: Option<NwValue<LocalizedText>>,

    // Represents the difficulty class (DC) for disarming any traps on the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "DisarmDC")]
    pub disarm_dc: Option<NwValue<i32>>,

    // The faction associated with the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Faction")]
    pub faction: Option<NwValue<i32>>,

    // Represents the fortitude saving throw of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Fort")]
    pub fortitude: Option<NwValue<i32>>,

    // Represents the generic type of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "GenericType")]
    pub generic_type: Option<NwValue<i32>>,

    // Represents the hardness of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Hardness")]
    pub hardness: Option<NwValue<i32>>,

    // Represents the hit points (HP) of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "HP")]
    pub hp: Option<NwValue<i32>>,

    // Indicates whether the door can be interrupted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Interruptable")]
    pub interruptable: Option<NwValue<u8>>,

    // The name of the key required to open the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "KeyName")]
    pub key_name: Option<NwValue<String>>,

    // Indicates whether a key is required to open the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "KeyRequired")]
    pub key_required: Option<NwValue<u8>>,

    // Represents the object tag of the door that this door is linked to.
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkedTo")]
    pub linked_to: Option<NwValue<String>>,

    // Represents the door flags of the linked door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkedToFlags")]
    pub linked_to_flags: Option<NwValue<i32>>,

    // The load screen ID associated with the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "LoadScreenID")]
    pub load_screen_id: Option<NwValue<i32>>,

    // Indicates whether the door is lockable.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Lockable")]
    pub lockable: Option<NwValue<u8>>,

    // Indicates whether the door is locked.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Locked")]
    pub locked: Option<NwValue<u8>>,

    // The localized name of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocName")]
    pub loc_name: Option<NwValue<LocalizedText>>,

    // The script to run when the door is clicked.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnClick")]
    pub on_click: Option<NwValue<String>>,

    // The script to run when the door is closed.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnClosed")]
    pub on_closed: Option<NwValue<String>>,

    // The script to run when the door is damaged.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDamaged")]
    pub on_damaged: Option<NwValue<String>>,

    // The script to run when the door is destroyed.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDeath")]
    pub on_death: Option<NwValue<String>>,

    // The script to run when the door is disarmed.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDisarm")]
    pub on_disarm: Option<NwValue<String>>,

    // The script to run when the door fails to open.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnFailToOpen")]
    pub on_fail_to_open: Option<NwValue<String>>,

    // The script to run on the door's heartbeat.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnHeartbeat")]
    pub on_heartbeat: Option<NwValue<String>>,

    // The script to run when the door is locked.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnLock")]
    pub on_lock: Option<NwValue<String>>,

    // The script to run when the door is melee attacked.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnMeleeAttacked")]
    pub on_melee_attacked: Option<NwValue<String>>,

    // The script to run when the door is opened.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnOpen")]
    pub on_open: Option<NwValue<String>>,

    // The script to run when a spell is cast at the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnSpellCastAt")]
    pub on_spell_cast_at: Option<NwValue<String>>,

    // The script to run when the door's trap is triggered.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnTrapTriggered")]
    pub on_trap_triggered: Option<NwValue<String>>,

    // The script to run when the door is unlocked.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUnlock")]
    pub on_unlock: Option<NwValue<String>>,

    // The script to run when a user-defined event occurs.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUserDefined")]
    pub on_user_defined: Option<NwValue<String>>,

    // The Open Lock difficulty class (DC) of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "OpenLockDC")]
    pub open_lock_dc: Option<NwValue<i32>>,

    // The palette ID of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "PaletteID")]
    pub palette_id: Option<NwValue<i32>>,

    // Indicates whether the door is related to a plot.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Plot")]
    pub plot: Option<NwValue<u8>>,

    // The portrait ID associated with the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "PortraitId")]
    pub portrait_id: Option<NwValue<i32>>,

    // The reference ID of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Ref")]
    pub ref_: Option<NwValue<i32>>,

    // The tag of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,

    // The template resource reference (ResRef) of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,

    // Indicates whether the trap is detectable on the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDetectable")]
    pub trap_detectable: Option<NwValue<u8>>,

    // The trap detection difficulty class (DC) of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDetectDC")]
    pub trap_detect_dc: Option<NwValue<i32>>,

    // Indicates whether the trap on the door is disarmable.
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDisarmable")]
    pub trap_disarmable: Option<NwValue<u8>>,

    // The flag associated with the trap on the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapFlag")]
    pub trap_flag: Option<NwValue<i32>>,

    // Indicates whether the trap on the door is a one-shot trap.
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapOneShot")]
    pub trap_one_shot: Option<NwValue<u8>>,

    // The trap type of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapType")]
    pub trap_type: Option<NwValue<i32>>,

    // The Will saving throw DC of the door.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Will")]
    pub will: Option<NwValue<i32>>,
}
