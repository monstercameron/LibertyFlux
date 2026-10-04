// original: 0x00d26830 ped_task_request_fire (proposed)

/// Fire a task request, normalising its arguments first.
///
/// Takes eight words: `a1` is a float select (an exact -1.0 becomes 0.75;
///
/// anything else, including NaN, passes through), `a3` is an index with -1
/// meaning 2, and the low bytes of `a5`, `a6`, `a7` become three flag words.
/// Forwards everything to the request handler (intercepted) as twelve words
/// `(a0, select, a2, 1, &a7, index, a4, 0, f5, f6, f7, 0)` — the fifth is a
/// pointer to the caller's own `a7` slot, so the contract skips its address
/// and snapshots its contents. Returns 1 when `a7` is zero, else 0, in the
/// low byte only (the upper bytes keep stack residue).
///
/// Original: 0x00D26830 (cdecl, eight stack arguments).
lf_checker_rt::export!(cdecl, rw_00d26830(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32) -> u32 {
    unsafe {
        const SELECT_BITS: u32 = 0xbf80_0000; // -1.0f
        const SELECT_ALT_BITS: u32 = 0x3f40_0000; // 0.75f
        const INDEX_DEFAULT: u32 = 2;
        const INDEX_SENTINEL: u32 = 0xffff_ffff;
        let f1 = f32::from_bits(a1);
        let select = if f1 == f32::from_bits(SELECT_BITS) {
            f32::from_bits(SELECT_ALT_BITS)
        } else {
            f1
        };
        let index = if a3 == INDEX_SENTINEL { INDEX_DEFAULT } else { a3 };
        let f7 = if (a7 & 0xff) != 0 { 1u32 } else { 0 };
        let f6 = if (a6 & 0xff) != 0 { 1u32 } else { 0 };
        let f5 = if (a5 & 0xff) != 0 { 1u32 } else { 0 };
        let a7ptr = (&a7 as *const u32) as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            1, u32, a0, select.to_bits(), a2, 1, a7ptr, index, a4, 0, f5, f6, f7, 0
        );
        if a7 == 0 { 1 } else { 0 }
    }
});
