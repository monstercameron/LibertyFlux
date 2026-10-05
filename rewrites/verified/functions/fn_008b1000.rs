// original: 0x008B1000 audio_block_copy_17a0 (proposed)

/// Copy twelve words from `src` into the object at offsets `0x17a0..0x17d0`.
///
/// `this` is the effect object, `src` points at twelve argument words. The
/// original issues twelve discrete dword loads and stores (no loop, no
/// rep-mov). No return value, no calls. Original is thiscall with one stack
/// word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_008B1000(this: u32, src: u32) -> u32 {
    const DST: u32 = 0x17a0;
    const WORDS: u32 = 12;
    unsafe {
        let mut i = 0u32;
        while i < WORDS {
            let w = ((src + i * 4) as *const u32).read_unaligned();
            ((this + DST + i * 4) as *mut u32).write_unaligned(w);
            i += 1;
        }
    }
    0
});
