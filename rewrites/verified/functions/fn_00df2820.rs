// original: 0x00DF2820 UIMontageText::format_duration_label

/// Format the time stored as an `f32` at `this + 0x210` and pass the result
/// to the display object at `this + 0x20c`. The ordered thresholds are 10,
/// 1, and 0.25 seconds. The first two `JB` branches also take unordered
/// values; the final `JBE` selects the quarter-second label for equality or
/// an unordered value. The two upper formats promote the original `f32` to
/// `f64` before calling the formatter. All strings and threshold values are
/// read from their mapped image locations, and the display update remains a
/// virtual call through slot `0x1e0`.
lf_checker_rt::export!(thiscall, rw_00df2820(this: u32) -> u32 {
    unsafe {
        const TIME_VALUE: u32 = 0x210;
        const DISPLAY_OBJECT: u32 = 0x20c;
        const DISPLAY_SLOT: u32 = 0x1e0;
        const FORMAT_OBJECT: u32 = 0x0116_bff0;
        const TEN_SECONDS: u32 = 0x00fe_8b08;
        const ONE_SECOND: u32 = 0x00fe_88e8;
        const QUARTER_SECOND: u32 = 0x00fe_87e4;
        const UNIT_KEY_TEN: u32 = 0x00f0_1378;
        const FORMAT_TEN: u32 = 0x00f0_1384;
        const UNIT_KEY_ONE: u32 = 0x00f0_1390;
        const FORMAT_ONE: u32 = 0x00f0_139c;
        const UNIT_KEY_HALF: u32 = 0x00f0_13a8;
        const FORMAT_HALF: u32 = 0x00f0_13b4;
        const UNIT_KEY_QUARTER: u32 = 0x00f0_13bc;
        const FORMAT_QUARTER: u32 = 0x00f0_13c8;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }

        #[inline(always)]
        fn below_or_unordered(value: f32, limit: f32) -> bool {
            value.is_nan() || value < limit
        }

        #[inline(always)]
        unsafe fn lookup_unit(key: u32) -> u32 {
            lf_checker_rt::callee_thiscall!(
                1,
                u32,
                lf_checker_rt::relocated(FORMAT_OBJECT),
                lf_checker_rt::relocated(key),
            )
        }

        #[inline(always)]
        unsafe fn format_promoted(value: f32, format: u32, unit: u32) -> u32 {
            let promoted = f64::from(value).to_bits();
            lf_checker_rt::callee_cdecl!(
                2,
                u32,
                lf_checker_rt::relocated(format),
                promoted as u32,
                (promoted >> 32) as u32,
                unit,
            )
        }

        #[inline(always)]
        unsafe fn format_fixed(format: u32, unit: u32) -> u32 {
            lf_checker_rt::callee_cdecl!(
                3,
                u32,
                lf_checker_rt::relocated(format),
                unit,
            )
        }

        #[inline(always)]
        unsafe fn update_display(display: u32, table: u32, formatted: u32) -> u32 {
            unsafe {
                let address = read_u32(table.wrapping_add(DISPLAY_SLOT));
                let method: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(address as usize);
                method(display, formatted, 0)
            }
        }

        let seconds = read_f32(this.wrapping_add(TIME_VALUE));
        let initial_display = read_u32(this.wrapping_add(DISPLAY_OBJECT));
        let display_table = read_u32(initial_display);

        if !below_or_unordered(seconds, read_f32(lf_checker_rt::relocated(TEN_SECONDS))) {
            let unit = lookup_unit(UNIT_KEY_TEN);
            let formatted = format_promoted(
                read_f32(this.wrapping_add(TIME_VALUE)),
                FORMAT_TEN,
                unit,
            );
            update_display(
                read_u32(this.wrapping_add(DISPLAY_OBJECT)),
                display_table,
                formatted,
            )
        } else if !below_or_unordered(seconds, read_f32(lf_checker_rt::relocated(ONE_SECOND))) {
            let unit = lookup_unit(UNIT_KEY_ONE);
            let formatted = format_promoted(
                read_f32(this.wrapping_add(TIME_VALUE)),
                FORMAT_ONE,
                unit,
            );
            update_display(
                read_u32(this.wrapping_add(DISPLAY_OBJECT)),
                display_table,
                formatted,
            )
        } else if seconds > read_f32(lf_checker_rt::relocated(QUARTER_SECOND)) {
            let unit = lookup_unit(UNIT_KEY_HALF);
            let formatted = format_fixed(FORMAT_HALF, unit);
            update_display(
                read_u32(this.wrapping_add(DISPLAY_OBJECT)),
                display_table,
                formatted,
            )
        } else {
            let unit = lookup_unit(UNIT_KEY_QUARTER);
            let formatted = format_fixed(FORMAT_QUARTER, unit);
            update_display(
                read_u32(this.wrapping_add(DISPLAY_OBJECT)),
                display_table,
                formatted,
            )
        }
    }
});
