// original: 0x0069F2A0 unit_float_from_packed_byte_e
/// Scale the packed byte at `this+14` to [-1.0, 1.0].
///
/// `v = (byte as f32 - 127.5) * (1/127)`, clamped to [-1.0, 1.0] with
/// ordered float comparisons (no NaN can arise from byte input). The
/// multiply constant is the exact single 1/127 the original loads. Float
/// operation order is the original's and pinned. Returns the value on the
/// x87 stack (`fld`). One of four identical routines differing only in the
/// byte offset (12, 13, 14, 15). Original: thiscall, no stack words, no calls.
lf_checker_rt::export!(thiscall, rw_0069f2a0(this: u32) -> f32 {
    unsafe {
        const OFF: u32 = 14;
        const BIAS: f32 = 127.5;
        const RECIP127: f32 = f32::from_bits(0x3C01_0204);
        const LO: f32 = -1.0;
        const HI: f32 = 1.0;
        let b = (this.wrapping_add(OFF) as *const u8).read() as f32;
        let t = core::hint::black_box(b) - core::hint::black_box(BIAS);
        let v = core::hint::black_box(t) * core::hint::black_box(RECIP127);
        if core::hint::black_box(LO) > v {
            LO
        } else if core::hint::black_box(v) > core::hint::black_box(HI) {
            HI
        } else {
            v
        }
    }
});
