// original: 0x00A4B670 vehicle_accumulate_10c8 (proposed)

/// Accumulates a scaled speed magnitude into the float at `this + ACC`,
/// clamped above at 15.0.
///
/// Calls the velocity source through virtual slot `SRC_SLOT` (0xEC) with
/// `this` in `ecx` and a frame scratch word on the stack (skipped by the
/// contract: a frame address, its contents never read back), loading the
/// target through the object's vtable. Takes the answer as two floats
/// `(vx, vy)`, computes `mag = sqrt(vx*vx + vy*vy)` in the original's SSE
/// order (pinned with `black_box`), scales by the global step `DT` and by
/// one of two global gains (those at `GAIN_LO`, used when `1.0 > [GAIN_HI]`,
/// else those at `GAIN_HI`), adds the current accumulator, and stores
/// `min(total, 15.0)` back (a NaN total stores 15.0, matching the original's
/// unordered-`comiss` path). Returns the source answer in `eax`.
///
/// Original: 0x00A4B670 (thiscall, no stack words), one indirect callee.
lf_checker_rt::export!(thiscall, rw_00A4B670(this: u32) -> u32 {
    unsafe {
        const ACC: u32 = 0x10C8;
        const SRC_SLOT: u32 = 0xEC;
        const DT: u32 = 0x011735BC;
        const GAIN_HI: u32 = 0x0103CCF8;
        const GAIN_LO: u32 = 0x0103CCFC;
        const ONE: f32 = f32::from_bits(0x3F80_0000); // 1.0
        const CAP: f32 = f32::from_bits(0x4170_0000); // 15.0
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let vtable = (this as *const u32).read_unaligned();
        let src: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(
                ((vtable + SRC_SLOT) as *const u32).read_unaligned() as usize,
            );
        let mut scratch = [0u32; 4];
        let ans = src(this, scratch.as_mut_ptr() as u32);
        let vx =
            f32::from_bits((ans as *const u32).read_unaligned());
        let vy =
            f32::from_bits(((ans + 4) as *const u32).read_unaligned());
        let mag = mul(vx, vx);
        let mag = add(mag, mul(vy, vy));
        let mag = core::hint::black_box(mag).sqrt();
        let dt = f32::from_bits(
            lf_checker_rt::global::<u32>(DT).read_unaligned(),
        );
        let ghi = f32::from_bits(
            lf_checker_rt::global::<u32>(GAIN_HI).read_unaligned(),
        );
        let glo = f32::from_bits(
            lf_checker_rt::global::<u32>(GAIN_LO).read_unaligned(),
        );
        // jbe after comiss(1.0, ghi): taken when 1.0 <= ghi or unordered.
        let k = if ghi.is_nan() || ONE <= ghi { ghi } else { glo };
        let total = add(mul(mul(mag, dt), k), f32::from_bits(
            ((this + ACC) as *const u32).read_unaligned(),
        ));
        let out = if CAP > total { total } else { CAP };
        ((this + ACC) as *mut u32).write_unaligned(out.to_bits());
        ans
    }
});
