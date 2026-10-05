// original: 0x009BA310 CCamScriptInstruction_SetHintAdvancedParams::vf2
/// Forward four floats and a flag byte to the hint camera's advanced-params setter.
///
/// Looks up the camera (`callee 1`), then calls `callee 2` (thiscall/5) with
/// the floats at `this+0x08..0x14` as raw bits and the zero-extended byte at
/// `this+0x18`. No null check on the lookup.
lf_checker_rt::export!(thiscall, rw_009BA310(this: u32) -> u32 {
    unsafe {
        const HINT_MGR: u32 = 0x103E498;
        const LOOKUP: u32 = 1;
        const SET_ADVANCED: u32 = 2;
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(HINT_MGR));
        let w0 = (this.wrapping_add(0x08) as *const u32).read_unaligned();
        let w1 = (this.wrapping_add(0x0C) as *const u32).read_unaligned();
        let w2 = (this.wrapping_add(0x10) as *const u32).read_unaligned();
        let w3 = (this.wrapping_add(0x14) as *const u32).read_unaligned();
        let flag = (this.wrapping_add(0x18) as *const u8).read() as u32;
        lf_checker_rt::callee_thiscall!(SET_ADVANCED, u32, cam, w0, w1, w2, w3, flag);
        0
    }
});
