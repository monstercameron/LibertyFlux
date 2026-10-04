// original: 0x00a96290 fade_alpha_or_tail
/// Fade alpha gate: returns 0 unless the channel is idle, then tails into
/// the shared alpha reader.
///
/// When the readiness probe reports false the progress-dirty flag must be
/// set or the result is 0; a non-idle state word also forces 0. Otherwise
/// the call is forwarded to the shared reader with the same channel.
export!(thiscall, rw_00a96290(this: u32) -> u32 {
    unsafe {
        if callee_thiscall!(1, u32, this) & 0xFF == 0 {
            if *((this + 0x20) as *const u8) & 4 == 0 {
                return 0;
            }
        }
        if *(this as *const u32) != 0 {
            return 0;
        }
        callee_thiscall!(2, u32, this)
    }
});
