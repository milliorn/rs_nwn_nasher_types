//! # Area information
//! Structs for the `git` file format

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Git {
    #[serde(rename = "AreaProperties")]
    pub area_properties: NwStruct<AreaProperty>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "Creature List")]
    pub creature_list: Option<NwValue<Vec<Creature>>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "Door List")]
    pub door_list: Option<NwValue<Vec<Door>>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "Encounter List")]
    pub encounter_list: Option<NwValue<Vec<Encounter>>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "List")]
    pub list: Option<NwValue<Vec<List>>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "Placeable List")]
    pub placeable_list: Option<NwValue<Vec<Placeable>>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "SoundList")]
    pub sound_list: Option<NwValue<Vec<Sound>>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "StoreList")]
    pub store_list: Option<NwValue<Vec<Store>>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "TriggerList")]
    pub trigger_list: Option<NwValue<Vec<Trigger>>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "WaypointList")]
    pub waypoint_list: Option<NwValue<Vec<Waypoint>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaProperty {
    #[serde(rename = "__struct_id")]
    pub struct_id: u32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "AmbientSndDay")]
    pub ambient_snd_day: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "AmbientSndDayVol")]
    pub ambient_snd_day_vol: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "AmbientSndNight")]
    pub ambient_snd_night: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "AmbientSndNitVol")]
    pub ambient_snd_nit_vol: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "EnvAudio")]
    pub env_audio: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MusicBattle")]
    pub music_battle: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MusicDay")]
    pub music_day: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MusicDelay")]
    pub music_delay: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MusicNight")]
    pub music_night: Option<NwValue<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Creature {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Door {
    #[serde(rename = "__struct_id")]
    pub struct_id: u32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "AnimationState")]
    pub animation_state: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Appearance")]
    pub appearance: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "AutoRemoveKey")]
    pub auto_remove_key: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Bearing")]
    pub bearing: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "CloseLockDC")]
    pub close_lock_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Conversation")]
    pub conversation: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentHP")]
    pub current_hp: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
    pub description: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "DisarmDC")]
    pub disarm_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Faction")]
    pub faction: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Fort")]
    pub fortitude: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "GenericType_New")]
    pub generic_type_new: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Hardness")]
    pub hardness: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "HP")]
    pub hp: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Interruptable")]
    pub interuptable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "KeyName")]
    pub key_name: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "KeyRequired")]
    pub key_required: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkedTo")]
    pub linked_to: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkedToFlags")]
    pub linked_to_flags: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LoadScreenID")]
    pub load_screen_id: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Lockable")]
    pub lockable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Locked")]
    pub locked: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocName")]
    pub location_name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnClick")]
    pub on_click: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnClosed")]
    pub on_closed: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDamaged")]
    pub on_damaged: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDeath")]
    pub on_death: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDisarm")]
    pub on_disarm: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnFailToOpen")]
    pub on_fail_to_open: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnHeartbeat")]
    pub on_heartbeat: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnLock")]
    pub on_lock: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnMeleeAttacked")]
    pub on_melee_attacked: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnOpen")]
    pub on_open: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnSpellCastAt")]
    pub on_spell_cast_at: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnTrapTriggered")]
    pub on_trap_triggered: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUnlock")]
    pub on_unlock: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUserDefined")]
    pub on_user_defined: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OpenLockDC")]
    pub open_lock_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Plot")]
    pub plot: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PortraitId")]
    pub portrait_id: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Ref")]
    pub ref_: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDetectable")]
    pub trap_detectable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDetectDC")]
    pub trap_detect_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDisarmable")]
    pub trap_disarmable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapFlag")]
    pub trap_flag: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapOneShot")]
    pub trap_one_shot: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapType")]
    pub trap_type: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Will")]
    pub will: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "X")]
    pub x: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Y")]
    pub y: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Z")]
    pub z: Option<NwValue<Decimal>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Encounter {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
    pub active: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "CreatureList")]
    pub creature_list: Option<NwValue<Vec<EncounterCreature>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Difficulty")]
    pub difficulty: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "DifficultyIndex")]
    pub difficulty_index: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Faction")]
    pub faction: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Geometry")]
    pub geometry: Option<NwValue<Vec<EncounterGeometry>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocalizedName")]
    pub localized_name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MaxCreatures")]
    pub max_creatures: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnEntered")]
    pub on_entered: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnExhausted")]
    pub on_exhausted: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnExit")]
    pub on_exit: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnHeartbeat")]
    pub on_heartbeat: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUserDefined")]
    pub on_user_defined: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PlayerOnly")]
    pub player_only: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "RecCreatures")]
    pub rec_creatures: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Reset")]
    pub reset: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ResetTime")]
    pub reset_time: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Respawns")]
    pub respawns: Option<NwValue<i8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SpawnOption")]
    pub spawn_option: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SpawnPointList")]
    pub spawn_point_list: Option<NwValue<Vec<SpawnPoint>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "XPosition")]
    pub x_position: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "YPosition")]
    pub y_position: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ZPosition")]
    pub z_position: Option<NwValue<Decimal>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterCreature {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Appearance")]
    pub appearance: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "CR")]
    pub challenge_rating: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ResRef")]
    pub res_ref: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SingleSpawn")]
    pub single_spawn: Option<NwValue<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterGeometry {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "X")]
    pub x: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Y")]
    pub y: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Z")]
    pub z: Option<NwValue<Decimal>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Placeable {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "AnimationState")]
    pub animation_state: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Appearance")]
    pub appearance: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "AutoRemoveKey")]
    pub auto_remove_key: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Bearing")]
    pub bearing: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "BodyBag")]
    pub body_bag: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "CloseLockDC")]
    pub close_lock_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Conversation")]
    pub conversation: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentHP")]
    pub current_hp: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
    pub description: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "DisarmDC")]
    pub disarm_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Faction")]
    pub faction: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Fort")]
    pub fortitude: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Hardness")]
    pub hardness: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "HasInventory")]
    pub has_inventory: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "HP")]
    pub hp: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Interruptable")]
    pub interuptable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "KeyName")]
    pub key_name: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "KeyRequired")]
    pub key_required: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Lockable")]
    pub lockable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Locked")]
    pub locked: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocName")]
    pub location_name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnClick")]
    pub on_click: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnClosed")]
    pub on_closed: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDamaged")]
    pub on_damaged: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDeath")]
    pub on_death: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDisarm")]
    pub on_disarm: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnHeartbeat")]
    pub on_heartbeat: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnInvDisturbed")]
    pub on_inv_disturbed: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnLock")]
    pub on_lock: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnMeleeAttacked")]
    pub on_melee_attacked: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnOpen")]
    pub on_open: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnSpellCastAt")]
    pub on_spell_cast_at: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnTrapTriggered")]
    pub on_trap_triggered: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUnlock")]
    pub on_unlock: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUsed")]
    pub on_used: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUserDefined")]
    pub on_user_defined: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OpenLockDC")]
    pub open_lock_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Plot")]
    pub plot: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PortraitId")]
    pub portrait_id: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Ref")]
    pub reflex: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Static")]
    pub static_: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDetectable")]
    pub trap_detectable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDetectDC")]
    pub trap_detect_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDisarmable")]
    pub trap_disarmable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapFlag")]
    pub trap_flag: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapOneShot")]
    pub trap_one_shot: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapType")]
    pub trap_type: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
    pub _type: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Useable")]
    pub usable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Will")]
    pub will: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "X")]
    pub x: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Y")]
    pub y: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Z")]
    pub z: Option<NwValue<Decimal>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sound {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trigger {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "AutoRemoveKey")]
    pub auto_remove_key: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Cursor")]
    pub cursor: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "DisarmDC")]
    pub disarm_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Faction")]
    pub faction: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Geometry")]
    pub geometry: Option<NwValue<Vec<TriggerGeometry>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "HighlightHeight")]
    pub highlight_height: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "KeyName")]
    pub key_name: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkedTo")]
    pub linked_to: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkedToFlags")]
    pub linked_to_flags: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LoadScreenID")]
    pub load_screen_id: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocalizedName")]
    pub localized_name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnClick")]
    pub on_click: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDisarm")]
    pub on_disarm: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnTrapTriggered")]
    pub on_trap_triggered: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PortraitId")]
    pub portrait_id: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptHeartbeat")]
    pub script_heartbeat: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptOnEnter")]
    pub script_on_enter: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptOnExit")]
    pub script_on_exit: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptUserDefine")]
    pub script_user_define: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDetectable")]
    pub trap_detectable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDetectDC")]
    pub trap_detect_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDisarmable")]
    pub trap_disarmable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapFlag")]
    pub trap_flag: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapOneShot")]
    pub trap_one_shot: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapType")]
    pub trap_type: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
    pub _type: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "XOrientation")]
    pub x_orientation: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "XPosition")]
    pub x_position: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "YOrientation")]
    pub y_orientation: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "YPosition")]
    pub y_position: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ZOrientation")]
    pub z_orientation: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ZPosition")]
    pub z_position: Option<NwValue<Decimal>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerGeometry {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PointX")]
    pub x: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PointY")]
    pub y: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PointZ")]
    pub z: Option<NwValue<Decimal>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpawnPoint {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Orientation")]
    pub orientation: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "X")]
    pub x: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Y")]
    pub y: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Z")]
    pub z: Option<NwValue<Decimal>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Waypoint {
    #[serde(rename = "__struct_id")]
    pub struct_id: u32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Appearance")]
    pub appearance: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
    pub description: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "HasMapNote")]
    pub has_map_note: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkedTo")]
    pub linked_to: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocalizedName")]
    pub localized_name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MapNote")]
    pub map_note: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MapNoteEnabled")]
    pub map_note_enabled: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "XOrientation")]
    pub x_orientation: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "XPosition")]
    pub x_position: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "YOrientation")]
    pub y_orientation: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "YPosition")]
    pub y_position: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ZPosition")]
    pub z_position: Option<NwValue<Decimal>>,
}
