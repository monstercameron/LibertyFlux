// original: 0x009B9E10 CCamScriptInstruction_SetCamPos_Q::vf2

/// Execute the SetCamPos_Q script instruction: fetch the active cam
/// (callee 1); if one is active, reset the manager pair (callees 2 and 3
/// with flags (1,1)), select position slot 0x0e on it (callee 4 with
/// (0x0e, 0)), copy the four position words at `this+0x10`..`this+0x1c`
/// into the selected cam at +0x40..+0x4c, and mark it flags 0x0c at
/// +0x13c.
///
/// The copies are bit-copies through SSE registers; no arithmetic is
/// performed. No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B9E10(this: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x0103_E498;
        const MGR: u32 = 0x0128_E400;
        const FIELD_POS: u32 = 0x10;
        const OBJ_POS: u32 = 0x40;
        const OBJ_FLAGS: u32 = 0x13c;
        const SLOT_POS: u32 = 0x0e;
        let cam: u32 = lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(CTX));
        if cam != 0 {
            let m = lf_checker_rt::relocated(MGR);
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, m);
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, m, 1, 1);
            let sel: u32 = lf_checker_rt::callee_thiscall!(4, u32, cam,
                SLOT_POS, 0);
            for i in 0..4u32 {
                let bits = ((this + FIELD_POS + i * 4) as *const u32)
                    .read_unaligned();
                ((sel + OBJ_POS + i * 4) as *mut u32).write_unaligned(bits);
            }
            let cell = (sel + OBJ_FLAGS) as *mut u8;
            cell.write(cell.read() | 0x0c);
        }
        0
    }
});
