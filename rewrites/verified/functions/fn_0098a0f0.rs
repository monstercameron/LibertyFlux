// original: 0x0098A0F0 fire_audio_cooldown_gain
/// Updates the fire-audio cooldown state and returns the current gain.
///
/// With an active cooldown, the current time refreshes the primary timestamp.
/// Before its unsigned deadline, the stored gain moves by the image-resident
/// delta and is clamped to the image-resident limit; after the deadline, the
/// active byte clears and the secondary timestamp and gain reset. With the
/// active byte clear, the secondary unsigned deadline can set it, and the
/// primary timestamp and gain are refreshed. The returned float uses the
/// x87 single-precision return convention.
const ACTIVE_BYTE_VA: u32 = 0x0128_3141;
const PRIMARY_TIME_VA: u32 = 0x0128_31C8;
const SECONDARY_TIME_VA: u32 = 0x0128_31CC;
const STORED_GAIN_VA: u32 = 0x0128_31D0;
const CURRENT_TIME_VA: u32 = 0x0117_35B4;
const ACTIVE_PERIOD_VA: u32 = 0x0103_8B78;
const INACTIVE_PERIOD_VA: u32 = 0x0103_8B7C;
const GAIN_DELTA_VA: u32 = 0x0103_8B80;
const GAIN_LIMIT_VA: u32 = 0x0103_8B74;

export!(cdecl, rw_0098a0f0() -> f32 {
    let active = unsafe { lf_checker_rt::global::<u8>(ACTIVE_BYTE_VA).read() };
    let now = unsafe { lf_checker_rt::global::<u32>(CURRENT_TIME_VA).read() };

    if active != 0 {
        let period = unsafe { (lf_checker_rt::relocated(ACTIVE_PERIOD_VA) as *const u32).read() };
        let last_time = unsafe { lf_checker_rt::global::<u32>(PRIMARY_TIME_VA).read() };
        let deadline = period.wrapping_add(last_time);
        unsafe { lf_checker_rt::global::<u32>(PRIMARY_TIME_VA).write(now); }

        if deadline >= now {
            let delta_bits = unsafe { (lf_checker_rt::relocated(GAIN_DELTA_VA) as *const u32).read() };
            let delta = f32::from_bits(delta_bits);
            let stored_bits = unsafe { lf_checker_rt::global::<u32>(STORED_GAIN_VA).read() };
            let stored_gain = f32::from_bits(stored_bits);
            let candidate = core::hint::black_box(delta) + core::hint::black_box(stored_gain);
            let limit_bits = unsafe { (lf_checker_rt::relocated(GAIN_LIMIT_VA) as *const u32).read() };
            let limit = f32::from_bits(limit_bits);
            let result = if candidate > limit { candidate } else { limit };
            unsafe { lf_checker_rt::global::<u32>(STORED_GAIN_VA).write(result.to_bits()); }
            result
        } else {
            unsafe {
                lf_checker_rt::global::<u8>(ACTIVE_BYTE_VA).write(0);
                lf_checker_rt::global::<u32>(SECONDARY_TIME_VA).write(now);
                lf_checker_rt::global::<u32>(STORED_GAIN_VA).write(0);
            }
            0.0
        }
    } else {
        let period = unsafe { (lf_checker_rt::relocated(INACTIVE_PERIOD_VA) as *const u32).read() };
        let last_time = unsafe { lf_checker_rt::global::<u32>(SECONDARY_TIME_VA).read() };
        let deadline = period.wrapping_add(last_time);
        let should_activate = deadline < now;
        unsafe {
            lf_checker_rt::global::<u32>(PRIMARY_TIME_VA).write(now);
            lf_checker_rt::global::<u8>(ACTIVE_BYTE_VA).write(should_activate as u8);
            lf_checker_rt::global::<u32>(STORED_GAIN_VA).write(0);
        }
        0.0
    }
});
