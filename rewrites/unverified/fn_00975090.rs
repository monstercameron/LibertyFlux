// original: 0x00975090 audio_triple_store_squared (proposed)

/// Store two ids and the square of a gain: +0xC0 holds `a`, +0xC4 holds `b`,
/// +0xC8 holds `g * g` through one multiply whose operand order is pinned.
/// Returns `b` in EAX.
/// Original: 0x00975090 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00975090(this: u32, a: u32, b: u32, g: f32) -> u32 {
    unsafe {
        const O_A: u32 = 0xC0;
        const O_B: u32 = 0xC4;
        const O_G: u32 = 0xC8;
        let pinned = core::hint::black_box(g);
        let sq = core::hint::black_box(pinned) * core::hint::black_box(pinned);
        ((this.wrapping_add(O_A)) as *mut u32).write_unaligned(a);
        ((this.wrapping_add(O_B)) as *mut u32).write_unaligned(b);
        ((this.wrapping_add(O_G)) as *mut f32).write_unaligned(sq);
        b
    }
});
