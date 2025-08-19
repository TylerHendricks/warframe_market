use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Full item model with all possible fields.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub tags: Option<Vec<String>>,
    pub set_root: Option<bool>,
    pub set_parts: Option<Vec<String>>,
    pub quantity_in_set: Option<i32>,
    pub rarity: Option<String>,
    pub bulk_tradable: Option<bool>,
    pub subtypes: Option<Vec<String>>,
    pub max_rank: Option<i32>,
    pub max_charges: Option<i32>,
    pub max_amber_stars: Option<i32>,
    pub max_cyan_stars: Option<i32>,
    pub base_endo: Option<i32>,
    pub endo_multiplier: Option<f32>,
    pub ducats: Option<i32>,
    pub vosfor: Option<i32>,
    pub req_mastery_rank: Option<i32>,
    pub vaulted: Option<bool>,
    pub trading_tax: Option<i32>,
    pub tradable: Option<bool>,
    pub i18n: Option<HashMap<String, ItemI18N>>,
}

/// Represent trimmed Item model, only used to build initial local copy of
/// tradable items list on a client.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemShort {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub tags: Option<Vec<String>>,
    pub bulk_tradable: Option<bool>,
    pub subtypes: Option<Vec<String>>,
    pub max_rank: Option<i32>,
    pub max_amber_stars: Option<i32>,
    pub max_cyan_stars: Option<i32>,
    pub base_endo: Option<i32>,
    pub endo_multiplier: Option<f32>,
    pub ducats: Option<i32>,
    pub vaulted: Option<bool>,
    pub i18n: Option<HashMap<String, ItemShortI18N>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemI18N {
    pub name: String,
    pub description: Option<String>,
    pub wiki_link: Option<String>,
    pub icon: String,
    pub thumb: String,
    pub sub_icon: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemShortI18N {
    pub name: String,
    pub icon: String,
    pub sub_icon: Option<String>,
}
