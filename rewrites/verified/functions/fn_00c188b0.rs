// original: 0x00c188b0 proximity_check_xy_absz

/// Test whether a target point is near a source point, when enabled.
///
/// `obj` (may be null) holds a flag byte at `+3` and a point at
/// `+0x34/+0x38/+0x3c`; `tgt` points to a triple. Returns 0 unless `obj` is
/// non-null, bit 0 of the flag byte is set, the horizontal squared distance
/// `(dx*dx + dy*dy)` is strictly below the constant at `0x00e9cae0` (900.0),
/// and the absolute vertical gap is not above the constant at `0x00fe8b38`
/// (20.0); else 1. NaN in either distance gives 0 (both jumps take the
/// unordered path). The absolute value clears the sign bit, as the original's
/// `andps` with the mask at `0x00fe8f80` does.
///
/// Original: 0x00C188B0 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_00c188b0(obj: u32, tgt: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            let r = core::hint::black_box(a) * core::hint::black_box(b);
            core::hint::black_box(a);
            core::hint::black_box(b);
            r
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            let r = core::hint::black_box(a) + core::hint::black_box(b);
            core::hint::black_box(a);
            core::hint::black_box(b);
            r
        }
        if obj == 0 {
            return 0;
        }
        if ((obj + 3) as *const u8).read() & 1 == 0 {
            return 0;
        }
        let dx = ((obj + 0x34) as *const f32).read_unaligned()
            - (tgt as *const f32).read_unaligned();
        let dy = ((obj + 0x38) as *const f32).read_unaligned()
            - ((tgt + 4) as *const f32).read_unaligned();
        let dxy = fadd(fmul(dx, dx), fmul(dy, dy));
        let range_sq = lf_checker_rt::global::<f32>(0x00e9_cae0).read();
        if !(range_sq > dxy) {
            return 0;
        }
        let dz = ((obj + 0x3c) as *const f32).read_unaligned()
            - ((tgt + 8) as *const f32).read_unaligned();
        let adz = f32::from_bits(dz.to_bits() & 0x7fff_ffff);
        let band = lf_checker_rt::global::<f32>(0x00fe_8b38).read();
        if !(adz <= band) {
            return 0;
        }
        1
    }
});
