// original: 0x009FD440 frag_classify_probe (proposed)

/// Classify a fixed tag through two data-table probe callees.
///
/// Calls the first data-table callee with (pointer to the tag cell,
/// pointer to a zero cell)
/// — the slots live in global data and are redirected to recorder stubs by
/// the contract — feeds its answer to the second data-table callee (whose
/// answer is ignored), then classifies the tag cell the original compares:
/// above 15 unsigned returns 3, equal to 7 returns 2, else 0. With the
/// fixed tag 0x3f the result is always 3; the callees' answers only flow
/// into the second call's logged argument. Both callees pop their arguments
/// (8 and 4 bytes — the only split that balances the caller's cleanup).
///
/// Original: 0x009FD440 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_009FD440() -> u32 {
    unsafe {
        const SLOT1: u32 = 0x00E731D4;
        const SLOT2: u32 = 0x00E731F8;
        const TAG: u32 = 0x3F;
        let mut cell = [TAG, 0u32];
        let p_tag = (&mut cell[0] as *mut u32) as u32;
        let p_zero = (&mut cell[1] as *mut u32) as u32;
        let a1 = (lf_checker_rt::relocated(SLOT1) as *const u32).read_unaligned();
        let f1: extern "stdcall" fn(u32, u32) -> u32 = core::mem::transmute(a1 as usize);
        let r1 = f1(p_tag, p_zero);
        let a2 = (lf_checker_rt::relocated(SLOT2) as *const u32).read_unaligned();
        let f2: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(a2 as usize);
        f2(r1);
        let tag = cell[0];
        if tag > 15 {
            3
        } else if tag == 7 {
            2
        } else {
            0
        }
    }
});
