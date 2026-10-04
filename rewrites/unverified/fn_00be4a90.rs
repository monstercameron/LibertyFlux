// original: 0x00be4a90 task_slot_clear_if_equal (proposed)

/// Clear a task slot's tag word when it still holds the expected value.
///
/// `obj` points to the task object; `want` is the expected tag. Reads the
/// 32-bit tag at `obj + TAG_OFF` (0x38), compares it with `want`, and writes
/// zero over it only on equality. Returns the tag value as read, whether or
/// not it was cleared. The comparison is an exact 32-bit equality; there are
/// no other inputs and no calls.
///
/// Original: 0x00be4a90 (cdecl, two stack words: expected tag, object).
lf_checker_rt::export!(cdecl, rw_00be4a90(want: u32, obj: u32) -> u32 {
    unsafe {
        const TAG_OFF: u32 = 0x38;
        let cur = (obj.wrapping_add(TAG_OFF) as *const u32).read_unaligned();
        if cur == want {
            (obj.wrapping_add(TAG_OFF) as *mut u32).write_unaligned(0);
        }
        cur
    }
});
