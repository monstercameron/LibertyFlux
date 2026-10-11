// original: 0x00D27FF0 task_manager_find_free_task_slot (proposed)

/// Scan eight task cells at a 0x40-byte stride, beginning at receiver offset
/// 0x34. When the first empty cell is found, call the receiver's indexed
/// helper, then call the cell's helper with the incoming task pointer. Return
/// true after that pair of calls; return false if all eight cells are
/// occupied. This thiscall method removes its single four-byte stack input.
lf_checker_rt::export!(thiscall, rw_00D27FF0(this: u32, task: u32) -> u8 {
    const CELL_STATE: u32 = 0x34;
    const CELL_BASE: u32 = 0x20;
    const CELL_STRIDE: u32 = 0x40;
    const CELL_COUNT: u32 = 8;
    const FIND_SLOT_CALLEE: u32 = 1;
    const ASSIGN_TASK_CALLEE: u32 = 2;

    unsafe {
        let mut slot = 0u32;
        while slot < CELL_COUNT {
            let state = this
                .wrapping_add(CELL_STATE)
                .wrapping_add(slot.wrapping_mul(CELL_STRIDE));
            if crate::read_u32(state) == 0 {
                let _ = lf_checker_rt::callee_thiscall!(FIND_SLOT_CALLEE, u32, this, slot);
                let cell = this
                    .wrapping_add(CELL_BASE)
                    .wrapping_add(slot.wrapping_mul(CELL_STRIDE));
                let _ = lf_checker_rt::callee_thiscall!(ASSIGN_TASK_CALLEE, u32, cell, task);
                return 1;
            }
            slot = slot.wrapping_add(1);
        }
        0
    }
});
