// original: 0x009B9FE0 CCamScriptInstruction_SetCinematicCam_Q::vf2

/// Execute the SetCinematicCam_Q script instruction: fetch the active cam
/// (callee 1); if one is active, reset the manager pair (callees 2 and 3
/// with flags (1,1)), select cinematic slot 0x16 on it (callee 4 with
/// (0x16, 0)), apply the operand at `this+0x08` to the selected cam
/// (callee 5), and mark it flags 0x0c at +0x13c.
///
/// No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B9FE0(this: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x0103_E498;
        const MGR: u32 = 0x0128_E400;
        const FIELD_VALUE: u32 = 0x08;
        const OBJ_FLAGS: u32 = 0x13c;
        const SLOT_CINEMATIC: u32 = 0x16;
        const FLAG_MASK: u8 = 0x0c;
        let cam: u32 = lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(CTX));
        if cam != 0 {
            let m = lf_checker_rt::relocated(MGR);
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, m);
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, m, 1, 1);
            let sel: u32 = lf_checker_rt::callee_thiscall!(4, u32, cam,
                SLOT_CINEMATIC, 0);
            let value = ((this + FIELD_VALUE) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, sel, value);
            let cell = (sel + OBJ_FLAGS) as *mut u8;
            cell.write(cell.read() | FLAG_MASK);
        }
        0
    }
});
