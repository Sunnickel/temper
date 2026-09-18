use bitcode::{Decode, Encode};
use temper_macros::{NetDecode, NetEncode};

#[derive(PartialEq, Debug, Clone, Encode, Decode, NetEncode, NetDecode)]
pub struct PaintingVariant {
    pub width: i32,
    pub height: i32,
    pub asset_id: String,
}
