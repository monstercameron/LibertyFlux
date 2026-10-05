// original: 0x009B9860 CCamScriptInstruction_InsertSplineNode::vf2

/// Execute the InsertSplineNode script instruction: look two cams up
/// by the indexes at `this+0x08` (callee 1) and `this+0x0c` (callee 2);
/// if the first exists, insert the second into it (callee 3).
///
/// The two lookups are separate call sites of one engine routine, scripted
/// as two callee ids so each answers its own heap object per trial.
/// No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B9860(this: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0128_E400;
        const FIELD_FIRST: u32 = 0x08;
        const FIELD_SECOND: u32 = 0x0c;
        let first: u32 = lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(MGR),
            ((this + FIELD_FIRST) as *const u32).read_unaligned());
        if first != 0 {
            let second: u32 = lf_checker_rt::callee_thiscall!(2, u32,
                lf_checker_rt::relocated(MGR),
                ((this + FIELD_SECOND) as *const u32).read_unaligned());
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, first, second);
        }
        0
    }
});
