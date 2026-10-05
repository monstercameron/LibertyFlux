// original: 0x00c45b50 ccam_copy_block128 (proposed)
/// Copy the 128-byte camera block at `this + 0x10` to `dst`.
///
/// If `dst` is null nothing happens. Otherwise 32 words are copied from
/// `this + SRC_OFF` to `dst` (the original uses `rep movsd` with count 32).
/// No value is returned.
///
/// Original: thiscall, one stack word, callee cleanup (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c45b50(this: u32, dst: u32) -> u32 {
    const SRC_OFF: u32 = 0x10;
    const WORDS: u32 = 0x20;
    if dst != 0 {
        let src = (this + SRC_OFF) as *const u32;
        let d = dst as *mut u32;
        let mut i = 0u32;
        while i < WORDS {
            unsafe { d.add(i as usize).write_unaligned(src.add(i as usize).read_unaligned()) };
            i += 1;
        }
    }
    0
});
