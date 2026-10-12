// original: 0x00B13FF0 lookup_grid_cell_for_entity

/// Select the model-specific grid for an entity, then query it at the entity's
/// position. The signed 16-bit model index selects a pointer in the global
/// model-info table; the selected record's grid pointer is at byte offset
/// 0x3c. If the entity's optional position pointer at 0x20 is non-null, the
/// queried position starts 0x30 bytes into that object; otherwise it starts
/// 0x10 bytes into the entity. The grid lookup receives the incoming `this`,
/// the position pointer, and the selected grid pointer. The callee returns the
/// query result and uses thiscall stack cleanup.
lf_checker_rt::export!(thiscall, rw_00B13FF0(this: u32, entity: u32) -> u32 {
    unsafe {
        const MODEL_INFO_TABLE_VA: u32 = 0x0129_5CD8;
        const MODEL_INFO_GRID_OFFSET: usize = 0x3c;
        const ENTITY_POSITION_OFFSET: u32 = 0x10;
        const ENTITY_OPTIONAL_POSITION_OFFSET: u32 = 0x20;
        const OPTIONAL_POSITION_DATA_OFFSET: u32 = 0x30;

        let entity_bytes = entity as *const u8;
        let model_index = core::ptr::read_unaligned(entity_bytes.add(0x2e).cast::<i16>()) as isize;
        let model_info = core::ptr::read_unaligned(
            lf_checker_rt::global::<u32>(MODEL_INFO_TABLE_VA).offset(model_index),
        );
        let grid = core::ptr::read_unaligned(
            (model_info as *const u8).add(MODEL_INFO_GRID_OFFSET).cast::<u32>(),
        );
        let optional_position = core::ptr::read_unaligned(
            entity_bytes.add(ENTITY_OPTIONAL_POSITION_OFFSET as usize).cast::<u32>(),
        );
        let position = if optional_position != 0 {
            optional_position.wrapping_add(OPTIONAL_POSITION_DATA_OFFSET)
        } else {
            entity.wrapping_add(ENTITY_POSITION_OFFSET)
        };

        lf_checker_rt::callee_thiscall!(1, u32, this, position, grid)
    }
});
