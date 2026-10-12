// original: 0x005d49f0 CViewportRadar::vf1
//
// Refresh the radar viewport. The object is passed in ECX. The routine
// selects an aspect ratio from a global mode byte and an input-axis query,
// writes four view parameters and a ready byte, calls the viewport and base
// setup methods, then scales its signed width and height by four runtime
// factors. Each product is converted with truncation toward zero; NaN and
// out-of-range values produce i32::MIN as cvttss2si does. The scaled values,
// zero and 1.0 are passed to the final view helper. Float factors remain
// data loads, and the source-before-factor multiplication order is pinned.
// The helper result remains in EAX. Convention: thiscall, no stack args.
lf_checker_rt::export!(thiscall, rw_005d49f0(this: u32) -> u32 {
    unsafe {
        const ID_AXIS_QUERY: u32 = 1;
        const ID_VIEW_SETUP: u32 = 2;
        const ID_BASE_SETUP: u32 = 3;
        const ID_VIEW_SCALE: u32 = 4;

        const ASPECT_MODE_VA: u32 = 0x0116_09f6;
        const AXIS_CONTEXT_VA: u32 = 0x0118_d7f0;
        const DEFAULT_ASPECT_VA: u32 = 0x00fe_88e8;
        const WIDE_ASPECT_VA: u32 = 0x00fe_89b8;
        const TALL_ASPECT_VA: u32 = 0x00fe_8934;
        const HEIGHT_SCALE_NEAR_VA: u32 = 0x0103_44b8;
        const WIDTH_SCALE_NEAR_VA: u32 = 0x0103_44bc;
        const HEIGHT_SCALE_FAR_VA: u32 = 0x0103_44c0;
        const WIDTH_SCALE_FAR_VA: u32 = 0x0103_44c4;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn read_f32(file_va: u32) -> f32 {
            let bits = unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() };
            f32::from_bits(bits)
        }
        #[inline(always)]
        fn signed_to_f32(value: i32) -> f32 {
            core::hint::black_box(value as f32)
        }
        #[inline(always)]
        fn ordered_mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        fn cvttss2si(value: f32) -> i32 {
            const I32_MIN_F32: f32 = -2147483648.0;
            const I32_UPPER_EXCLUSIVE_F32: f32 = 2147483648.0;
            if value.is_nan() || value < I32_MIN_F32 || value >= I32_UPPER_EXCLUSIVE_F32 {
                i32::MIN
            } else {
                core::hint::black_box(value) as i32
            }
        }

        let mut aspect = read_f32(DEFAULT_ASPECT_VA);
        if lf_checker_rt::global::<u8>(ASPECT_MODE_VA).read() != 0 {
            let axis_result: u32 = lf_checker_rt::callee_thiscall!(
                ID_AXIS_QUERY,
                u32,
                lf_checker_rt::relocated(AXIS_CONTEXT_VA)
            );
            aspect = if axis_result as u8 != 0 {
                read_f32(WIDE_ASPECT_VA)
            } else {
                read_f32(TALL_ASPECT_VA)
            };
        }

        write_u32(this.wrapping_add(0x2c8), 0x4252_0000);
        (this.wrapping_add(0x2cc) as *mut f32).write_unaligned(aspect);
        write_u32(this.wrapping_add(0x2d0), 0x3f00_0000);
        write_u32(this.wrapping_add(0x2d4), 0x4448_0000);
        (this.wrapping_add(0x300) as *mut u8).write(1);
        let view: u32 = this.wrapping_add(0x10);
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_VIEW_SETUP, u32, view);
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_BASE_SETUP, u32, this);
        let flags = (this.wrapping_add(0x558) as *const u8).read();
        (this.wrapping_add(0x558) as *mut u8).write(flags | 1);
        write_u32(this.wrapping_add(0x544), 0x3e8);

        let width = read_u32(this.wrapping_add(0x2c4)) as i32;
        let height = read_u32(this.wrapping_add(0x2c0)) as i32;
        let width_float = signed_to_f32(width);
        let height_float = signed_to_f32(height);
        let width_far = cvttss2si(ordered_mul(width_float, read_f32(WIDTH_SCALE_FAR_VA))) as u32;
        let width_near = cvttss2si(ordered_mul(width_float, read_f32(WIDTH_SCALE_NEAR_VA))) as u32;
        let height_far = cvttss2si(ordered_mul(height_float, read_f32(HEIGHT_SCALE_FAR_VA))) as u32;
        let height_near = cvttss2si(ordered_mul(height_float, read_f32(HEIGHT_SCALE_NEAR_VA))) as u32;

        lf_checker_rt::callee_thiscall!(
            ID_VIEW_SCALE,
            u32,
            view,
            height_near,
            width_near,
            height_far,
            width_far,
            0,
            0x3f80_0000
        )
    }
});
