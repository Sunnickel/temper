use std::collections::BTreeMap;
use temper_core::block_data::BlockData;
use crate::errors::BlockPlaceError;
use crate::{BlockFace, BlockPlaceContext, PlacableBlock, PlacedBlocks};
use temper_macros::item;
use temper_state::GlobalState;

pub(crate) struct PlacableLog;

impl PlacableBlock for PlacableLog {
    fn place(
        context: BlockPlaceContext,
        state: GlobalState,
    ) -> Result<PlacedBlocks, BlockPlaceError> {
        let axis = match context.face_clicked {
            BlockFace::Top | BlockFace::Bottom => "y",
            BlockFace::North | BlockFace::South => "z",
            BlockFace::West | BlockFace::East => "x",
        };

        let block_name = match context.item_used {
            item!("oak_log") => "minecraft:oak_log",
            item!("spruce_log") => "minecraft:spruce_log",
            item!("birch_log") => "minecraft:birch_log",
            item!("jungle_log") => "minecraft:jungle_log",
            item!("acacia_log") => "minecraft:acacia_log",
            item!("dark_oak_log") => "minecraft:dark_oak_log",
            item!("mangrove_log") => "minecraft:mangrove_log",
            item!("cherry_log") => "minecraft:cherry_log",
            item!("pale_oak_log") => "minecraft:pale_oak_log",
            item!("crimson_stem") => "minecraft:crimson_stem",
            item!("warped_stem") => "minecraft:warped_stem",
            
            item!("stripped_oak_log") => "minecraft:stripped_oak_log",
            item!("stripped_spruce_log") => "minecraft:stripped_spruce_log",
            item!("stripped_birch_log") => "minecraft:stripped_birch_log",
            item!("stripped_jungle_log") => "minecraft:stripped_jungle_log",
            item!("stripped_acacia_log") => "minecraft:stripped_acacia_log",
            item!("stripped_dark_oak_log") => "minecraft:stripped_dark_oak_log",
            item!("stripped_mangrove_log") => "minecraft:stripped_mangrove_log",
            item!("stripped_cherry_log") => "minecraft:stripped_cherry_log",
            item!("stripped_pale_oak_log") => "minecraft:stripped_pale_oak_log",
            item!("stripped_crimson_stem") => "minecraft:stripped_crimson_stem",
            item!("stripped_warped_stem") => "minecraft:stripped_warped_stem",
            _ => return Err(BlockPlaceError::ItemNotMappedToBlock(context.item_used)),
        };
        
        let block_data = BlockData {
            name: block_name.to_string(),
            properties: Some(BTreeMap::from([("axis".to_string(), axis.to_string())])),
        };
        
        let Some(block_id) = block_data.try_to_block_state_id() else {
            return Err(BlockPlaceError::BlockNotMappedToBlockStateId(block_data));
        };

        state.world.get_or_generate_mut(context.block_position.chunk(), temper_core::dimension::Dimension::Overworld)
            .expect("Could not load chunk")
            .set_block(context.block_position.chunk_block_pos(), block_id);
        
        Ok(PlacedBlocks {
            blocks: std::collections::HashMap::from([(context.block_position, block_id)]),
            take_item: true,
        })
    }
}
