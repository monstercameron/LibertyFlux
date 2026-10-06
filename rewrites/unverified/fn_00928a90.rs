// original: 0x00928A90 select_frame_slot_and_init (proposed)

/// Initialise the current frame's render slot and mark it active.
///
/// Reads the frame index from `FRAME_INDEX`, selects the slot
/// `SLOT_BASE + index * SLOT_STRIDE`, and calls the slot-init helper
/// (callee 1, cdecl) with `(slot, slot + EXTRA_OFF, 0)`. Then writes 1 to
/// the active byte at `ACTIVE_FLAG`. Returns nothing meaningful.
///
/// Original: 0x00928A90 (cdecl, no arguments). One direct call, one byte write.
lf_checker_rt::export!(cdecl, rw_00928A90() -> u32 {
    unsafe {
        const FRAME_INDEX: u32 = 0x0117_4790;
        const SLOT_BASE: u32 = 0x011A_1C50;
        const SLOT_STRIDE: u32 = 0x270;
        const EXTRA_OFF: u32 = 0x60;
        const ACTIVE_FLAG: u32 = 0x011A_1BCC;
        let idx = (lf_checker_rt::relocated(FRAME_INDEX) as *const u32).read_unaligned();
        let slot = idx.wrapping_mul(SLOT_STRIDE).wrapping_add(lf_checker_rt::relocated(SLOT_BASE));
        lf_checker_rt::callee_cdecl!(1, u32, slot, slot.wrapping_add(EXTRA_OFF), 0u32);
        (lf_checker_rt::relocated(ACTIVE_FLAG) as *mut u8).write(1);
        0
    }
});
