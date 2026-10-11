// original: 0x00609930 physics_seed_joint_cache

/// Seed three cached scalar lanes from incoming XMM1 and copy the three
/// meaningful words from `source` into `this + 0x1f0`. The source gaps at
/// 0x0c and 0x1c are ignored. XMM1 is an input register; the worker mirror
/// supplies its exact low-word bits to this portable Rust body.
lf_checker_rt::export!(thiscall, rw_00609930(this: u32, source: u32) -> u32 {
    unsafe {
        const CACHE: u32 = 0x1e0;
        const SOURCE: u32 = 0x1f0;
        let scalar_bits = lf_checker_rt::xmm_word(1, 0);
        for offset in [0, 4, 8] {
            ((this + CACHE + offset) as *mut u32).write_unaligned(scalar_bits);
        }
        let mut last = 0;
        for offset in [0, 4, 8] {
            last = ((source + offset) as *const u32).read_unaligned();
            ((this + SOURCE + offset) as *mut u32).write_unaligned(last);
        }
        last
    }
});
