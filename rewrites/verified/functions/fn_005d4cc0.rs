// original: 0x005d4cc0 CViewportFrontend3DScene::vf1
//
// Refresh the frontend 3D viewport. The base viewport setup and aspect query
// run first; the query selects one of two image-resident aspect constants.
// The function clears the ready bit, resets its pending interval, and writes
// four view parameters. Signed width and height values are converted to f32
// and multiplied in the order used by the original SSE instructions by four
// factors read through a runtime data pointer. Each result is truncated
// toward zero with cvttss2si's indefinite value for NaN or overflow. The
// six arguments go to the final view helper and its EAX result is returned.
// Convention: thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_005d4cc0(this: u32) -> u32 {
    unsafe {
        const ID_BASE_SETUP: u32 = 1;
        const ID_ASPECT_QUERY: u32 = 2;
        const ID_VIEW_SETUP: u32 = 3;
        const ID_VIEW_SCALE: u32 = 4;
        const AXIS_CONTEXT_VA: u32 = 0x0118_d7f0;
        const WIDE_ASPECT_VA: u32 = 0x00fe_89b8;
        const TALL_ASPECT_VA: u32 = 0x00fe_8934;
        const FACTOR_TABLE_POINTER_VA: u32 = 0x018b_6eb0;
        const WIDTH_SCALE_FAR_OFFSET: u32 = 0x3c;
        const HEIGHT_SCALE_FAR_OFFSET: u32 = 0x38;
        const WIDTH_SCALE_NEAR_OFFSET: u32 = 0x34;
        const HEIGHT_SCALE_NEAR_OFFSET: u32 = 0x30;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn read_image_f32(file_va: u32) -> f32 {
            let bits = unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() };
            f32::from_bits(bits)
        }
        #[inline(always)]
        unsafe fn read_table_f32(table: u32, offset: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(table.wrapping_add(offset)) })
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
            const MIN: f32 = -2147483648.0;
            const UPPER: f32 = 2147483648.0;
            if value.is_nan() || value < MIN || value >= UPPER {
                i32::MIN
            } else {
                core::hint::black_box(value) as i32
            }
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(ID_BASE_SETUP, u32, this);
        let axis_result: u32 = lf_checker_rt::callee_thiscall!(
            ID_ASPECT_QUERY,
            u32,
            lf_checker_rt::relocated(AXIS_CONTEXT_VA)
        );
        let aspect = if axis_result as u8 != 0 {
            read_image_f32(WIDE_ASPECT_VA)
        } else {
            read_image_f32(TALL_ASPECT_VA)
        };

        let flag_address = this.wrapping_add(0x558);
        let flags = (flag_address as *const u8).read();
        (flag_address as *mut u8).write(flags & 0xfe);
        write_u32(this.wrapping_add(0x544), 0x3e8);
        let view = this.wrapping_add(0x10);
        write_u32(this.wrapping_add(0x2c8), 0x4252_0000);
        (this.wrapping_add(0x2cc) as *mut f32).write_unaligned(aspect);
        write_u32(this.wrapping_add(0x2d0), 0x3f00_0000);
        write_u32(this.wrapping_add(0x2d4), 0x4448_0000);
        (this.wrapping_add(0x300) as *mut u8).write(1);
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_VIEW_SETUP, u32, view);

        let width = read_u32(this.wrapping_add(0x2c4)) as i32;
        let height = read_u32(this.wrapping_add(0x2c0)) as i32;
        let width_float = signed_to_f32(width);
        let height_float = signed_to_f32(height);
        let factor_table = lf_checker_rt::global::<u32>(FACTOR_TABLE_POINTER_VA).read_unaligned();
        let width_far = cvttss2si(ordered_mul(
            read_table_f32(factor_table, WIDTH_SCALE_FAR_OFFSET), width_float
        )) as u32;
        let height_far = cvttss2si(ordered_mul(
            read_table_f32(factor_table, HEIGHT_SCALE_FAR_OFFSET), height_float
        )) as u32;
        let width_near = cvttss2si(ordered_mul(
            read_table_f32(factor_table, WIDTH_SCALE_NEAR_OFFSET), width_float
        )) as u32;
        let height_near = cvttss2si(ordered_mul(
            read_table_f32(factor_table, HEIGHT_SCALE_NEAR_OFFSET), height_float
        )) as u32;

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
