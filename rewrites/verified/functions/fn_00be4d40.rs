// original: 0x00be4d40 task_effect_reset_slot_a (proposed)

/// Reset one effect slot: clear its handle word and mark it inactive.
///
/// `obj` points to the slot owner (the second stack word; the first is
/// unread). Writes zero to the 32-bit word at `obj + HANDLE_OFF` (0x18) and
/// one to the flag byte at `obj + FLAG_OFF` (0x24), in that order. Returns
/// `obj` unchanged.
///
/// Original: 0x00be4d40 (cdecl, two stack words: unused, object).
lf_checker_rt::export!(cdecl, rw_00be4d40(_unused: u32, obj: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 0x18;
        const FLAG_OFF: u32 = 0x24;
        (obj.wrapping_add(HANDLE_OFF) as *mut u32).write_unaligned(0);
        (obj.wrapping_add(FLAG_OFF) as *mut u8).write(1);
        obj
    }
});
