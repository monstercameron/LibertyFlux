// original: 0x009B9600 CCamScriptInstruction_DestroyAllCams::vf2

/// Execute the DestroyAllCams script instruction: fetch the cam
/// registry from the global camera context (callee 1 with flag 1) and
/// destroy every cam in it (callee 2, no arguments).
///
/// Reads no instruction fields. No return value (thiscall).
lf_checker_rt::export!(thiscall, rw_009B9600(this: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x0103_E498;
        let _ = this;
        let reg: u32 = lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(CTX), 1);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, reg);
        0
    }
});
