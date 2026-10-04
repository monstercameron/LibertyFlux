// original: 0x00be4d50 task_effect_reset_slot_b (proposed)

/// Reset one effect slot: clear its handle word and mark it inactive.
///
/// `obj` points to the slot owner (the second stack word; the first is
/// unread). Writes zero to the 32-bit word at `obj + HANDLE_OFF` (0x14) and
/// one to the flag byte at `obj + FLAG_OFF` (0x20), in that order. Returns
/// `obj` unchanged. Same shape as the neighbouring slot resets, differing
/// only in the two offsets.
///
/// Original: 0x00be4d50 (cdecl, two stack words: unused, object).
lf_checker_rt::export!(cdecl, rw_00be4d50(_unused: u32, obj: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 0x14;
        const FLAG_OFF: u32 = 0x20;
        (obj.wrapping_add(HANDLE_OFF) as *mut u32).write_unaligned(0);
        (obj.wrapping_add(FLAG_OFF) as *mut u8).write(1);
        obj
    }
});
