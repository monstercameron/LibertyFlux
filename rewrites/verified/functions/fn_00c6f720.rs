// original: 0x00C6F720 anim_group_name

/// Return the name field for an animation group index.
///
/// The function reads the 32-bit pointer stored in the animation-group table
/// global, adds the name-field offset within a group record, then adds the
/// index multiplied by the 0x58-byte record stride. It performs no bounds
/// check, makes no calls, and writes no memory. The original uses wrapping
/// 32-bit address arithmetic and removes its single stack argument.
lf_checker_rt::export!(stdcall, rw_00c6f720(group_index: u32) -> u32 {
    const ANIM_GROUP_TABLE: u32 = 0x016D_D640;
    const GROUP_NAME_OFFSET: u32 = 0x20;
    const GROUP_RECORD_STRIDE: u32 = 0x58;

    unsafe {
        let group_table = lf_checker_rt::global::<u32>(ANIM_GROUP_TABLE).read_unaligned();
        group_table
            .wrapping_add(GROUP_NAME_OFFSET)
            .wrapping_add(group_index.wrapping_mul(GROUP_RECORD_STRIDE))
    }
});
