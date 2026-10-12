// original: 0x00A86010 CAnimTaskInfo::vf4

/// Serialize the animation task's rate, flags, and timing fields in their observed order.
///
/// The two rates are single precision; the speed is multiplied by the relocated
/// image factor and truncated through the low word of an x87 64-bit integer store.
/// NaN, infinities, and values outside the signed 64-bit range produce a zero low word.
/// `this` is the task record and `stream` is the serializer context.
lf_checker_rt::export!(thiscall, rw_00a86010(this: u32, stream: u32) -> u32 {
    unsafe {
        const RATE_OFFSET: u32 = 0x20;
        const SPEED_OFFSET: u32 = 0x24;
        const FLAG_OFFSET: u32 = 0x28;
        const CLIP_MODE_OFFSET: u32 = 0x29;
        const STEP_OFFSET: u32 = 0x14;
        const GROUP_OFFSET: u32 = 0x18;
        const EXTRA_OFFSET: u32 = 0x1c;
        const SCALE_FACTOR_VA: u32 = 0x00ea2004;
        const RATE_TAG: u32 = 0x08;
        const SPEED_TAG: u32 = 0x08;
        const FLAG_TAG: u32 = 0x03;
        const EXTRA_TAG: u32 = 0x1f;
        const I64_UPPER_EXCLUSIVE: f32 = 9_223_372_036_854_775_808.0f32;
        const I64_LOWER_INCLUSIVE: f32 = -9_223_372_036_854_775_808.0f32;
        #[inline(always)] unsafe fn rd8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
        #[inline(always)] unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)] fn multiply_f32(left: f32, right: f32) -> f32 {
            let left = core::hint::black_box(left);
            let right = core::hint::black_box(right);
            left * right
        }
        // The source uses FISTP qword with rounding set to truncate. Invalid
        // conversions store the x87 indefinite qword, whose low word is zero.
        #[inline(always)] fn fistp_low_word(value: f32) -> u32 {
            if !value.is_finite() || value >= I64_UPPER_EXCLUSIVE || value < I64_LOWER_INCLUSIVE {
                0
            } else {
                (value.trunc() as i64) as u32
            }
        }
        let animation_rate = f32::from_bits(unsafe { rd32(this.wrapping_add(RATE_OFFSET)) });
        let quantized_rate = lf_checker_rt::callee_cdecl!(1, u32, animation_rate.to_bits());
        let step = unsafe { rd32(this.wrapping_add(STEP_OFFSET)) };
        let _step = lf_checker_rt::callee_thiscall!(2, u32, stream, step, 0x10u32);
        let group = unsafe { rd32(this.wrapping_add(GROUP_OFFSET)) };
        let _group = lf_checker_rt::callee_thiscall!(2, u32, stream, group, 0x10u32);
        let _rate = lf_checker_rt::callee_thiscall!(2, u32, stream, quantized_rate, RATE_TAG);
        let clip_mode = unsafe { rd8(this.wrapping_add(CLIP_MODE_OFFSET)) } & 1;
        let _clip_mode = lf_checker_rt::callee_thiscall!(3, u32, stream, u32::from(clip_mode));
        let speed = f32::from_bits(unsafe { rd32(this.wrapping_add(SPEED_OFFSET)) });
        // This is the image-backed factor at the operand used by the original.
        let scale_factor = unsafe { lf_checker_rt::global::<f32>(SCALE_FACTOR_VA).read_unaligned() };
        let scaled_speed = multiply_f32(speed, scale_factor);
        let flag = unsafe { rd8(this.wrapping_add(FLAG_OFFSET)) };
        let _flag = lf_checker_rt::callee_thiscall!(2, u32, stream, u32::from(flag), FLAG_TAG);
        let integral_speed = fistp_low_word(scaled_speed);
        let _integral_speed = lf_checker_rt::callee_thiscall!(2, u32, stream, integral_speed, SPEED_TAG);
        let extra = unsafe { rd32(this.wrapping_add(EXTRA_OFFSET)) };
        lf_checker_rt::callee_thiscall!(2, u32, stream, extra, EXTRA_TAG)
    }
});
