// original: 0x00a8a400 pool_set_pair_a10 (proposed)

/// Copy two words from a source pair into this object's slots at +0xA10.
///
/// Same shape as the +0xA08 setter: `this` is the pool object, `src` points
/// to two consecutive words, both are copied in order, and the second word
/// (left in eax by the original) is returned.
///
/// Original: 0x00A8A400 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8a400(this: u32, src: u32) -> u32 {
    unsafe {
        const SLOT0: u32 = 0x0a10;
        const SLOT1: u32 = 0x0a14;
        let w0 = (src as *const u32).read_unaligned();
        let w1 = (src as *const u32).add(1).read_unaligned();
        ((this + SLOT0) as *mut u32).write_unaligned(w0);
        ((this + SLOT1) as *mut u32).write_unaligned(w1);
        w1
    }
});
