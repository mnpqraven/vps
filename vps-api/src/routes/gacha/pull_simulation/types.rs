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
    pub pity_current_count: i32,
    /// total amount of pulls in possession
    pub pulls: i32,
    /// if your next targeted pull is really the correct target (you already
    /// failed pity)
    pub next_guaranteed: bool,
    /// the banner rules
    pub banner: Banner,
}

// master struct
#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct ProbabilityRateResponse {
    pub roll_budget: i32,
    pub data: Vec<Vec<ReducedSim>>,
    pub banner: Banner,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub banner_name: String,
    /// base pity rate. hitting this rate ends the calc, failing this rate
    /// resets the calc and enables `guaranteed_pity`
    /// E.g:
    /// 0.5 for hoyo char, 0.75 for hoyo weapons
    /// 1.0 is a guaranteed, you get what you want
    pub banner: f64,
    /// base rate chance of the gacha
    pub base_rate: f64,
    /// soft pity threshold of the banner
    pub pity_soft_threshold: i32,
    pub rarity: u32,
    /// chance to get specifically the unit you want
    /// E.g star rail banner with 3 SR means 0.33
    pub guaranteed: f64,
    /// https://www.hoyolab.com/article/533196
    #[serde(skip)]
    pub enpitomized_pity: Option<i32>,
    pub const_prefix: String,
    pub const_shorthand: char,
    pub min_const: i32,
    pub max_const: i32,
    pub max_pity: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
#[repr(i32)]
pub enum BannerType {
    #[serde(rename = "SSR")]
    HsrSsr = 0,
    #[serde(rename = "SR")]
    HsrSr = 1,
    #[serde(rename = "LC")]
    HsrLc = 2,
    #[serde(rename = "Custom")]
    Custom = 3,
}

impl Banner {
    pub fn rate_fn(&self) -> Box<dyn Fn(i32) -> f64> {
        let (base_rate, pity_start) = (self.base_rate, self.pity_soft_threshold);
        let func = move |pity: i32| match pity < pity_start {
            true => base_rate,
            false => base_rate + base_rate * 10.0 * (pity - pity_start + 1) as f64,
        };
        Box::new(func)
    }

    #[deprecated]
    fn _char_ssr() -> Self {
        Self {
            banner_name: "5* Character Banner".into(),
            banner: 0.5,
            base_rate: 0.6,
            pity_soft_threshold: 74,
            rarity: 5,
            guaranteed: 1.0,
            enpitomized_pity: None,
            min_const: -1,
            max_const: 6,
            max_pity: 90,
            const_prefix: BannerType::HsrSsr.const_prefix(),
            const_shorthand: 'E',
        }
    }
    #[deprecated]
    fn _char_sr() -> Self {
        Self {
            banner_name: "Specific 4* Character Banner".into(),
            banner: 0.5,
            base_rate: 0.7,
            // uhh do we need this when max_pity exists
            pity_soft_threshold: 63,
            rarity: 4,
            guaranteed: 0.333333333,
            enpitomized_pity: None,
            min_const: -1,
            max_const: 6,
            max_pity: 10,
            const_prefix: BannerType::HsrSr.const_prefix(),
            const_shorthand: 'E',
        }
    }
    #[deprecated]
    fn _basic_weapon() -> Self {
        Self {
            banner_name: "5* Light Cone".into(),
            banner: 0.75,
            base_rate: 5.1,
            pity_soft_threshold: 9,
            rarity: 5,
            guaranteed: 1.0,
            enpitomized_pity: None,
            min_const: -1,
            max_const: 4,
            max_pity: 80,
            const_prefix: BannerType::HsrLc.const_prefix(),
            const_shorthand: 'S',
        }
    }
    #[deprecated]
    fn _dev_weapon() -> Self {
        Self {
            banner_name: "5* Light Cone Banner".into(),
            banner: 0.75,
            base_rate: 5.1,
            pity_soft_threshold: 9,
            rarity: 5,
            guaranteed: 0.5,
            enpitomized_pity: Some(3),
            min_const: 0,
            max_const: 5,
            max_pity: 80,
            const_prefix: BannerType::HsrLc.const_prefix(),
            const_shorthand: 'S',
        }
    }
}

impl BannerType {
    pub fn const_prefix(&self) -> String {
        match self {
            BannerType::HsrSsr => "Eidolon".into(),
            BannerType::HsrSr => "Eidolon".into(),
            BannerType::HsrLc => "Superimpose".into(),
            BannerType::Custom => "Dupe".into(),
        }
    }
}
