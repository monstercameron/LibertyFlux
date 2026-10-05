// original: 0x009B97B0 CCamScriptInstruction_EnableDebugCam::vf2

/// Execute the EnableDebugCam script instruction: open debug slots 0x27
/// then 0x26 on the global camera context (callees 1 and 2, chained
/// through their answers), activate the debug cam (callee 3), copy the
/// context's +0x10 block into the cam's +0x10 (callee 4) and re-derive
/// its dependent vectors (callee 5, cdecl, with the cam's
/// +0x20/+0x150/+0x154).
///
/// Reads no instruction fields; the context root comes from its engine
/// global. The copy arguments are pointers into the heap objects on both
/// sides, so they compare by normalised address. No return value
/// (thiscall).
lf_checker_rt::export!(thiscall, rw_009B97B0(this: u32) -> u32 {
    unsafe {
        const CTX_PTR: u32 = 0x0103_E49C;
        let _ = this;
        let ctx = lf_checker_rt::global::<u32>(CTX_PTR).read();
        let mid: u32 = lf_checker_rt::callee_thiscall!(1, u32, ctx, 0x27, 0);
        let cam: u32 = lf_checker_rt::callee_thiscall!(2, u32, mid, 0x26, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, cam);
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32,
            cam.wrapping_add(0x10), ctx.wrapping_add(0x10));
        let _: u32 = lf_checker_rt::callee_cdecl!(5, u32,
            cam.wrapping_add(0x20), cam.wrapping_add(0x150),
            cam.wrapping_add(0x154));
        0
    }
});
