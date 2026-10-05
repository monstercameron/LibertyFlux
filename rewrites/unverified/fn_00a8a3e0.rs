// original: 0x00a8a3e0 pool_set_pair_a08 (proposed)

/// Copy two words from a source pair into this object's slots at +0xA08.
///
/// `this` points to the pool object, `src` to two consecutive words. Both
/// words are copied in order; the value left in eax (the second word) is
/// returned as the original leaves it.
///
/// Original: 0x00A8A3E0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8a3e0(this: u32, src: u32) -> u32 {
    unsafe {
        const SLOT0: u32 = 0x0a08;
        const SLOT1: u32 = 0x0a0c;
        let w0 = (src as *const u32).read_unaligned();
        let w1 = (src as *const u32).add(1).read_unaligned();
        ((this + SLOT0) as *mut u32).write_unaligned(w0);
        ((this + SLOT1) as *mut u32).write_unaligned(w1);
        w1
    }
});
