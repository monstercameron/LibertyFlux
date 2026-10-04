// original: 0x009e91f0 ped_init_core_words
/// Initialises the four core words: keeps bit 21 of word 0 and sets
/// the signature bits, rebuilds word 2 from the old word 2 with its low
/// six bits forced, zeroes word 1 and stores 1.0 in word 3. The stack
/// argument is ignored. Returns the rebuilt word 2. (thiscall, 1 arg.)
lf_checker_rt::export!(thiscall, rw_009e91f0(this_ptr: u32, _unused: u32) -> u32 {
    unsafe {
        const KEEP_MASK: u32 = 0x200000;
        const SIG_BITS: u32 = 0x1F000000;
        const W2_MASK: u32 = 0xFFFFFFC0;
        const W2_BITS: u32 = 0x1C0;
        const ONE_BITS: u32 = 0x3F800000;
        let w0 = (this_ptr as *const u32).read_unaligned();
        (this_ptr as *mut u32).write_unaligned((w0 & KEEP_MASK) | SIG_BITS);
        let w2 = ((this_ptr.wrapping_add(8) as *const u32).read_unaligned() & W2_MASK) | W2_BITS;
        (this_ptr.wrapping_add(4) as *mut u32).write_unaligned(0);
        (this_ptr.wrapping_add(8) as *mut u32).write_unaligned(w2);
        (this_ptr.wrapping_add(0xC) as *mut u32).write_unaligned(ONE_BITS);
        w2
    }
});
