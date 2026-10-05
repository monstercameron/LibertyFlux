// original: 0x009B9D80 CCamScriptInstruction_SetCamLookAtPos_Q::vf2

/// Execute the SetCamLookAtPos_Q script instruction: fetch the active cam (callee 1);
/// if one is active, reset the manager pair (callees 2 and 3 with flags
/// (1,1)) and select slot 0x0e on it (callee 4 with (0x0e, 0), marked
/// flags 0x0c at +0x13c). When the operand at `this+0x20` is zero, bind
/// the cam through the binder at its +0x114 word (callee 5 with
/// (0x0e, 0, cam)), install the look-at point (callee 6), mark the bound handle 0x0c and register the
/// pair (callee 7 with (selector, bound)); otherwise the leaf installs the look-at point directly (callee 6).
///
/// No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B9D80(this: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x0103_E498;
        const MGR: u32 = 0x0128_E400;
        const FIELD_MODE: u32 = 0x20;
        const FIELD_POINT: u32 = 0x10;
        const CAM_BINDER: u32 = 0x114;
        const OBJ_FLAGS: u32 = 0x13c;
        const SLOT: u32 = 0x0e;
        let cam: u32 = lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(CTX));
        if cam != 0 {
            let m = lf_checker_rt::relocated(MGR);
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, m);
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, m, 1, 1);
            let sel: u32 = lf_checker_rt::callee_thiscall!(4, u32, cam,
                SLOT, 0);
            let cell = (sel + OBJ_FLAGS) as *mut u8;
            cell.write(cell.read() | 0x0c);
            let mode = ((this + FIELD_MODE) as *const u32).read_unaligned();
            if mode == 0 {
                let binder = ((cam + CAM_BINDER) as *const u32)
                    .read_unaligned();
                let bound: u32 = lf_checker_rt::callee_thiscall!(5, u32,
                    binder, SLOT, 0, cam);
                let _: u32 = lf_checker_rt::callee_thiscall!(6, u32,
                    bound, this.wrapping_add(FIELD_POINT));
                let bcell = (bound + OBJ_FLAGS) as *mut u8;
                bcell.write(bcell.read() | 0x0c);
                let _: u32 = lf_checker_rt::callee_thiscall!(7, u32, m,
                    sel, bound);
            } else {
                let _: u32 = lf_checker_rt::callee_thiscall!(6, u32,
                    sel, this.wrapping_add(FIELD_POINT));
            }
        }
        0
    }
});
