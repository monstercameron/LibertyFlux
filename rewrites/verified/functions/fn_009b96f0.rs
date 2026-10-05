// original: 0x009B96F0 CCamScriptInstruction_DoFadeOut_Q::vf2

/// Execute the DoFadeOut_Q script instruction: run one nine-argument fade-setup
/// call (callee 1) against the fade context at global+0x424, fading out.
///
/// Arguments are constants except arg1 (the fade parameter global), arg4 (a pointer to a
/// scratch word holding 0xFF000000, skipped as an address with its target
/// snapshotted instead) and the two 1.0f words. The callee's answer is
/// ignored. No return value (thiscall).
lf_checker_rt::export!(thiscall, rw_009B96F0(this: u32) -> u32 {
    unsafe {
        const CTX_PTR: u32 = 0x0118_D804;
        const PARAM: u32 = 0x0128_E928;
        const CTX_BIAS: u32 = 0x424;
        const ONE: u32 = 0x3F80_0000;
        const MARKER: u32 = 0xFF00_0000;
        let ctx = lf_checker_rt::global::<u32>(CTX_PTR).read();
        let arg1 = lf_checker_rt::global::<u32>(PARAM).read();        let marker = MARKER;
        let marker_ptr = &marker as *const u32 as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32,
            ctx.wrapping_add(CTX_BIAS),
            0, arg1, 0, 0, marker_ptr, ONE, ONE, 1, 0);
        0
    }
});
