// original: 0x00DDE050 MO_TEXT_FIELD builder
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Build the field's five text records and attach them. Issue five record
/// requests to the text builder, each carrying the next word of a five-word
/// scratch block, a record id (0x3E, 0x3C, 1, 2, 0x3E), the previous answer
/// and constant tag words; then attach the chain (the five answers plus the
/// tag words) to the field through the finalizer and return its answer.
/// The builder cleans nothing (caller convention); the finalizer pops the
/// whole 13-word chain. The scratch addresses differ per side (skipped in
/// the proof; the words are never re-read). Original: thiscall, no stack
/// words, final answer in eax.
lf_checker_rt::export!(thiscall, rw_00DDE050(this: u32) -> u32 {
    unsafe {
        const BUILD: u32 = 1;
        const ATTACH: u32 = 2;
        const TAG_A: u32 = 0x00EFCDF1;
        const TAG_B: u32 = 0x00EFCF64;
        const W15: u32 = 0x41700000;
        const W30: u32 = 0x41F00000;
        let mut blk = [0u32; 5];
        let base = blk.as_mut_ptr() as u32;
        let tag_a = lf_checker_rt::relocated(TAG_A);
        let tag_b = lf_checker_rt::relocated(TAG_B);
        let r1 = lf_checker_rt::callee_cdecl!(BUILD, u32, base, 0x3E, tag_a, tag_b, W15, W30);
        let r2 = lf_checker_rt::callee_cdecl!(BUILD, u32, base.wrapping_add(4), 0x3C, r1, tag_a, tag_b, W15);
        let r3 = lf_checker_rt::callee_cdecl!(BUILD, u32, base.wrapping_add(8), 1, r2, r1, tag_a, tag_b);
        let r4 = lf_checker_rt::callee_cdecl!(BUILD, u32, base.wrapping_add(12), 2, r3, r2, r1, tag_a);
        let r5 = lf_checker_rt::callee_cdecl!(BUILD, u32, base.wrapping_add(16), 0x3E, r4, r3, r2, r1);
        lf_checker_rt::callee_thiscall!(ATTACH, u32, this, r5, r4, r3, r2, r1,
                                        tag_a, tag_b, W15, W30, 0, 0, 0, 0)
    }
});
