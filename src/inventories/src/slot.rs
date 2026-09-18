use crate::components::ItemComponent;
use crate::item::ItemID;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::{Read, Write};
use temper_codec::decode::errors::NetDecodeError;
use temper_codec::decode::{NetDecode, NetDecodeOpts};
use temper_codec::encode::errors::NetEncodeError;
use temper_codec::encode::{NetEncode, NetEncodeOpts};
use temper_codec::net_types::var_int::VarInt;
use type_hash::TypeHash;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, TypeHash)]
pub struct InventorySlot {
    pub count: VarInt,
    pub item_id: Option<ItemID>,
    #[type_hash(skip)]
    pub components_to_add: Option<Vec<ItemComponent>>,
    #[type_hash(skip)]
    pub components_to_remove: Option<Vec<ItemComponent>>,
    // https://minecraft.wiki/w/Java_Edition_protocol/Slot_data
}

impl InventorySlot {
    pub fn empty() -> Self {
        Self {
            count: VarInt(0),
            item_id: None,
            components_to_add: None,
            components_to_remove: None,
        }
    }
}

impl Display for InventorySlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "InventorySlot {{ count: {}, item_id: {:?}, components_to_add: {:?}, components_to_remove: {:?} }}",
            self.count.0, self.item_id, self.components_to_add, self.components_to_remove,
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
                    components.push(ItemComponent::decode(reader, opts)?);
                }
                Some(components)
            };

            Ok(Self {
                count,
                item_id: Some(ItemID(item_id)),
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

        // 2. Encode ItemID
        match &self.item_id {
            Some(item_id) => item_id.0.encode(writer, opts)?,
            None => VarInt::new(0).encode(writer, opts)?,
        }

        // 3. Get add_count and remove_count
        let add_count = self
            .components_to_add
            .as_ref()
            .map(|v| VarInt::from(v.len() as i32))
            .unwrap_or_default();
        let remove_count = self
            .components_to_remove
            .as_ref()
            .map(|v| VarInt::from(v.len() as i32))
            .unwrap_or_default();

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
            components_to_add: Some(vec![]),
            components_to_remove: Some(vec![]),
        };

        let decoded_simple = run_roundtrip_test(&simple_slot);
        assert_eq!(simple_slot, decoded_simple, "Simple slot roundtrip failed");

        // --- Test Case 2: The Full NBT/Component Slot ---
        let complex_slot = InventorySlot {
            count: VarInt::new(1),
            item_id: Some(ItemID::new(872)),
            components_to_add: Some(vec![
                ItemComponent::MaxStackSize(VarInt::new(10)),
                ItemComponent::MaxDamage(VarInt::new(11)),
            ]),
            components_to_remove: Some(vec![
                ItemComponent::MaxStackSize(VarInt::new(20)),
                ItemComponent::MaxDamage(VarInt::new(21)),
            ]),
        };
        let decoded_complex = run_roundtrip_test(&complex_slot);
        assert_eq!(
            complex_slot, decoded_complex,
            "Complex slot roundtrip failed"
        );
    }
}
