// original: 0x009B9620 CCamScriptInstruction_DestroyCam::vf2

/// Execute the DestroyCam script instruction: look the cam up by the
/// index at `this+0x08` through the camera manager singleton (callee 1);
/// if found, run its teardown step (callee 2, no arguments) and then its
/// release step (callee 3 with a zero flag).
///
/// No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B9620(this: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0128_E400;
        const FIELD_INDEX: u32 = 0x08;
        let index = ((this + FIELD_INDEX) as *const u32).read_unaligned();
        let cam: u32 = lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(MGR), index);
        if cam != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, cam);
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, cam, 0);
        }
        0
    }
});
