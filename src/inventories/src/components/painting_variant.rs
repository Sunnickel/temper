use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use temper_macros::{NetDecode, NetEncode};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, NetEncode, NetDecode)]
pub struct PaintingVariant {
    pub width: i32,
    pub height: i32,
    pub asset_id: String,
}
