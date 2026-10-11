// original: 0x00C496D0 camera_spline_interpolate_state

/// Blend six single-precision components from `source` into this object.
///
/// `this` is the destination. `alternate` selects the baseline block: when
/// its pointer field at `+0x20` is non-null, the baseline begins at that
/// pointer plus `0x10`; otherwise it begins at `this + 0x10`. The source
/// components are at offsets `0x60` through `0x74`; baseline components are
/// at the corresponding offsets `0x50` through `0x64`. Each of the first
/// five destination values is `(source - baseline) * factor + baseline`.
/// The sixth uses the same operation and is then clamped to `[0, 1]`; ordered
/// comparisons leave a NaN unchanged. The selected baseline pointer is
/// returned in EAX. This is a thiscall routine with three stack words, which
/// the callee removes.
lf_checker_rt::export!(thiscall, rw_c496d0(this: u32, alternate: u32, source: u32, factor_bits: u32) -> u32 {
    unsafe {
        const ALTERNATE_BASE_FIELD: u32 = 0x20;
        const BASE_ADJUSTMENT: u32 = 0x10;
        const SOURCE_COMPONENTS: [u32; 6] = [0x60, 0x64, 0x68, 0x6c, 0x70, 0x74];
        const BASE_COMPONENTS: [u32; 6] = [0x50, 0x54, 0x58, 0x5c, 0x60, 0x64];
        const DESTINATION_COMPONENTS: [u32; 6] = [0x60, 0x64, 0x68, 0x6c, 0x70, 0x74];

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            (address as *const u32).read_unaligned()
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            (address as *mut u32).write_unaligned(value);
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(read_u32(address))
        }

        #[inline(always)]
        unsafe fn write_f32(address: u32, value: f32) {
            write_u32(address, value.to_bits());
        }

        #[inline(always)]
        fn subtract(lhs: f32, rhs: f32) -> f32 {
            core::hint::black_box(lhs) - core::hint::black_box(rhs)
        }

        #[inline(always)]
        fn multiply(lhs: f32, rhs: f32) -> f32 {
            core::hint::black_box(lhs) * core::hint::black_box(rhs)
        }

        #[inline(always)]
        fn add(lhs: f32, rhs: f32) -> f32 {
            core::hint::black_box(lhs) + core::hint::black_box(rhs)
        }

        let alternate_base = read_u32(alternate + ALTERNATE_BASE_FIELD);
        let baseline = if alternate_base != 0 {
            alternate_base.wrapping_add(BASE_ADJUSTMENT)
        } else {
            this.wrapping_add(BASE_ADJUSTMENT)
        };
        let factor = f32::from_bits(factor_bits);

        for index in 0..5 {
            let source_value = read_f32(source + SOURCE_COMPONENTS[index]);
            let baseline_value = read_f32(baseline + BASE_COMPONENTS[index]);
            let difference = subtract(source_value, baseline_value);
            let scaled_difference = multiply(difference, factor);
            let blended = add(scaled_difference, baseline_value);
            write_f32(this + DESTINATION_COMPONENTS[index], blended);
        }

        let source_value = read_f32(source + SOURCE_COMPONENTS[5]);
        let baseline_value = read_f32(baseline + BASE_COMPONENTS[5]);
        let difference = subtract(source_value, baseline_value);
        let scaled_difference = multiply(difference, factor);
        let blended = add(scaled_difference, baseline_value);
        let clamped = if 0.0f32 > blended {
            0.0f32
        } else if blended > 1.0f32 {
            1.0f32
        } else {
            blended
        };
        write_f32(this + DESTINATION_COMPONENTS[5], clamped);
        baseline
    }
});
