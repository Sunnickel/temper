use crate::BlockStateId;
use crate::errors::BlockPlaceError;
use crate::{BlockFace, BlockPlaceContext, PlacableBlock, PlacedBlocks};
use std::collections::BTreeMap;
use temper_core::block_data::BlockData;
use temper_macros::block;
use temper_state::GlobalState;

pub(crate) struct PlacableLog;

impl PlacableBlock for PlacableLog {
    fn place(
        context: BlockPlaceContext,
        state: GlobalState,
    ) -> Result<PlacedBlocks, BlockPlaceError> {
        let target_block = {
            let chunk = state
                .world
                .get_or_generate_mut(
                    context.block_position.chunk(),
                    temper_core::dimension::Dimension::Overworld,
                )
                .expect("Could not load chunk");
            chunk.get_block(context.block_position.chunk_block_pos())
        };
        if target_block != block!("air") && target_block != block!("cave_air") {
            return Err(BlockPlaceError::TargetBlockNotEmpty(context.block_position));
        }
        let axis = match context.face_clicked {
            BlockFace::Top | BlockFace::Bottom => "y",
            BlockFace::North | BlockFace::South => "z",
            BlockFace::West | BlockFace::East => "x",
        };

        let block_name = match context.item_used.to_name() {
            Some(name) => name,
            None => return Err(BlockPlaceError::ItemIdHasNoNameMapping(context.item_used)),
        };

        let block_data = BlockData {
            name: block_name,
            properties: Some(BTreeMap::from([("axis".to_string(), axis.to_string())])),
        };

        let Some(block_id) = block_data.try_to_block_state_id() else {
            return Err(BlockPlaceError::BlockNotMappedToBlockStateId(block_data));
        };

        state
            .world
            .get_or_generate_mut(
                context.block_position.chunk(),
                temper_core::dimension::Dimension::Overworld,
            )
            .expect("Could not load chunk")
            .set_block(context.block_position.chunk_block_pos(), block_id);

        Ok(PlacedBlocks {
            blocks: std::collections::HashMap::from([(context.block_position, block_id)]),
            take_item: true,
        })
    }
}
