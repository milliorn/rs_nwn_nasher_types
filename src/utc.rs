//! # Creature
//! Structs for the `utc` file format

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Utc {
    #[serde(skip_serializing_if = "Option::is_none", rename = "Appearance_Type")]
    pub appearance_type: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "BodyBag")]
    pub body_bag: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Cha")]
    pub charisma: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ChallengeRating")]
    pub challenge_rating: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ClassList")]
    pub class_list: Option<NwValue<Vec<Class>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Con")]
    pub constitution: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Conversation")]
    pub conversation: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "CRAdjust")]
    pub cr_adjust: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentHitPoints")]
    pub current_hit_points: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "DecayTime")]
    pub decay_time: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Deity")]
    pub deity: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
    pub description: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Dex")]
    pub dexterity: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Disarmable")]
    pub disarmable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Equip_ItemList")]
    pub equip_item_list: Option<NwValue<Vec<EquippedItem>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FactionID")]
    pub faction_id: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FeatList")]
    pub feat_list: Option<NwValue<Vec<Feat>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FirstName")]
    pub first_name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fortbonus")]
    pub fortitude_bonus: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Gender")]
    pub gender: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "GoodEvil")]
    pub good_evil: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "HitPoints")]
    pub hit_points: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Int")]
    pub intelligence: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Interruptable")]
    pub interruptable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "IsImmortal")]
    pub is_immortal: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "IsPC")]
    pub is_pc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ItemList")]
    pub item_list: Option<NwValue<Vec<Item>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LastName")]
    pub last_name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LawfulChaotic")]
    pub lawful_chaotic: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Lootable")]
    pub lootable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MaxHitPoints")]
    pub max_hit_points: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "NaturalAC")]
    pub natural_ac: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "NoPermDeath")]
    pub no_perm_death: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PaletteID")]
    pub palette_id: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PerceptionRange")]
    pub perception_range: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Phenotype")]
    pub phenotype: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Plot")]
    pub plot: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PortraitId")]
    pub portrait_id: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Race")]
    pub race: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "refbonus")]
    pub reflex_bonus: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptAttacked")]
    pub script_attacked: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptDamaged")]
    pub script_damaged: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptDeath")]
    pub script_death: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptDialogue")]
    pub script_dialogue: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptDisturbed")]
    pub script_disturbed: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptEndRound")]
    pub script_end_round: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptHeartbeat")]
    pub script_heartbeat: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptOnBlocked")]
    pub script_on_blocked: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptOnNotice")]
    pub script_on_notice: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptRested")]
    pub script_rested: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptSpawn")]
    pub script_spawn: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptSpellAt")]
    pub script_spell_at: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptUserDefine")]
    pub script_user_define: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SkillList")]
    pub skill_list: Option<NwValue<Vec<Skill>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SoundSetFile")]
    pub sound_set_file: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SpecAbilityList")]
    pub special_ability_list: Option<NwValue<Vec<SpecialAbility>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "StartingPackage")]
    pub starting_package: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Str")]
    pub strength: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Subrace")]
    pub subrace: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tail_New")]
    pub tail_new: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateList")]
    pub template_list: Option<NwValue<Vec<Template>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "WalkRate")]
    pub walk_rate: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "willbonus")]
    pub will_bonus: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Wings_New")]
    pub wings_new: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Wis")]
    pub wisdom: Option<NwValue<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    /// Identifier for the struct (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// Class value (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "Class")]
    pub class: Option<NwValue<i32>>,

    /// Level value (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "ClassLevel")]
    pub level: Option<NwValue<i32>>,

    /// Memorized spells for level 0 (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList0")]
    pub memorized_list0: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 1 (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList1")]
    pub memorized_list1: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 2 (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList2")]
    pub memorized_list2: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 3 (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList3")]
    pub memorized_list3: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 4 (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList4")]
    pub memorized_list4: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 5 (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList5")]
    pub memorized_list5: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 6 (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList6")]
    pub memorized_list6: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 7 (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList7")]
    pub memorized_list7: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 8 (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList8")]
    pub memorized_list8: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 9 (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList9")]
    pub memorized_list9: Option<NwValue<Vec<MemorizedSpell>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemorizedSpell {
    /// Identifier for the struct (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// Spell value (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "Spell")]
    pub spell: Option<NwValue<i32>>,

    /// Spell flags (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "SpellFlags")]
    pub flags: Option<NwValue<i32>>,

    /// Spell meta-magic value (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "SpellMetaMagic")]
    pub meta_magic: Option<NwValue<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    /// Identifier for the struct (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// Resource reference for the inventory item (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "InventoryRes")]
    pub res_ref: Option<NwValue<String>>,

    /// X-coordinate for the position in the repository (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "Repos_PosX")]
    pub repos_pos_x: Option<NwValue<i32>>,

    /// Y-coordinate for the position in the repository (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "Repos_Posy")]
    pub repos_pos_y: Option<NwValue<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquippedItem {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "EquippedRes")]
    pub res_ref: Option<NwValue<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feat {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Feat")]
    pub feat: Option<NwValue<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Rank")]
    pub rank: Option<NwValue<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialAbility {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Spell")]
    pub spell: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SpellCasterLevel")]
    pub caster_level: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SpellFlags")]
    pub flags: Option<NwValue<i32>>,
}

/// Represents a template for a specific purpose.
#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    /// Optional struct ID
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
}
