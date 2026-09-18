use crate::components::ItemComponent;
use crate::item::ItemID;
use bitcode::__private::{
    Buffer as BitcodeBuffer, Decoder as BitcodeDecoder, Encoder as BitcodeEncoder,
    Result as BitcodeResult, View as BitcodeView,
};
use bitcode::{Decode as BitcodeDecode, Encode as BitcodeEncode};
use bitcode_derive::{Decode, Encode};
use std::fmt::Display;
use std::io::{Cursor, Read, Write};
use std::num::NonZeroUsize;
use temper_codec::decode::errors::NetDecodeError;
use temper_codec::decode::{NetDecode, NetDecodeOpts};
use temper_codec::encode::errors::NetEncodeError;
use temper_codec::encode::{NetEncode, NetEncodeOpts};
use temper_codec::net_types::var_int::VarInt;
use type_hash::TypeHash;

#[derive(Debug, Clone, Default, PartialEq, TypeHash)]
pub struct InventorySlot {
    pub count: VarInt,
    pub item_id: Option<ItemID>,
    pub components_to_add_count: Option<VarInt>,
    pub components_to_remove_count: Option<VarInt>,
    #[type_hash(skip)]
    pub components_to_add: Option<Vec<ItemComponent>>,
    pub components_to_remove: Option<Vec<VarInt>>,
    // https://minecraft.wiki/w/Java_Edition_protocol/Slot_data
}

#[derive(Clone, Default, Decode, Encode)]
struct StoredInventorySlot {
    count: VarInt,
    item_id: Option<ItemID>,
    components_to_add_count: Option<VarInt>,
    components_to_remove_count: Option<VarInt>,
    components_to_add: Option<Vec<Vec<u8>>>,
    components_to_remove: Option<Vec<VarInt>>,
}

impl StoredInventorySlot {
    fn from_slot(slot: &InventorySlot) -> Self {
        Self {
            count: slot.count,
            item_id: slot.item_id,
            components_to_add_count: slot.components_to_add_count,
            components_to_remove_count: slot.components_to_remove_count,
            components_to_add: slot.components_to_add.as_ref().map(|components| {
                components
                    .iter()
                    .map(encode_component_for_storage)
                    .collect()
            }),
            components_to_remove: slot.components_to_remove.clone(),
        }
    }

    fn into_slot(self) -> InventorySlot {
        InventorySlot {
            count: self.count,
            item_id: self.item_id,
            components_to_add_count: self.components_to_add_count,
            components_to_remove_count: self.components_to_remove_count,
            components_to_add: self.components_to_add.map(|components| {
                components
                    .into_iter()
                    .map(decode_component_from_storage)
                    .collect()
            }),
            components_to_remove: self.components_to_remove,
        }
    }
}

#[derive(Default)]
pub struct InventorySlotEncoder {
    inner: <StoredInventorySlot as BitcodeEncode>::Encoder,
}

impl BitcodeBuffer for InventorySlotEncoder {
    fn collect_into(&mut self, out: &mut Vec<u8>) {
        self.inner.collect_into(out);
    }

    fn reserve(&mut self, additional: NonZeroUsize) {
        self.inner.reserve(additional);
    }
}

impl BitcodeEncoder<InventorySlot> for InventorySlotEncoder {
    fn encode(&mut self, slot: &InventorySlot) {
        self.inner.encode(&StoredInventorySlot::from_slot(slot));
    }
}

impl BitcodeEncode for InventorySlot {
    type Encoder = InventorySlotEncoder;
}

#[derive(Default)]
pub struct InventorySlotDecoder<'a> {
    inner: <StoredInventorySlot as BitcodeDecode<'a>>::Decoder,
    slots: Vec<InventorySlot>,
    next_slot: usize,
}

impl<'a> BitcodeView<'a> for InventorySlotDecoder<'a> {
    fn populate(&mut self, input: &mut &'a [u8], length: usize) -> BitcodeResult<()> {
        self.inner.populate(input, length)?;
        self.slots.clear();
        self.slots.reserve(length);
        self.next_slot = 0;

        for _ in 0..length {
            self.slots.push(self.inner.decode().into_slot());
        }

        Ok(())
    }
}

impl<'a> BitcodeDecoder<'a, InventorySlot> for InventorySlotDecoder<'a> {
    fn decode(&mut self) -> InventorySlot {
        let slot = self.slots[self.next_slot].clone();
        self.next_slot += 1;
        slot
    }
}

impl<'a> BitcodeDecode<'a> for InventorySlot {
    type Decoder = InventorySlotDecoder<'a>;
}

fn encode_component_for_storage(component: &ItemComponent) -> Vec<u8> {
    let mut buffer = Vec::new();
    component
        .encode(&mut buffer, &NetEncodeOpts::None)
        .unwrap_or_else(|_| panic!("failed to encode item component for storage"));
    buffer
}

fn decode_component_from_storage(bytes: Vec<u8>) -> ItemComponent {
    let mut reader = Cursor::new(bytes);
    ItemComponent::decode(&mut reader, &NetDecodeOpts::None)
        .unwrap_or_else(|_| panic!("failed to decode item component from storage"))
}

