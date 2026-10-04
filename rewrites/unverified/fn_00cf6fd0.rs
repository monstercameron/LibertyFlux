// original: 0x00cf6fd0 climb_task_effect_chain (proposed)

/// Runs the climb task's effect chain: looks up the effect for the word at
/// `+0x7c` through the controller at `+0x78` of the target (returning null
/// when there is none), resolves its parameter, dispatches the 0xce00/3
/// effect call, applies the float stage at `+0x4c` when dispatch succeeds,
/// and always runs the finaliser with -999.0f, returning its result.
///
/// Original: 0x00cf6fd0 (thiscall: ecx holds the object, one stack word).
lf_checker_rt::export!(thiscall, rw_00cf6fd0(this: u32, target: u32) -> u32 {
    unsafe {
        const LOOKUP_CALLEE: u32 = 1;
        const PARAM_CALLEE: u32 = 2;
        const DISPATCH_CALLEE: u32 = 3;
        const STAGE_CALLEE: u32 = 4;
        const FINAL_CALLEE: u32 = 5;
        const EFFECT_ID: u32 = 0xce00;
        const FINAL_FLOAT: u32 = 0xc47a_0000; // -999.0f
        let ctl = ((target + 0x78) as *const u32).read_unaligned();
        let key = ((this + 0x7c) as *const u32).read_unaligned();
        let effect = lf_checker_rt::callee_thiscall!(LOOKUP_CALLEE, u32, ctl, key);
        if effect == 0 {
            return 0;
        }
        let ep = ((effect + 0x10) as *const u32).read_unaligned();
        let param: u32 = lf_checker_rt::callee_cdecl!(PARAM_CALLEE, u32, ep);
        let ctl2 = ((target + 0x78) as *const u32).read_unaligned();
        let e18 = ((effect + 0x18) as *const u32).read_unaligned();
        let disp = lf_checker_rt::callee_thiscall!(DISPATCH_CALLEE, u32, ctl2, param, e18, EFFECT_ID, 3);
        if disp != 0 {
            let f = ((effect + 0x4c) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(STAGE_CALLEE, u32, disp, f);
        }
        lf_checker_rt::callee_thiscall!(FINAL_CALLEE, u32, effect, FINAL_FLOAT)
    }
});
