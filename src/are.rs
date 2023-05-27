//! # Area
//! Structs for the `are` file format

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::*;

/// Area struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    /// Field representing the chance of lightning in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "ChanceLightning")]
    pub chance_lightning: Option<NwValue<i32>>,
    /// Field representing the chance of rain in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "ChanceRain")]
    pub chance_rain: Option<NwValue<i32>>,
    /// Field representing the chance of snow in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "ChanceSnow")]
    pub chance_snow: Option<NwValue<i32>>,
    /// Field representing comments about the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comments")]
    pub comments: Option<NwValue<String>>,
    /// Field representing the ID of the creator of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "Creator_ID")]
    pub creator_id: Option<NwValue<i32>>,
    /// Field representing the day-night cycle value of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "DayNightCycle")]
    pub day_night_cycle: Option<NwValue<u8>>,
    /// Field representing the expansion list of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "Expansion_List")]
    pub expansion_list: Option<NwValue<Vec<String>>>,
    /// Field representing the flags of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "Flags")]
    pub flags: Option<NwValue<u32>>,
    /// Field representing the fog clip distance of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "FogClipDist")]
    pub fog_clip_dist: Option<NwValue<Decimal>>,
    /// Field representing the height of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "Height")]
    pub height: Option<NwValue<i32>>,
    /// Field representing the ID of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "ID")]
    pub id: Option<NwValue<i32>>,
    /// Field representing whether the area is night
    #[serde(skip_serializing_if = "Option::is_none", rename = "IsNight")]
    pub is_night: Option<NwValue<u8>>,
    /// Field representing the lighting scheme of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "LightingScheme")]
    pub lighting_scheme: Option<NwValue<u8>>,
    /// Field representing the load screen ID of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "LoadScreenID")]
    pub load_screen_id: Option<NwValue<u16>>,
    /// Field representing the mod listen check of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "ModListenCheck")]
    pub mod_listen_check: Option<NwValue<i32>>,
    /// Field representing the mod spot check of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "ModSpotCheck")]
    pub mod_spot_check: Option<NwValue<i32>>,

    /// Field representing the ambient color of the moon in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "MoonAmbientColor")]
    pub moon_ambient_color: Option<NwValue<u32>>,
    /// Field representing the diffuse color of the moon in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "MoonDiffuseColor")]
    pub moon_diffuse_color: Option<NwValue<u32>>,
    /// Field representing the fog amount of the moon in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "MoonFogAmount")]
    pub moon_fog_amount: Option<NwValue<u8>>,
    /// Field representing the fog color of the moon in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "MoonFogColor")]
    pub moon_fog_color: Option<NwValue<u32>>,
    /// Field representing the shadow behavior of the moon in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "MoonShadows")]
    pub moon_shadows: Option<NwValue<u8>>,
    /// Field representing the name of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
    pub name: Option<NwValue<super::LocalizedText>>,
    /// Field representing whether resting is allowed in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "NoRest")]
    pub no_rest: Option<NwValue<u8>>,
    /// Field representing the script executed when entering the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnEnter")]
    pub on_enter: Option<NwValue<String>>,
    /// Field representing the script executed when exiting the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnExit")]
    pub on_exit: Option<NwValue<String>>,
    /// Field representing the script executed on area heartbeat
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnHeartbeat")]
    pub on_heartbeat: Option<NwValue<String>>,
    /// Field representing the user-defined script executed in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUserDefined")]
    pub on_user_defined: Option<NwValue<String>>,
    /// Field representing the player versus player setting of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "PlayerVsPlayer")]
    pub player_vs_player: Option<NwValue<u8>>,
    /// Field representing the reference resource of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "ResRef")]
    pub res_ref: Option<NwValue<String>>,
    /// Field representing the opacity of shadows in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "ShadowOpacity")]
    pub shadow_opacity: Option<NwValue<u8>>,
    /// Field representing the skybox setting of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "SkyBox")]
    pub sky_box: Option<NwValue<u8>>,

    /// Field representing the ambient color of the sun in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "SunAmbientColor")]
    pub sun_ambient_color: Option<NwValue<u32>>,
    /// Field representing the diffuse color of the sun in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "SunDiffuseColor")]
    pub sun_diffuse_color: Option<NwValue<u32>>,
    /// Field representing the fog amount of the sun in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "SunFogAmount")]
    pub sun_fog_amount: Option<NwValue<u8>>,
    /// Field representing the fog color of the sun in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "SunFogColor")]
    pub sun_fog_color: Option<NwValue<u32>>,
    /// Field representing the shadow behavior of the sun in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "SunShadows")]
    pub sun_shadows: Option<NwValue<u8>>,
    /// Field representing the tag of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
    /// Field representing the list of tiles in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_List")]
    pub tile_list: Option<NwValue<Vec<Tile>>>,
    /// Field representing the tileset used in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tileset")]
    pub tileset: Option<NwValue<String>>,
    /// Field representing the width of the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "Width")]
    pub width: Option<NwValue<i32>>,
    /// Field representing the wind power in the area
    #[serde(skip_serializing_if = "Option::is_none", rename = "WindPower")]
    pub wind_power: Option<NwValue<i32>>,
}

/// Area Tile struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    /// Field representing the struct ID of the tile
    #[serde(rename = "__struct_id")]
    pub struct_id: u8,
    /// Field representing the first tile animation loop
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_AnimLoop1")]
    pub tile_anim_loop1: Option<NwValue<u8>>,
    /// Field representing the second tile animation loop
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_AnimLoop2")]
    pub tile_anim_loop2: Option<NwValue<u8>>,
    /// Field representing the third tile animation loop
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_AnimLoop3")]
    pub tile_anim_loop3: Option<NwValue<u8>>,
    /// Field representing the height of the tile
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_Height")]
    pub tile_height: Option<NwValue<i32>>,
    /// Field representing the ID of the tile
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_ID")]
    pub tile_id: Option<NwValue<i32>>,
    /// Field representing the first main light of the tile
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_MainLight1")]
    pub tile_main_light1: Option<NwValue<u8>>,
    /// Field representing the second main light of the tile
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_MainLight2")]
    pub tile_main_light2: Option<NwValue<u8>>,
    /// Field representing the orientation of the tile
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_Orientation")]
    pub tile_orientation: Option<NwValue<i32>>,
    /// Field representing the first source light of the tile
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_SrcLight1")]
    pub tile_src_light1: Option<NwValue<u8>>,
    /// Field representing the second source light of the tile
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_SrcLight2")]
    pub tile_src_light2: Option<NwValue<u8>>,
}
