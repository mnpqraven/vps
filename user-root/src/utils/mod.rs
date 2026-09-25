use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

pub mod error;
pub mod hooks;
pub mod pagination;
pub mod router;

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
pub enum FormMode {
    Create,
    Update,
}
