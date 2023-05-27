//! # Creature
//! Structs for the `utc` file format

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Utc {
    // The appearance type of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Appearance_Type")]
    pub appearance_type: Option<NwValue<i32>>,

    // Indicates whether the creature has a body bag.
    #[serde(skip_serializing_if = "Option::is_none", rename = "BodyBag")]
    pub body_bag: Option<NwValue<u8>>,

    // The charisma attribute of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Cha")]
    pub charisma: Option<NwValue<i32>>,

    // The challenge rating of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ChallengeRating")]
    pub challenge_rating: Option<NwValue<Decimal>>,

    // The list of classes associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ClassList")]
    pub class_list: Option<NwValue<Vec<Class>>>,

    // Comments or additional information about the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,

    // The constitution attribute of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Con")]
    pub constitution: Option<NwValue<i32>>,

    // The conversation associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Conversation")]
    pub conversation: Option<NwValue<String>>,

    // The challenge rating adjustment for the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "CRAdjust")]
    pub cr_adjust: Option<NwValue<i32>>,

    // The current hit points of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentHitPoints")]
    pub current_hit_points: Option<NwValue<i32>>,

    // The decay time for the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "DecayTime")]
    pub decay_time: Option<NwValue<i32>>,

    // The deity associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Deity")]
    pub deity: Option<NwValue<String>>,

    // The description of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
    pub description: Option<NwValue<LocalizedText>>,

    // The dexterity attribute of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Dex")]
    pub dexterity: Option<NwValue<i32>>,

    // Indicates whether the creature is disarmable.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Disarmable")]
    pub disarmable: Option<NwValue<u8>>,

    // The list of equipped items associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Equip_ItemList")]
    pub equip_item_list: Option<NwValue<Vec<EquippedItem>>>,

    // The faction ID associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "FactionID")]
    pub faction_id: Option<NwValue<i32>>,

    // The list of feats associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "FeatList")]
    pub feat_list: Option<NwValue<Vec<Feat>>>,

    // The first name of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "FirstName")]
    pub first_name: Option<NwValue<LocalizedText>>,

    // The fortitude bonus of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fortbonus")]
    pub fortitude_bonus: Option<NwValue<i32>>,

    // The gender of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Gender")]
    pub gender: Option<NwValue<u8>>,

    // The alignment (good/evil) of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "GoodEvil")]
    pub good_evil: Option<NwValue<u8>>,

    // The total hit points of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "HitPoints")]
    pub hit_points: Option<NwValue<i32>>,

    // The intelligence attribute of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Int")]
    pub intelligence: Option<NwValue<i32>>,

    // Indicates whether the creature is interruptable.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Interruptable")]
    pub interruptable: Option<NwValue<u8>>,

    // Indicates whether the creature is immortal.
    #[serde(skip_serializing_if = "Option::is_none", rename = "IsImmortal")]
    pub is_immortal: Option<NwValue<u8>>,

    // Indicates whether the creature is a player character (PC).
    #[serde(skip_serializing_if = "Option::is_none", rename = "IsPC")]
    pub is_pc: Option<NwValue<u8>>,

    // The list of items associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ItemList")]
    pub item_list: Option<NwValue<Vec<Item>>>,

    // The last name of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "LastName")]
    pub last_name: Option<NwValue<LocalizedText>>,

    // The lawful/chaotic alignment of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "LawfulChaotic")]
    pub lawful_chaotic: Option<NwValue<i32>>,

    // Indicates whether the creature is lootable.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Lootable")]
    pub lootable: Option<NwValue<u8>>,

    // The maximum hit points of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "MaxHitPoints")]
    pub max_hit_points: Option<NwValue<i32>>,

    // The natural armor class of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "NaturalAC")]
    pub natural_ac: Option<NwValue<i32>>,

    // Indicates whether the creature is not subject to permanent death.
    #[serde(skip_serializing_if = "Option::is_none", rename = "NoPermDeath")]
    pub no_perm_death: Option<NwValue<u8>>,

    // The palette ID associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "PaletteID")]
    pub palette_id: Option<NwValue<i32>>,

    // The perception range of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "PerceptionRange")]
    pub perception_range: Option<NwValue<i32>>,

    // The phenotype ID associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Phenotype")]
    pub phenotype: Option<NwValue<i32>>,

    // Indicates whether the creature is part of the plot.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Plot")]
    pub plot: Option<NwValue<u8>>,

    // The portrait ID associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "PortraitId")]
    pub portrait_id: Option<NwValue<i32>>,

    // The race ID associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Race")]
    pub race: Option<NwValue<i32>>,

    // The reflex bonus of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "refbonus")]
    pub reflex_bonus: Option<NwValue<i32>>,

    // The script to be executed when the creature is attacked.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptAttacked")]
    pub script_attacked: Option<NwValue<String>>,

    // The script to be executed when the creature is damaged.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptDamaged")]
    pub script_damaged: Option<NwValue<String>>,

    // The script to be executed when the creature dies.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptDeath")]
    pub script_death: Option<NwValue<String>>,

    // The script to be executed during dialogues with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptDialogue")]
    pub script_dialogue: Option<NwValue<String>>,

    // The script to be executed when the creature is disturbed.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptDisturbed")]
    pub script_disturbed: Option<NwValue<String>>,

    // The script to be executed at the end of each round for the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptEndRound")]
    pub script_end_round: Option<NwValue<String>>,

    // The script to be executed during the creature's heartbeat event.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptHeartbeat")]
    pub script_heartbeat: Option<NwValue<String>>,

    // The script to be executed when the creature is blocked.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptOnBlocked")]
    pub script_on_blocked: Option<NwValue<String>>,

    // The script to be executed when the creature notices something.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptOnNotice")]
    pub script_on_notice: Option<NwValue<String>>,

    // The script to be executed when the creature rests.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptRested")]
    pub script_rested: Option<NwValue<String>>,

    // The script to be executed when the creature spawns.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptSpawn")]
    pub script_spawn: Option<NwValue<String>>,

    // The script to be executed when a spell is cast at the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptSpellAt")]
    pub script_spell_at: Option<NwValue<String>>,

    // The user-defined script associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptUserDefine")]
    pub script_user_define: Option<NwValue<String>>,

    // The list of skills associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "SkillList")]
    pub skill_list: Option<NwValue<Vec<Skill>>>,

    // The sound set file associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "SoundSetFile")]
    pub sound_set_file: Option<NwValue<i32>>,

    // The list of special abilities associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "SpecAbilityList")]
    pub special_ability_list: Option<NwValue<Vec<SpecialAbility>>>,

    // The ID of the starting package associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "StartingPackage")]
    pub starting_package: Option<NwValue<i32>>,

    // The strength attribute value of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Str")]
    pub strength: Option<NwValue<i32>>,

    // The subrace associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Subrace")]
    pub subrace: Option<NwValue<String>>,

    // The tag associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,

    // The new tail ID associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tail_New")]
    pub tail_new: Option<NwValue<i32>>,

    // The list of templates associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateList")]
    pub template_list: Option<NwValue<Vec<Template>>>,

    // The template resource reference associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,

    // The walking rate of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "WalkRate")]
    pub walk_rate: Option<NwValue<i32>>,

    // The willpower bonus of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "willbonus")]
    pub will_bonus: Option<NwValue<i32>>,

    // The new wings ID associated with the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Wings_New")]
    pub wings_new: Option<NwValue<i32>>,

    // The wisdom attribute value of the creature.
    #[serde(skip_serializing_if = "Option::is_none", rename = "Wis")]
    pub wisdom: Option<NwValue<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    /// Identifier for the struct
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// Class value
    #[serde(skip_serializing_if = "Option::is_none", rename = "Class")]
    pub class: Option<NwValue<i32>>,

    /// Level value
    #[serde(skip_serializing_if = "Option::is_none", rename = "ClassLevel")]
    pub level: Option<NwValue<i32>>,

    /// Memorized spells for level 0
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList0")]
    pub memorized_list0: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 1
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList1")]
    pub memorized_list1: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 2
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList2")]
    pub memorized_list2: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 3
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList3")]
    pub memorized_list3: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 4
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList4")]
    pub memorized_list4: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 5
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList5")]
    pub memorized_list5: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 6
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList6")]
    pub memorized_list6: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 7
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList7")]
    pub memorized_list7: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 8
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList8")]
    pub memorized_list8: Option<NwValue<Vec<MemorizedSpell>>>,

    /// Memorized spells for level 9
    #[serde(skip_serializing_if = "Option::is_none", rename = "MemorizedList9")]
    pub memorized_list9: Option<NwValue<Vec<MemorizedSpell>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemorizedSpell {
    /// Identifier for the struct
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// Spell value
    #[serde(skip_serializing_if = "Option::is_none", rename = "Spell")]
    pub spell: Option<NwValue<i32>>,

    /// Spell flags
    #[serde(skip_serializing_if = "Option::is_none", rename = "SpellFlags")]
    pub flags: Option<NwValue<i32>>,

    /// Spell meta-magic value
    #[serde(skip_serializing_if = "Option::is_none", rename = "SpellMetaMagic")]
    pub meta_magic: Option<NwValue<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    /// Identifier for the struct
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// Resource reference for the inventory item
    #[serde(skip_serializing_if = "Option::is_none", rename = "InventoryRes")]
    pub res_ref: Option<NwValue<String>>,

    /// X-coordinate for the position in the repository
    #[serde(skip_serializing_if = "Option::is_none", rename = "Repos_PosX")]
    pub repos_pos_x: Option<NwValue<i32>>,

    /// Y-coordinate for the position in the repository
    #[serde(skip_serializing_if = "Option::is_none", rename = "Repos_Posy")]
    pub repos_pos_y: Option<NwValue<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquippedItem {
    /// Identifier for the struct
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// Resource reference for the equipped item
    #[serde(skip_serializing_if = "Option::is_none", rename = "EquippedRes")]
    pub res_ref: Option<NwValue<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feat {
    /// Identifier for the struct
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// Feat value
    #[serde(skip_serializing_if = "Option::is_none", rename = "Feat")]
    pub feat: Option<NwValue<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    /// Identifier for the struct
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// Skill rank value
    #[serde(skip_serializing_if = "Option::is_none", rename = "Rank")]
    pub rank: Option<NwValue<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialAbility {
    /// Identifier for the struct
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,

    /// Spell value
    #[serde(skip_serializing_if = "Option::is_none", rename = "Spell")]
    pub spell: Option<NwValue<i32>>,

    /// Spell caster level value
    #[serde(skip_serializing_if = "Option::is_none", rename = "SpellCasterLevel")]
    pub caster_level: Option<NwValue<i32>>,

    /// Spell flags value
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
