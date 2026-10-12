// original: 0x00DF3240 UIMontageText::format_secondary_scale_label

/// Read the secondary scale through the child at `this + 0x1e4`. A value at
/// or below ten, including an unordered NaN comparison, selects the fixed
/// forty-unit update; a greater value subtracts the mapped five-unit step in
/// `current - step` order. The child setter receives that `f32` and a zero
/// flag. The label object at `this + 0x1fc` then receives the second getter's
/// value promoted exactly to `f64`, formatted and sent through virtual slot
/// `0x1e0`.
lf_checker_rt::export!(thiscall, rw_00df3240(this: u32) -> u32 {
    unsafe {
        const CHILD_FIELD: u32 = 0x1e4;
        const LABEL_FIELD: u32 = 0x1fc;
        const LABEL_SLOT: u32 = 0x1e0;
        const FIVE: u32 = 0x00fe_8ad8;
        const FORMAT_STRING: u32 = 0x00f0_12a4;
        const GET_SCALE: u32 = 1;
        const SET_SCALE: u32 = 2;
        const FORMATTER: u32 = 3;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }

        #[inline(always)]
        unsafe fn get_scale(child: u32) -> f32 {
            lf_checker_rt::callee_thiscall!(GET_SCALE, f32, child)
        }

        #[inline(always)]
        unsafe fn set_scale(child: u32, value: f32) {
            let _ = lf_checker_rt::callee_thiscall!(SET_SCALE, u32, child, value.to_bits(), 0);
        }

        #[inline(always)]
        fn ordered_subtract(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        let child = read_u32(this.wrapping_add(CHILD_FIELD));
        let current_scale = get_scale(child);
        let ten = f32::from_bits(0x4120_0000);
        let update_scale = if current_scale.is_nan() || current_scale <= ten {
            f32::from_bits(0x4220_0000)
        } else {
            let five = read_f32(lf_checker_rt::relocated(FIVE));
            ordered_subtract(current_scale, five)
        };
        set_scale(child, update_scale);

        let label_object = read_u32(this.wrapping_add(LABEL_FIELD));
        let label_table = read_u32(label_object);
        let displayed_scale = get_scale(child);
        let promoted = f64::from(displayed_scale).to_bits();
        let formatted_label = lf_checker_rt::callee_cdecl!(
            FORMATTER,
            u32,
            lf_checker_rt::relocated(FORMAT_STRING),
            promoted as u32,
            (promoted >> 32) as u32,
        );

        let method_address = read_u32(label_table.wrapping_add(LABEL_SLOT));
        let method: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(method_address as usize);
        method(label_object, formatted_label, 0)
    }
});
