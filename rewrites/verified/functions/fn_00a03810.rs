// original: 0x00A03810 frag_filter_submit (proposed)

/// Submit four filter floats and a tag to the filter sink callee.
///
/// Packs `a0..a2` into a frame block and passes (`block`, `a3`, `a4`) to
/// the sink: the `(an instruction of the original)` in the original only reserves the slot, which
/// is overwritten with `a3` before the call. Returns the sink's answer.
///
/// Original: 0x00A03810 (cdecl, five stack words).
lf_checker_rt::export!(cdecl, rw_00A03810(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        let blk = [a0, a1, a2];
        lf_checker_rt::callee_cdecl!(1, u32, blk.as_ptr() as u32, a3, a4)
    }
});
