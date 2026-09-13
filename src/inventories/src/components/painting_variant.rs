use temper_macros::{NetDecode, NetEncode};

#[derive(NetEncode, NetDecode)]
pub struct PaintingVariant {
    pub width: i32,
    pub height: i32,
    pub asset_id: String,
}
