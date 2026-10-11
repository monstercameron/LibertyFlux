// original: 0x00D4F930 should_ignore_related_object_for_task
/// Decide whether to ignore a related object for the current task. A null
/// subject or a zero state word at `+0x38` returns false. Otherwise a related
/// object's virtual query runs through slot `+0x128`; a false query, a low
/// nibble below two at subject offset `+0x1e2`, or a pointer match at `+0x1bc`
/// returns true. The nibble comparison is unsigned. This cdecl function takes
/// two stack pointers and returns a one-byte boolean.
lf_checker_rt::export!(cdecl, rw_00d4f930(subject: u32, related: u32) -> u8 {
    const STATE_WORD: u32 = 0x38;
    const STATE_NIBBLE: u32 = 0x1e2;
    const RELATED_POINTER: u32 = 0x1bc;
    const RELATED_QUERY: u32 = 1;
    const MINIMUM_NIBBLE: u8 = 2;

    if subject == 0 {
        return 0;
    }

    let state = unsafe { ((subject + STATE_WORD) as *const u32).read_unaligned() };
    if state == 0 {
        return 0;
    }

    let query_result: u8 = lf_checker_rt::callee_thiscall!(RELATED_QUERY, u8, related);
    if query_result == 0 {
        return 1;
    }

    let nibble = unsafe { ((subject + STATE_NIBBLE) as *const u8).read() & 0x0f };
    if nibble < MINIMUM_NIBBLE {
        return 1;
    }

    let stored_related =
        unsafe { ((subject + RELATED_POINTER) as *const u32).read_unaligned() };
    u8::from(stored_related == related)
});
