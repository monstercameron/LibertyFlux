// original: 0x009B99D0 CCamScriptInstruction_RestoreJumpCut_Q::vf2

/// Execute the RestoreJumpCut_Q script instruction: drive the camera
/// manager singleton through a fixed six-call restore sequence (callees
/// 1-5, all thiscall with the manager as `this`).
///
/// Callee 2 fires twice with different flag pairs ((1,1), then (0,0));
/// callee 3's answer is forwarded as the argument of callee 4; the last
/// step is a tail jump (the rewrite issues it as a normal call). Reads no
/// instruction fields. No return value (thiscall).
lf_checker_rt::export!(thiscall, rw_009B99D0(this: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0128_E400;
        let _ = this;
        let m = lf_checker_rt::relocated(MGR);
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, m);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, m, 1, 1);
        let token: u32 = lf_checker_rt::callee_thiscall!(3, u32, m);
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, m, token);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, m, 0, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, m);
        0
    }
});
