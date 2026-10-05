// original: 0x00AF8420 veh_code_is_active (proposed)

/// Test whether a code byte selects an active handling path.
///
/// Takes the low byte of the argument as signed and dispatches through the
/// original's two-target jump table (table contents decoded into the match
/// below, since the table sits in code the rewrite cannot read). Codes below
/// 2 or above 34 return 1, as do codes 4, 5, 8, 14 through 25 and 27;
/// every other code returns 0.
///
/// Original: 0x00AF8420 (cdecl, one stack argument, result in AL).
lf_checker_rt::export!(cdecl, rw_00AF8420(code: u32) -> u8 {
    let v = code as u8 as i8;
    if v < 2 || v > 34 {
        return 1;
    }
    match v {
        4 | 5 | 8 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 27 => 1,
        _ => 0,
    }
});
