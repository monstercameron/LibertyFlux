// original: 0x008DC3A0 draw_command_alloc

/// Allocate bytes from the command buffer. Round the requested size up to a 16-byte boundary, ask the shared space checker to update its used offset, then return the selected slot's backing pointer plus the previous offset. If the resulting offset reaches 2 MiB, reset it and mark the slot for reuse. The slot index, backing pointer, cursor, and marker are 32-bit game globals.
lf_checker_rt::export!(cdecl, rw_008dc3a0(size: u32, auxiliary: u32) -> u32 {
    const TABLE_BASE_VA: u32 = 0x01175C58;
    const SLOT_INDEX_VA: u32 = 0x01175C6C;
    const USED_OFFSET_VA: u32 = 0x01175C74;
    const FREE_MARKERS_VA: u32 = 0x01175C60;
    const CAPACITY: u32 = 0x00200000;
    const SPACE_CHECK_CALLEE: u32 = 95;

    let alignment_padding = size.wrapping_neg() & 0xF;
    let aligned_size = size.wrapping_add(alignment_padding);
    let _ = lf_checker_rt::callee_thiscall!(
        SPACE_CHECK_CALLEE,
        u32,
        lf_checker_rt::relocated(TABLE_BASE_VA),
        aligned_size,
        auxiliary,
    );
    unsafe {
        let slot_index = core::ptr::read_volatile(lf_checker_rt::global::<u32>(SLOT_INDEX_VA).cast_const());
        let previous_used = core::ptr::read_volatile(lf_checker_rt::global::<u32>(USED_OFFSET_VA).cast_const());
        let table_slot = TABLE_BASE_VA.wrapping_add(slot_index.wrapping_shl(2));
        let slot_base = core::ptr::read_volatile(lf_checker_rt::global::<u32>(table_slot).cast_const());
        let allocation = slot_base.wrapping_add(previous_used);
        let next_used = previous_used.wrapping_add(aligned_size);
        core::ptr::write_volatile(lf_checker_rt::global::<u32>(USED_OFFSET_VA), next_used);

        if next_used >= CAPACITY {
            core::ptr::write_volatile(lf_checker_rt::global::<u32>(USED_OFFSET_VA), 0);
            core::ptr::write_volatile(lf_checker_rt::global::<u8>(FREE_MARKERS_VA.wrapping_add(slot_index)), 1);
            let slot_base = core::ptr::read_volatile(lf_checker_rt::global::<u32>(table_slot).cast_const());
            slot_base.wrapping_add(0)
        } else {
            allocation.wrapping_add(0)
        }
    }
});
