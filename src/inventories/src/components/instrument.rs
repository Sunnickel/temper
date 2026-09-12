use super::SoundEvent;
use temper_codec::net_types::id_or_inline::IdOr;
use temper_text::TextComponent;

pub struct Instrument {
    pub sound_event: IdOr<SoundEvent>,
    pub use_duration: f32,
    pub range: f32,
    pub description: TextComponent,
}
