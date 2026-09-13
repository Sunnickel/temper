use temper_macros::NetEncode;

#[derive(NetEncode)]
pub struct PaintingVariant {
    pub width: i32,
    pub height: i32,
    pub asset_id: String,
}
