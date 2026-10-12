// original: 0x005d4b80 CViewportMobilePhone::vf1
//
// Refresh the mobile-phone viewport. The base viewport setup runs first,
// then the ready bit is cleared and the pending interval is set to 1000.
// Four view parameters and the ready byte are written before the view setup
// helper. Signed width and height values are converted to f32, multiplied
// in the source order by image constants, and truncated toward zero. The
// results and two constants are passed to the final view helper; its EAX
// result is returned. Convention: thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_005d4b80(this: u32) -> u32 {
    unsafe {
        const ID_BASE_SETUP: u32 = 1;
        const ID_VIEW_SETUP: u32 = 2;
        const ID_VIEW_SCALE: u32 = 3;
        const WIDTH_HALF_VA: u32 = 0x00fe_8830;
        const HEIGHT_TWO_FIFTHS_VA: u32 = 0x00fe_881c;
        const HEIGHT_THREE_FIFTHS_VA: u32 = 0x00fe_8858;

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
        let flag_address = this.wrapping_add(0x558);
        let flags = (flag_address as *const u8).read();
        (flag_address as *mut u8).write(flags & 0xfe);
        write_u32(this.wrapping_add(0x544), 0x3e8);

        let view = this.wrapping_add(0x10);
        write_u32(this.wrapping_add(0x2c8), 0x428c_0000);
        write_u32(this.wrapping_add(0x2cc), 0x3f40_0000);
        write_u32(this.wrapping_add(0x2d0), 0x3c23_d70a);
        write_u32(this.wrapping_add(0x2d4), 0x442f_0000);
        (this.wrapping_add(0x300) as *mut u8).write(1);
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_VIEW_SETUP, u32, view);

        let width = read_u32(this.wrapping_add(0x2c4)) as i32;
        let height = read_u32(this.wrapping_add(0x2c0)) as i32;
        let width_float = signed_to_f32(width);
        let height_float = signed_to_f32(height);
        let width_half = cvttss2si(ordered_mul(width_float, read_image_f32(WIDTH_HALF_VA))) as u32;
        let height_two_fifths = cvttss2si(
            ordered_mul(height_float, read_image_f32(HEIGHT_TWO_FIFTHS_VA))
        ) as u32;
        let height_three_fifths = cvttss2si(
            ordered_mul(height_float, read_image_f32(HEIGHT_THREE_FIFTHS_VA))
        ) as u32;

        lf_checker_rt::callee_thiscall!(
            ID_VIEW_SCALE,
            u32,
            view,
            height_three_fifths,
            width_half,
            height_two_fifths,
            width_half,
            0,
            0x3f80_0000
        )
    }
});
