// original: 0x00be4d60 task_effect_clear_handle_c (proposed)

/// Clear one effect slot's handle word.
///
/// `obj` points to the slot owner (the second stack word; the first is
/// unread). Writes zero to the 32-bit word at `obj + HANDLE_OFF` (0x1c) and
/// nothing else. Returns `obj` unchanged.
///
/// Original: 0x00be4d60 (cdecl, two stack words: unused, object).
lf_checker_rt::export!(cdecl, rw_00be4d60(_unused: u32, obj: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 0x1c;
        (obj.wrapping_add(HANDLE_OFF) as *mut u32).write_unaligned(0);
        obj
    }
});
