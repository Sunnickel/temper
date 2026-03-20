use crate::BlockStateId;
use crate::errors::BlockPlaceError;
use crate::{BlockPlaceContext, PlacableBlock, PlacedBlocks};
use bevy_math::IVec3;
use std::collections::{BTreeMap, HashMap};
use temper_core::block_data::BlockData;
use temper_core::dimension::Dimension;
use temper_macros::match_block;
use temper_state::GlobalState;

pub(crate) struct PlaceableFence;

impl PlacableBlock for PlaceableFence {
    fn place(
        context: BlockPlaceContext,
        state: GlobalState,
    ) -> Result<PlacedBlocks, BlockPlaceError> {
        let name = match context.item_used.to_name() {
            Some(name) => name,
            None => return Err(BlockPlaceError::ItemIdHasNoNameMapping(context.item_used)),
        };
        let target_block = {
            let chunk = state
                .world
                .get_or_generate_chunk(context.block_position.chunk(), Dimension::Overworld)
                .expect("Could not load chunk");
            chunk.get_block(context.block_position.chunk_block_pos())
        };
        if !match_block!("air", target_block) && !match_block!("cave_air", target_block) {
            return Err(BlockPlaceError::TargetBlockNotEmpty(context.block_position));
        }

        let mut props = BTreeMap::from([
            ("east".to_string(), "false".to_string()),
            ("west".to_string(), "false".to_string()),
            ("north".to_string(), "false".to_string()),
            ("south".to_string(), "false".to_string()),
            ("waterlogged".to_string(), "false".to_string()),
        ]);

        let adjacent_positions = [
            (context.block_position + IVec3::new(1, 0, 0).into(), "east"),
            (context.block_position + IVec3::new(-1, 0, 0).into(), "west"),
            (context.block_position + IVec3::new(0, 0, 1).into(), "south"),
            (
                context.block_position + IVec3::new(0, 0, -1).into(),
                "north",
            ),
        ];

        let mut changed_blocks = HashMap::new();

        for (pos, direction) in adjacent_positions {
            let block_id = {
                let chunk = state
                    .world
                    .get_or_generate_chunk(pos.chunk(), Dimension::Overworld)
                    .expect("Could not load chunk");
                chunk.get_block(pos.chunk_block_pos())
            };
            let block_data = block_id.to_block_data().unwrap_or_else(|| {
                panic!("Block ID {} not found in block mappings file", block_id)
            });
            let block_name = block_data
                .name
                .strip_prefix("minecraft:")
                .unwrap_or(&block_data.name);
            if block_name.ends_with("fence")
                || block_name.ends_with("wall")
                || block_name.ends_with("fence_gate")
            {
                props.insert(direction.to_string(), "true".to_string());
                // Update the adjacent block to connect to the new fence
                // TODO: This should be moved to a proper block update system that updates all blocks around a changed block, but for now this will do
                let mut adjacent_props = block_data.properties.unwrap_or_default();
                let opposite_direction = match direction {
                    "east" => "west",
                    "west" => "east",
                    "north" => "south",
                    "south" => "north",
                    _ => unreachable!(),
                };
                adjacent_props.insert(opposite_direction.to_string(), "true".to_string());
                let updated_block_data = BlockData {
                    name: block_data.name.clone(),
                    properties: Some(adjacent_props),
                };
                let updated_block_id = updated_block_data.to_block_state_id();
                state
                    .world
                    .get_or_generate_mut(pos.chunk(), Dimension::Overworld)
                    .expect("Could not load chunk")
                    .set_block(pos.chunk_block_pos(), updated_block_id);
                changed_blocks.insert(pos, updated_block_id);
            }
        }

        let block_data = BlockData {
            name: name.to_string(),
            properties: Some(props),
        };

        let block_state_id = block_data.to_block_state_id();
        state
            .world
            .get_or_generate_mut(context.block_position.chunk(), Dimension::Overworld)
            .expect("Could not load chunk")
            .set_block(context.block_position.chunk_block_pos(), block_state_id);
        changed_blocks.insert(context.block_position, block_state_id);

        Ok(PlacedBlocks {
            blocks: changed_blocks,
            take_item: true,
        })
    }
}
