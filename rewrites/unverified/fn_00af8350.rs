// original: 0x00AF8350 veh_kind_is_simple (proposed)

/// Test whether the kind byte selects a simple handling path.
///
/// Reads the signed byte at `this + 0x26` and dispatches through the
/// original's two-target jump table (table contents decoded into the match
/// below, since the table sits in code the rewrite cannot read). Returns 1
/// for kinds 2, 3, 6, 7, 9, 10, 11, 12, 13 and 16, and 0 for anything else.
///
/// Original: 0x00AF8350 (thiscall, no stack arguments, result in AL).
lf_checker_rt::export!(thiscall, rw_00AF8350(this: u32) -> u8 {
    unsafe {
        const KIND: u32 = 0x26;
        match ((this + KIND) as *const i8).read() {
            2 | 3 | 6 | 7 | 9 | 10 | 11 | 12 | 13 | 16 => 1,
            _ => 0,
        }
    }
});
