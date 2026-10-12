// original: 0x008952B0 audio_set_spatial_vector_1 (proposed)

/// Store a spatial vector through channel A's voice writer.
///
/// Passes the three stack words (`a0`, `a1`, and the float bits `a2`)
/// unchanged to the channel-A writer (patched callee) with `this + 0x550`.
/// Returns the callee's result.
///
/// Original: 0x008952B0 (thiscall: `this` in ecx, three stack words).
lf_checker_rt::export!(thiscall, rw_008952b0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const CHANNEL_A_WRITER: u32 = 1;
        const WRITER_OFF: u32 = 0x550;

        lf_checker_rt::callee_thiscall!(CHANNEL_A_WRITER, u32, this.wrapping_add(WRITER_OFF), a0, a1, a2)
    }
});
