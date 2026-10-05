// original: 0x009B9780 CCamScriptInstruction_EnableCamCollision::vf2

/// Execute the EnableCamCollision script instruction: look the cam up
/// by the index at `this+0x08` (callee 1); if found, forward the
/// zero-extended operand byte at `this+0x0c` to the collision setter
/// (callee 2).
///
/// No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B9780(this: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0128_E400;
        const FIELD_INDEX: u32 = 0x08;
        const FIELD_FLAG: u32 = 0x0c;
        let index = ((this + FIELD_INDEX) as *const u32).read_unaligned();
        let cam: u32 = lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(MGR), index);
        if cam != 0 {
            let flag = ((this + FIELD_FLAG) as *const u8).read() as u32;
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, cam, flag);
        }
        0
    }
});
