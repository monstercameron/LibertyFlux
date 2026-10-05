// original: 0x00AEFB50 stream_triplet_a (proposed)

/// Resolve three suffixed names against a base name into an out triple.
///
/// For each of three seed strings in turn: join it with `base` through the
/// join callee (seed, base, 0), ask the named-float callee with (this,
/// joined, 0), and keep the float. The three floats are stored to `out`
/// at offsets 0, 4 and 8. Returns the out pointer.
///
/// Original: 0x00AEFB50 (thiscall, two stack words, two direct callees).
lf_checker_rt::export!(thiscall, rw_00aefb50(this: u32, out: u32, base: u32) -> u32 {
    unsafe {
        const JOIN_CALLEE: u32 = 1;
        const FLOAT_CALLEE: u32 = 2;
        const SEEDS: [u32; 3] = [0x00EA7DA8, 0x00EA7DA0, 0x00EA7D98];
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
