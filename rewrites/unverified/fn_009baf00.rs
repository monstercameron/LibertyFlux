// original: 0x009BAF00 CCamScriptInstruction_SetSniperZoomFactor::vf2
/// Store the sniper-zoom operand into the game camera object.
///
/// Looks up the camera (`callee 1`) and copies the word at `this+0x08` to
/// camera `+0x1BC`. No null check on the lookup.
lf_checker_rt::export!(thiscall, rw_009BAF00(this: u32) -> u32 {
    unsafe {
        const HINT_MGR: u32 = 0x103E498;
        const OPERAND: u32 = 0x08;
        const ZOOM: u32 = 0x1BC;
        const LOOKUP: u32 = 1;
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(HINT_MGR));
        let v = (this.wrapping_add(OPERAND) as *const u32).read_unaligned();
        (cam.wrapping_add(ZOOM) as *mut u32).write_unaligned(v);
        0
    }
});
