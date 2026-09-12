use super::SoundEvent;
use temper_codec::net_types::id_or_inline::IdOr;
use temper_codec::net_types::var_int::VarInt;
use temper_text::TextComponent;

pub struct JukeboxSong {
    pub sound_event: IdOr<SoundEvent>,
    pub description: TextComponent,
    pub duration: f32,
    pub output: VarInt,
}
