// original: 0x00AEFD60 stream_quad_c (proposed)

/// Resolve four suffixed names against a base name into an out vector.
///
/// Same shape as the sibling triplet resolvers with four seeds: join each
/// seed with `base`, ask the named-float callee, store the four floats to
/// `out` at offsets 0, 4, 8 and 12. Returns the out pointer.
///
/// Original: 0x00AEFD60 (thiscall, two stack words, two direct callees).
lf_checker_rt::export!(thiscall, rw_00aefd60(this: u32, out: u32, base: u32) -> u32 {
    unsafe {
        const JOIN_CALLEE: u32 = 1;
        const FLOAT_CALLEE: u32 = 2;
        const SEEDS: [u32; 4] = [0x00EA7DC8, 0x00EA7DC4, 0x00EA7DC0, 0x00EA7DBC];
        let mut i = 0usize;
        while i < SEEDS.len() {
            let joined: u32 = lf_checker_rt::callee_cdecl!(JOIN_CALLEE, u32, base, lf_checker_rt::relocated(SEEDS[i]), 0);
            let f: f32 = lf_checker_rt::callee_thiscall!(FLOAT_CALLEE, f32, this, joined, 0);
            ((out.wrapping_add((i as u32).wrapping_mul(4))) as *mut f32).write_unaligned(f);
            i += 1;
        }
        out
    }
});
