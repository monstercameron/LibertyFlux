// original: 0x009BA280 CCamScriptInstruction_SetGameCameraControlsActive::vf2
/// Set or clear the controls-active flag (bit 3) on the game camera object.
///
/// The byte operand at `this+0x08` selects the polarity: a zero operand sets
/// bit 3 of the flag byte at camera `+0x1C4`, a nonzero operand clears it;
/// all other bits are preserved. The original has no null check on the
/// lookup, so the camera pointer is always used.
lf_checker_rt::export!(thiscall, rw_009BA280(this: u32) -> u32 {
    unsafe {
        const HINT_MGR: u32 = 0x103E498;
        const OPERAND: u32 = 0x08;
        const FLAGS: u32 = 0x1C4;
        const CONTROLS_ACTIVE: u8 = 0x08;
        const LOOKUP: u32 = 1;
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(HINT_MGR));
        let sel = (this.wrapping_add(OPERAND) as *const u8).read();
        let slot = cam.wrapping_add(FLAGS) as *mut u8;
        let old = slot.read();
        slot.write(if sel == 0 { old | CONTROLS_ACTIVE } else { old & !CONTROLS_ACTIVE });
        0
    }
});
