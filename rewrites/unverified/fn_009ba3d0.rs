// original: 0x009BA3D0 CCamScriptInstruction_SetHintTimes::vf2
/// Store the two hint-time operands into the hint camera object.
///
/// Looks up the camera (`callee 1`) and copies the words at `this+0x08` and
/// `this+0x10` to camera `+0x194` and `+0x198`. No null check on the lookup.
lf_checker_rt::export!(thiscall, rw_009BA3D0(this: u32) -> u32 {
    unsafe {
        const HINT_MGR: u32 = 0x103E498;
        const TIME0: u32 = 0x08;
        const TIME1: u32 = 0x10;
        const DST0: u32 = 0x194;
        const DST1: u32 = 0x198;
        const LOOKUP: u32 = 1;
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(HINT_MGR));
        let v0 = (this.wrapping_add(TIME0) as *const u32).read_unaligned();
        let v1 = (this.wrapping_add(TIME1) as *const u32).read_unaligned();
        (cam.wrapping_add(DST0) as *mut u32).write_unaligned(v0);
        (cam.wrapping_add(DST1) as *mut u32).write_unaligned(v1);
        0
    }
});
