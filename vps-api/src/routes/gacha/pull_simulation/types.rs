use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone)]
pub struct Sim {
    pub eidolon: i32,
    pub rate: f64,
    pub pity: i32,
    pub guaranteed: bool,
    pub guaranteed_pity: i32,
}
#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct ReducedSim {
    pub eidolon: i32,
    pub rate: f64,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ProbabilityRatePayload {
    /// current dupe count
    pub current_eidolon: i32,
    /// current pity count, hitting the targeted pull sets this back to 00
    pub pity: i32,
    /// total amount of pulls in possession
    pub pulls: i32,
    /// if your next targeted pull is really the correct target (you already
    /// failed pity)
    pub next_guaranteed: bool,
    /// DEV
    #[param(ignore)]
    #[serde(skip)]
    pub _enpitomized_pity: Option<i32>,
    /// banner type that has some built in parameters
    pub banner: BannerType,
}

// master struct
#[derive(Debug, Serialize, Clone, Default, ToSchema)]
pub struct ProbabilityRateResponse {
    pub roll_budget: i32,
    pub data: Vec<Vec<ReducedSim>>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub banner_name: String,
    /// base pity rate. hitting this rate ends the calc, failing this rate
    /// resets the calc and enables `guaranteed_pity`
    /// E.g:
    /// 0.5 for hoyo char, 0.75 for hoyo weapons
    /// 1.0 is a guaranteed, you get what you want
    pub banner: f64,
    pub rarity: u32,
    /// the rate
    pub guaranteed: f64,
    /// https://www.hoyolab.com/article/533196
    pub enpitomized_pity: Option<i32>,
    pub const_prefix: String,
    pub const_shorthand: char,
    pub min_const: i32,
    pub max_const: i32,
    pub max_pity: i32,
    pub banner_type: BannerType,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
#[repr(i32)]
pub enum BannerType {
    #[serde(rename = "SSR")]
    Ssr = 0,
    #[serde(rename = "SR")]
    Sr = 1,
    #[serde(rename = "LC")]
    Lc = 2,
}

impl Banner {
    pub fn char_ssr() -> Self {
        Self {
            banner_name: "5* Character Banner".into(),
            banner: 0.5,
            rarity: 5,
            guaranteed: 1.0,
            enpitomized_pity: None,
            min_const: -1,
            max_const: 6,
            max_pity: 90,
            const_prefix: BannerType::Ssr.const_prefix(),
            const_shorthand: 'E',
            banner_type: BannerType::Ssr,
        }
    }
    pub fn char_sr() -> Self {
        Self {
            banner_name: "Specific 4* Character Banner".into(),
            banner: 0.5,
            rarity: 4,
            guaranteed: 0.333333333,
            enpitomized_pity: None,
            min_const: -1,
            max_const: 6,
            max_pity: 10,
            const_prefix: BannerType::Sr.const_prefix(),
            const_shorthand: 'E',
            banner_type: BannerType::Sr,
        }
    }
    pub fn basic_weapon() -> Self {
        Self {
            banner_name: "5* Light Cone".into(),
            banner: 0.75,
            rarity: 5,
            guaranteed: 1.0,
            enpitomized_pity: None,
            min_const: -1,
            max_const: 4,
            max_pity: 80,
            const_prefix: BannerType::Lc.const_prefix(),
            const_shorthand: 'S',
            banner_type: BannerType::Lc,
        }
    }

    pub fn dev_weapon() -> Self {
        Self {
            banner_name: "5* Light Cone Banner".into(),
            banner: 0.75,
            rarity: 5,
            guaranteed: 0.5,
            enpitomized_pity: Some(3),
            min_const: 0,
            max_const: 5,
            max_pity: 80,
            const_prefix: BannerType::Lc.const_prefix(),
            const_shorthand: 'S',
            banner_type: BannerType::Lc,
        }
    }

    pub fn to_internal(&self, pity_rate_fn: Box<dyn Fn(i32) -> f64>) -> BannerIternal {
        BannerIternal {
            banner_name: self.banner_name.to_owned(),
            banner: self.banner,
            guaranteed: self.guaranteed,
            enpitomized_pity: self.enpitomized_pity,
            min_const: self.min_const,
            max_const: self.max_const,
            max_pity: self.max_pity,
            rate: pity_rate_fn,
        }
    }
}

/// struct that is used in the backend
pub struct BannerIternal {
    pub banner_name: String,
    /// rate of the featured ssr (0.5 for character, 0.75 for LC)
    pub banner: f64,
    pub guaranteed: f64,
    /// not yet implemented, genshin epitomized path ???
    pub enpitomized_pity: Option<i32>,
    pub min_const: i32,
    pub max_const: i32,
    /// pity count (90 for char, 80 lc)
    pub max_pity: i32,
    // constFormat: string
    // constName: string
    pub rate: Box<dyn Fn(i32) -> f64>, // (pity: number) => number
}

impl BannerType {
    pub fn const_prefix(&self) -> String {
        match self {
            BannerType::Ssr => "Eidolon".into(),
            BannerType::Sr => "Eidolon".into(),
            BannerType::Lc => "Superimpose".into(),
        }
    }
}
