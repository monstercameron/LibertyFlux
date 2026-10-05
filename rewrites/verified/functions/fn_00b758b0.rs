// original: 0x00b758b0 global_choice_in_range (proposed)

/// Decide whether the active global choice lies strictly between 6 and 22.
///
/// Reads two globals: the override at 0x1295854 and the stored choice at
/// 0x1295848. The override wins unless it is -1 (none). Returns 1 when the
/// winning value is strictly between 6 and 22 (both compares signed), else
/// 0. The low return byte is the decision; the upper 24 bits repeat the
/// override word with its low byte cleared (the original sets only al).
///
/// Original: 0x00b758b0 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00b758b0() -> u32 {
    unsafe {
        const STORED_VA: u32 = 0x1295848;
        const OVERRIDE_VA: u32 = 0x1295854;
        const NONE: u32 = 0xffff_ffff;
        const LO: i32 = 6;
        const HI: i32 = 0x16;
        let ovr = *lf_checker_rt::global::<u32>(OVERRIDE_VA);
        let mut v = *lf_checker_rt::global::<u32>(STORED_VA);
        if ovr != NONE {
            v = ovr;
        }
        let s = v as i32;
        let flag = if s >= HI || s <= LO { 0u32 } else { 1u32 };
        (ovr & 0xffff_ff00) | flag
    }
});
