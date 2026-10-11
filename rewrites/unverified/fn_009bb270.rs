// original: 0x009BB270 CCamScriptInstruction_SetWidescreenBordersInstant::vf2

/// Set the widescreen border state immediately.
///
/// The handler takes the low byte at `this + 0x08`, zero extends it, and calls
/// the border helper with the global settings object and a zero duration. The
/// helper's EAX result is returned.
lf_checker_rt::export!(thiscall, rw_009bb270(this: u32) -> u32 {
    unsafe {
        const SETTINGS: u32 = 0x0118_d7f0;
        const SET_BORDERS: u32 = 1;
        let enabled = u32::from((this.wrapping_add(8) as *const u8).read());
        lf_checker_rt::callee_thiscall!(
            SET_BORDERS,
            u32,
            lf_checker_rt::relocated(SETTINGS),
            enabled,
            0,
        )
    }
});
