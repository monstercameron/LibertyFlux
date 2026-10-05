// original: 0x00c041d0 stream_short3_get (proposed)

/// Unpack three words at `+0x18`, `+0x1a`, `+0x1c` into floats.
///
/// `this` points to the object, `out` to three caller-owned floats. Each
/// words is sign-extended, converted to float and multiplied by `K_SCALE`
/// (`1/273.06`). Returns the third words sign-extended (what the original
/// leaves in `eax`).
///
/// Original: 0x00c041d0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c041d0(this: u32, out: u32) -> u32 {
    unsafe {
        const OFF0: u32 = 0x18;
        const OFF1: u32 = 0x1a;
        const OFF2: u32 = 0x1c;
        const K_SCALE: f32 = f32::from_bits(0x3b7001e1);
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let v0 = mul((((this.wrapping_add(OFF0)) as *const i16).read_unaligned() as i32) as f32, K_SCALE);
        (out as *mut u32).write_unaligned(v0.to_bits());
        let v1 = mul((((this.wrapping_add(OFF1)) as *const i16).read_unaligned() as i32) as f32, K_SCALE);
        (out.wrapping_add(4) as *mut u32).write_unaligned(v1.to_bits());
        let last = (this.wrapping_add(OFF2) as *const i16).read_unaligned() as i32;
        let v2 = mul(last as f32, K_SCALE);
        (out.wrapping_add(8) as *mut u32).write_unaligned(v2.to_bits());
        last as u32
    }
});
