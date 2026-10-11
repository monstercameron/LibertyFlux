// original: 0x00CF01A0 vehicle_decrement_counter_12ec
//! Scales a global rate, converts it with x87 round-toward-zero semantics,
//! and uses the low word as an unsigned decrement of the timer at `0x12EC`.
//!
//! The decrement is clamped to at least one. If the timer is greater than the
//! decrement, the stored value and EAX are the subtraction result. Otherwise
//! the field is cleared while EAX retains the timer value loaded before the
//! branch. NaN, infinity, and out-of-range conversion use the x87 signed
//! indefinite result. The function uses thiscall with no stack arguments.
const TIMER_FIELD: usize = 0x12EC;
const RATE_INPUT: u32 = 0x011735BC;
const RATE_SCALE: u32 = 0x00EDDB34;
const I64_MIN_F32: f32 = -9_223_372_036_854_775_808.0;
const I64_POS_LIMIT_F32: f32 = 9_223_372_036_854_775_808.0;

#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00cf01a0(this: *mut u8) -> u32 {
    let rate_input = unsafe {
        core::ptr::read_unaligned(lf_checker_rt::global::<f32>(RATE_INPUT))
    };
    let rate_scale = unsafe {
        core::ptr::read_unaligned(lf_checker_rt::global::<f32>(RATE_SCALE))
    };
    let scaled_rate = core::hint::black_box(
        core::hint::black_box(rate_input) * core::hint::black_box(rate_scale),
    );
    let converted = if !scaled_rate.is_finite()
        || scaled_rate < I64_MIN_F32
        || scaled_rate >= I64_POS_LIMIT_F32
    {
        i64::MIN
    } else {
        scaled_rate as i64
    };
    let decrement = (converted as u64 as u32).max(1);
    let timer = unsafe {
        core::ptr::read_unaligned(this.add(TIMER_FIELD).cast::<u32>())
    };
    let (remaining, return_value) = if timer > decrement {
        let after_decrement = timer.wrapping_sub(decrement);
        (after_decrement, after_decrement)
    } else {
        (0, timer)
    };
    unsafe {
        core::ptr::write_unaligned(this.add(TIMER_FIELD).cast::<u32>(), remaining);
    }
    return_value
}
