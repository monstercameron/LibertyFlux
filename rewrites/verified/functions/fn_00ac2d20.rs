// original: 0x00AC2D20 stream_channel_reset (proposed)

/// Reset a streaming channel: rewind it, then release it twice-null.
///
/// The original calls the rewind callee on `this`, then the release callee
/// on `this` with two zero words (thiscall, no stack arguments). It returns
/// the release callee's answer.
lf_checker_rt::export!(thiscall, rw_00AC2D20(this: u32) -> u32 {
    unsafe {
        const REWIND: u32 = 1;
        const RELEASE: u32 = 2;
        lf_checker_rt::callee_thiscall!(REWIND, u32, this);
        lf_checker_rt::callee_thiscall!(RELEASE, u32, this, 0u32, 0u32)
    }
});
