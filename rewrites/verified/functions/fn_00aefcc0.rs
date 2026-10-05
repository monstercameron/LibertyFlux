// original: 0x00AEFCC0 stream_triplet_b (proposed)

/// Resolve three suffixed names against a base name into an out triple.
///
/// Same shape as the sibling triplet resolver with a different seed set:
/// join each seed with `base`, ask the named-float callee, store the three
/// floats to `out` at offsets 0, 4 and 8. Returns the out pointer.
///
/// Original: 0x00AEFCC0 (thiscall, two stack words, two direct callees).
lf_checker_rt::export!(thiscall, rw_00aefcc0(this: u32, out: u32, base: u32) -> u32 {
    unsafe {
        const JOIN_CALLEE: u32 = 1;
        const FLOAT_CALLEE: u32 = 2;
        const SEEDS: [u32; 3] = [0x00EA7DB8, 0x00EA7DB4, 0x00EA7DAC];
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
