// original: 0x00a3c150 vehicle_table_transcend (proposed)

/// Transform one table float into three output floats via two transcendent
/// calls (ids 1 and 2, cdecl/3 each, sharing one scratch double).
///
/// Loads `v0 = base[(idx*128 + off)]`, scales it by 2^-16 and takes the
/// first call's out-double narrowed to `f1`. Then `v1 = v0 - f1*65536` is
/// scaled by 2^-9 for the second call, whose out-double narrows to `f4`.
/// The outputs are `(v1 - f4*256 - 128)/128`, `(f4 - 128)/128` and
/// `(f1 - 128)/128`, all single precision in the original's operand order.
/// Both calls' ST0 results are discarded. Stdcall/4 (out, base, off,
/// idx), returns `out`.
lf_checker_rt::export!(stdcall, rw_00a3c150(out: u32, base: u32, off: u32, idx: u32) -> u32 {
    unsafe {
        const DOWN16: f32 = f32::from_bits(0x3780_0000); // 2^-16
        const UP16: f32 = f32::from_bits(0x4780_0000); // 65536.0
        const DOWN9: f32 = f32::from_bits(0x3B80_0000); // 2^-9
        const UP8: f32 = f32::from_bits(0x4380_0000); // 256.0
        const DOWN7: f32 = f32::from_bits(0x3C00_0000); // 2^-7
        const C128: f32 = f32::from_bits(0x4300_0000);
        const FN1: u32 = 1;
        const FN2: u32 = 2;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let k = idx.wrapping_shl(7).wrapping_add(off);
        let v0 = f32::from_bits(core::ptr::read_unaligned(
            (base.wrapping_add(k.wrapping_mul(4))) as *const u32,
        ));
        core::ptr::write_unaligned(out as *mut u32, v0.to_bits());
        let t = mul(v0, DOWN16);
        // One scratch double, reused by both calls exactly like the
        // original reuses its stack slot.
        let mut s: f64 = 0.0;
        let tb = (t as f64).to_bits();
        // Stack layout matches the original exactly: each call passes
        // (double-lo, double-hi, out-pointer), so the pointer is arg 2.
        let _: f64 = lf_checker_rt::callee_cdecl!(
            FN1,
            f64,
            tb as u32,
            (tb >> 32) as u32,
            core::ptr::addr_of_mut!(s) as u32
        );
        let f1 = s as f32;
        core::ptr::write_unaligned((out + 8) as *mut u32, f1.to_bits());
        let v1 = sub(v0, mul(f1, UP16));
        core::ptr::write_unaligned(out as *mut u32, v1.to_bits());
        let w = mul(v1, DOWN9);
        let wb = (w as f64).to_bits();
        let _: f64 = lf_checker_rt::callee_cdecl!(
            FN2,
            f64,
            wb as u32,
            (wb >> 32) as u32,
            core::ptr::addr_of_mut!(s) as u32
        );
        let f4 = s as f32;
        let r4 = mul(sub(f4, C128), DOWN7);
        let r0 = mul(sub(f1, C128), DOWN7);
        let r1 = mul(sub(sub(v1, mul(f4, UP8)), C128), DOWN7);
        core::ptr::write_unaligned((out + 4) as *mut u32, r4.to_bits());
        core::ptr::write_unaligned((out + 8) as *mut u32, r0.to_bits());
        core::ptr::write_unaligned(out as *mut u32, r1.to_bits());
        out
    }
});
