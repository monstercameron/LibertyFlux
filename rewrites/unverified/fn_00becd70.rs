// original: 0x00BECD70 interp_float_track_eval (proposed)

/// Evaluate one compact float track into its destination record.
///
/// `this` points to the source record, `out` to a destination with fields at
/// the named offsets below, `other` to an optional second record, and
/// `weight_bits` is the raw single-precision interpolation weight. A null
/// `other` or either signed zero selects the source-only path. Otherwise the
/// byte channels are interpolated with the original signedness per field,
/// while the curve channel comes from the scripted piecewise-blend callee.
/// Float operands stay in the original subtract, multiply, add order; the
/// image constants are read from the relocated data image.
///
/// The entry is thiscall: `this` is in ECX, three words follow on the stack,
/// and the function returns `out` in EAX.
lf_checker_rt::export!(thiscall, rw_00becd70(this: u32, out: u32, other: u32, weight_bits: u32) -> u32 {
    unsafe {
        const SOURCE_X: u32 = 0x04;
        const SOURCE_Y: u32 = 0x08;
        const SOURCE_CURVE: u32 = 0x03;
        const SOURCE_ROTATION: u32 = 0x02;
        const SOURCE_A: u32 = 0x01;
        const SOURCE_B: u32 = 0x0c;
        const SOURCE_C: u32 = 0x0d;
        const SOURCE_D: u32 = 0x0e;
        const SOURCE_E: u32 = 0x0f;
        const SOURCE_F: u32 = 0x10;
        const SOURCE_G: u32 = 0x11;

        const OUT_D: u32 = 0x70;
        const OUT_E: u32 = 0x74;
        const OUT_A: u32 = 0x78;
        const OUT_CURVE: u32 = 0x7c;
        const OUT_F: u32 = 0x80;
        const OUT_G: u32 = 0x130;
        const OUT_ROTATION: u32 = 0x13c;
        const OUT_B: u32 = 0x140;
        const OUT_C: u32 = 0x158;
        const OUT_X: u32 = 0x15c;
        const OUT_Y: u32 = 0x160;

        const PIECEWISE_BLEND: u32 = 1;
        const SCALE_ROTATION_VA: u32 = 0x00FE8700;
        const SCALE_A_VA: u32 = 0x00FE8734;
        const SCALE_CURVE_VA: u32 = 0x00EBA938;
        const SCALE_B_VA: u32 = 0x00EBA934;
        const SCALE_C_VA: u32 = 0x00EBA930;
        const SCALE_F_VA: u32 = 0x00EBA944;
        const SCALE_G_VA: u32 = 0x00EBA93C;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_i8(address: u32) -> i8 {
            unsafe { (address as *const i8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }
        #[inline(always)]
        unsafe fn write_f32(address: u32, value: f32) {
            unsafe { write_u32(address, value.to_bits()) }
        }
        #[inline(always)]
        unsafe fn image_f32(file_va: u32) -> f32 {
            unsafe { lf_checker_rt::global::<f32>(file_va).read_unaligned() }
        }
        #[inline(always)]
        fn add_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)]
        fn sub_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        fn mul_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        fn blend_f32(source: f32, target: f32, weight: f32) -> f32 {
            add_f32(mul_f32(sub_f32(target, source), weight), source)
        }
        #[inline(always)]
        fn blend_i8(source: i8, target: i8, weight: f32) -> f32 {
            blend_f32(f32::from(source), f32::from(target), weight)
        }
        #[inline(always)]
        fn blend_i8_u8(source: i8, target: u8, weight: f32) -> f32 {
            blend_f32(f32::from(source), f32::from(target), weight)
        }
        #[inline(always)]
        fn blend_u8(source: u8, target: u8, weight: f32) -> f32 {
            blend_f32(f32::from(source), f32::from(target), weight)
        }

        let weight = f32::from_bits(weight_bits);
        let interpolate = other != 0 && weight != 0.0;
        let curve = if interpolate {
            let curve_delta = i32::from(read_i8(this + SOURCE_CURVE))
                - i32::from(read_i8(other + SOURCE_CURVE));
            let source_curve_raw = u32::from(read_u8(this + SOURCE_CURVE));
            let target_curve_raw = u32::from(read_u8(other + SOURCE_CURVE));
            lf_checker_rt::callee_cdecl!(
                PIECEWISE_BLEND,
                f32,
                (curve_delta as f32).to_bits(),
                source_curve_raw,
                target_curve_raw,
                weight_bits,
            )
        } else {
            f32::from(read_i8(this + SOURCE_CURVE))
        };

        let x_source = read_f32(this + SOURCE_X);
        let y_source = read_f32(this + SOURCE_Y);
        let (x, y) = if interpolate {
            let x_target = read_f32(other + SOURCE_X);
            let y_target = read_f32(other + SOURCE_Y);
            (blend_f32(x_source, x_target, weight), blend_f32(y_source, y_target, weight))
        } else {
            (x_source, y_source)
        };

        let byte_a = if interpolate {
            blend_i8(read_i8(this + SOURCE_A), read_i8(other + SOURCE_A), weight)
        } else {
            f32::from(read_i8(this + SOURCE_A))
        };
        let byte_rotation = if interpolate {
            blend_i8(read_i8(this + SOURCE_ROTATION), read_i8(other + SOURCE_ROTATION), weight)
        } else {
            f32::from(read_i8(this + SOURCE_ROTATION))
        };
        let byte_b = if interpolate {
            blend_u8(read_u8(this + SOURCE_B), read_u8(other + SOURCE_B), weight)
        } else {
            f32::from(read_u8(this + SOURCE_B))
        };
        let byte_c = if interpolate {
            blend_u8(read_u8(this + SOURCE_C), read_u8(other + SOURCE_C), weight)
        } else {
            f32::from(read_u8(this + SOURCE_C))
        };
        let byte_d = if interpolate {
            blend_i8_u8(read_i8(this + SOURCE_D), read_u8(other + SOURCE_D), weight)
        } else {
            f32::from(read_i8(this + SOURCE_D))
        };
        let byte_e = if interpolate {
            blend_i8_u8(read_i8(this + SOURCE_E), read_u8(other + SOURCE_E), weight)
        } else {
            f32::from(read_i8(this + SOURCE_E))
        };
        let byte_f = f32::from(read_i8(this + SOURCE_F));
        let byte_g = if interpolate {
            blend_u8(read_u8(this + SOURCE_G), read_u8(other + SOURCE_G), weight)
        } else {
            f32::from(read_u8(this + SOURCE_G))
        };

        let rotation = if interpolate {
            mul_f32(byte_rotation, image_f32(SCALE_ROTATION_VA))
        } else {
            mul_f32(byte_rotation, image_f32(SCALE_ROTATION_VA))
        };
        let scaled_a = mul_f32(byte_a, image_f32(SCALE_A_VA));
        let scaled_curve = mul_f32(curve, image_f32(SCALE_CURVE_VA));
        let scaled_b = mul_f32(byte_b, image_f32(SCALE_B_VA));
        let scaled_c = mul_f32(byte_c, image_f32(SCALE_C_VA));
        let scaled_f = mul_f32(byte_f, image_f32(SCALE_F_VA));
        let scaled_g = mul_f32(byte_g, image_f32(SCALE_G_VA));

        write_f32(out + OUT_D, byte_d);
        write_f32(out + OUT_E, byte_e);
        write_f32(out + OUT_A, scaled_a);
        write_f32(out + OUT_CURVE, scaled_curve);
        write_f32(out + OUT_F, scaled_f);
        write_f32(out + OUT_G, scaled_g);
        write_f32(out + OUT_ROTATION, rotation);
        write_f32(out + OUT_B, scaled_b);
        write_f32(out + OUT_C, scaled_c);
        write_f32(out + OUT_X, x);
        write_f32(out + OUT_Y, y);

        out
    }
});