impl InventorySlot {
    pub fn empty() -> Self {
        Self {
            count: VarInt(0),
            item_id: None,
            components_to_add_count: None,
            components_to_add: None,
            components_to_remove: None,
            components_to_remove_count: None,
        }
    }
}

impl Display for InventorySlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "InventorySlot {{ count: {}, item_id: {:?}, components_to_add_count: {:?}, components_to_remove_count: {:?} }}",
            self.count.0,
            self.item_id,
            self.components_to_add_count,
            self.components_to_remove_count
        )
    }
}

impl NetDecode for InventorySlot {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let count = VarInt::decode(reader, opts)?;
        if count.0 == 0 {
            Ok(Self {
                count,
                ..Default::default()
            })
        } else {
            let item_id = VarInt::decode(reader, opts)?;
            let components_to_add_count = VarInt::decode(reader, opts)?;
            let components_to_remove_count = VarInt::decode(reader, opts)?;

            let components_to_add = {
                let mut components = Vec::with_capacity(components_to_add_count.0 as usize);
                for _ in 0..components_to_add_count.0 {
                    components.push(ItemComponent::decode(reader, opts)?);
                }
                Some(components)
            };
            let components_to_remove = {
                let mut components = Vec::with_capacity(components_to_remove_count.0 as usize);
                for _ in 0..components_to_remove_count.0 {
                    components.push(VarInt::decode(reader, opts)?);
                }
                Some(components)
            };

            Ok(Self {
                count,
                item_id: Some(ItemID(item_id)),
                components_to_add_count: Some(components_to_add_count),
                components_to_remove_count: Some(components_to_remove_count),
                components_to_add,
                components_to_remove,
            })
        }
    }
}

impl NetEncode for InventorySlot {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        // 1. Always encode the count
        self.count.encode(writer, opts)?;

        // If count is 0, stop immediately
        if self.count.0 == 0 {
            return Ok(());
        }

        let zero_varint = VarInt::new(0);

        // 2. Encode ItemID
        match &self.item_id {
            Some(item_id) => item_id.0.encode(writer, opts)?,
            None => zero_varint.encode(writer, opts)?,
        }

        // 3. Get add_count and remove_count
        let add_count = self
            .components_to_add_count
            .as_ref()
            .unwrap_or(&zero_varint);
        let remove_count = self
            .components_to_remove_count
            .as_ref()
            .unwrap_or(&zero_varint);

        // 4. Encode components_to_add_count
        add_count.encode(writer, opts)?;

        // 5. Encode components_to_remove_count
        remove_count.encode(writer, opts)?;

        // 6. Encode components_to_add list (if any)
        if add_count.0 > 0
            && let Some(components) = &self.components_to_add
        {
            for component in components {
                component.encode(writer, opts)?;
            }
        }

        // 7. Encode components_to_remove list (if any)
        if remove_count.0 > 0
            && let Some(components) = &self.components_to_remove
        {
            for component in components {
                component.encode(writer, opts)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use temper_codec::encode::NetEncodeOpts;
    use temper_codec::net_types::var_int::VarInt;

    // This helper function runs the encode/decode cycle
    fn run_roundtrip_test(slot_in: &InventorySlot) -> InventorySlot {
        let mut buffer = Vec::new();

        // Create both types of options explicitly.
        let encode_opts = NetEncodeOpts::default();
        let decode_opts = NetDecodeOpts::default();

        // 1. Encode
        slot_in
            .encode(&mut buffer, &encode_opts)
            .expect("Encode failed");

        // 2. Decode
        let mut reader = Cursor::new(&buffer);
        let slot_out = InventorySlot::decode(&mut reader, &decode_opts).expect("Decode failed");

        // 3. Check that all bytes were read
        assert_eq!(
            reader.position() as usize,
            buffer.len(),
            "Decoder did not read the entire buffer"
        );

        slot_out
    }

    #[test]
    fn test_slot_encode_decode_roundtrip() {
        // --- Test Case 1: The Empty Slot ---

        let simple_slot = InventorySlot {
            count: VarInt::new(10),
            item_id: Some(ItemID::new(1)),
            components_to_add_count: Some(VarInt::new(0)),
            components_to_remove_count: Some(VarInt::new(0)),
            components_to_add: Some(vec![]),
            components_to_remove: Some(vec![]),
        };

        let decoded_simple = run_roundtrip_test(&simple_slot);
        assert_eq!(simple_slot, decoded_simple, "Simple slot roundtrip failed");

        // --- Test Case 2: The Full NBT/Component Slot ---
        let complex_slot = InventorySlot {
            count: VarInt::new(1),
            item_id: Some(ItemID::new(872)),
            components_to_add_count: Some(VarInt::new(2)),
            components_to_remove_count: Some(VarInt::new(1)),
            components_to_add: Some(vec![
                ItemComponent::MaxStackSize(VarInt::new(10)),
                ItemComponent::MaxDamage(VarInt::new(11)),
            ]),
            components_to_remove: Some(vec![VarInt::new(20)]),
        };
        let decoded_complex = run_roundtrip_test(&complex_slot);
        assert_eq!(
            complex_slot, decoded_complex,
            "Complex slot roundtrip failed"
        );
    }
}
