// original: 0x009B9A20 CCamScriptInstruction_Restore_Q::vf2

/// Execute the Restore_Q script instruction: drive the camera manager
/// singleton through a fixed eight-call restore sequence (callees 1-6).
///
/// Callee 3 takes (1, fade-parameter-global); its answer is forwarded to
/// callee 4's first call, callee 5's answer to callee 4's second call;
/// callee 2 fires twice with ((1,1), then (0,0)); the last step is a tail
/// jump (the rewrite issues it as a normal call). Reads no instruction
/// fields. No return value (thiscall).
lf_checker_rt::export!(thiscall, rw_009B9A20(this: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0128_E400;
        const PARAM: u32 = 0x0128_E928;
        let _ = this;
        let m = lf_checker_rt::relocated(MGR);
        let param = lf_checker_rt::global::<u32>(PARAM).read();
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, m);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, m, 1, 1);
        let first: u32 = lf_checker_rt::callee_thiscall!(3, u32, m, 1, param);
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, m, first);
        let second: u32 = lf_checker_rt::callee_thiscall!(5, u32, m);
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, m, second);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, m, 0, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, m);
        0
    }
});
