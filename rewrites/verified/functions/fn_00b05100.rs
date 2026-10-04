// original: 0x00b05100 sort_dispatch
/// Sort dispatcher: direct insertion sort for spans of 16 records or
/// fewer, split-and-recurse otherwise. The second call's last argument
/// slot holds this function's own return address (verified by stack
/// arithmetic: with the first callee being caller-cleanup, [esp+0x24] at
/// the push is the return-address slot); a Rust rewrite cannot observe
/// that value, so the contract skips that one argument and compares
/// everything else. The rewrite passes 0 there.
export!(cdecl, rw_00b05100(a0: u32, a1: u32, a2: u32) -> u32 {
    let span = a1.wrapping_sub(a0) & !7;
    if (span as i32) > 0x80 {
        let mid = a0.wrapping_add(0x80);
        let _: u32 = callee_cdecl!(1, u32, a0, mid, 0, a2);
        callee_cdecl!(2, u32, mid, a1, 0, 0)
    } else {
        callee_cdecl!(1, u32, a0, a1, 0, a2)
    }
});
