// original: 0x0097B700 audio_voice_init
// 0x0097B700 audio_voice_init (proposed): thiscall/1.
//
// Stores the config word, fills the descriptor with default flags, rates and
// zeroed filter blocks, derives two reciprocal-rate words from global tunables
// (1.0 / (f32)(u32)global, unsigned conversion), pushes the three parameter
// blocks through helper id3, and returns helper id4's answer.
export!(thiscall, rw_97b700(this: u32, cfg: u32) -> u32 {
    unsafe {
        let w = |off: usize| (this as *mut u32).add(off / 4);
        let b = this as *mut u8;
        *w(0x120) = cfg;
        *b.add(0x129) = 1;
        *w(0x78) = 1;
        *w(0x7c) = 0xb;
        *w(0x80) = 1;
        *w(0x170) = 0;
        *b.add(0x174) = 0;
        *w(0x178) = 1;
        let h = callee_thiscall!(1, u32, this);
        *w(0x08) = h;
        if h != 0 {
            // Global rate word forwarded by value (bits, no arithmetic).
            let rate_bits = *global::<u32>(0x1038990);
            callee_thiscall!(2, u32, h, this, rate_bits, 0x7d0, 0xfa0, 0x3f000000);
        }
        // Reciprocal rates: the original converts via cvtdq2pd plus a 2^32
        // bias for negative inputs, i.e. an unsigned u32 -> f32 conversion.
        let g2 = *global::<u32>(0x1038970);
        let f2 = 1.0f32 / (g2 as f32);
        callee_thiscall!(3, u32, this.wrapping_add(0x44), f2.to_bits(), f2.to_bits(), 0, 0x3f800000);
        let g3 = *global::<u32>(0x1038974);
        let f3 = 1.0f32 / (g3 as f32);
        callee_thiscall!(3, u32, this.wrapping_add(0x90), f3.to_bits(), f3.to_bits(), 0, 0x3f800000);
        *w(0xc4) = 0;
        *w(0xc8) = 0;
        *w(0xcc) = 0;
        *w(0xd0) = 0;
        *w(0x148) = 0;
        *w(0x14c) = 0;
        *w(0x150) = 0;
        *w(0x154) = 0;
        *w(0x15c) = 0;
        *w(0x160) = 0;
        *w(0x164) = 0xffff_ffff;
        *w(0x168) = 0xffff_ffff;
        callee_thiscall!(3, u32, this.wrapping_add(0x17c), 0x3b23_d70a, 0x3ba3_d70a, 0, 0x3f800000);
        *w(0x198) = 0;
        *w(0x19c) = 0;
        callee_thiscall!(4, u32, this)
    }
});
