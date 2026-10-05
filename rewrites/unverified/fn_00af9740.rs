// original: 0x00AF9740 veh_limit_check (proposed)

/// Test whether the active entry's budget word is below the current limit.
///
/// Callee 1 produces a context that callee 2 turns into a one-based level;
/// the global index table entry for the global selector is then read, and
/// the result is 1 when that entry's word at `+0xB0` is signed less than
/// level minus one.
///
/// Original: 0x00AF9740 (cdecl, no arguments, result in AL).
lf_checker_rt::export!(cdecl, rw_00AF9740() -> u8 {
    unsafe {
        const SELECTOR: u32 = 0x12F9E34;
        const TABLE: u32 = 0x1295CD8;
        const BUDGET: u32 = 0xB0;
        const MAKE_CTX: u32 = 1;
        const LEVEL_OF: u32 = 2;
        let ctx = lf_checker_rt::callee_cdecl!(MAKE_CTX, u32,);
        let level = lf_checker_rt::callee_thiscall!(LEVEL_OF, u32, ctx);
        let sel = (lf_checker_rt::global::<u32>(SELECTOR) as *const u32).read_unaligned();
        let table = lf_checker_rt::relocated(TABLE);
        let entry = ((table.wrapping_add(sel.wrapping_mul(4))) as *const u32).read_unaligned();
        let budget = ((entry + BUDGET) as *const u32).read_unaligned();
        ((budget as i32) < (level.wrapping_sub(1) as i32)) as u8
    }
});
