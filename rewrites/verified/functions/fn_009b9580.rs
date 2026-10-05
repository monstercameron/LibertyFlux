// original: 0x009B9580 CCamScriptInstruction_AttachCamToViewport::vf2

/// Execute the AttachCamToViewport script instruction: look the cam
/// (callee 1, index at `this+0x08`) and the viewport (callee 2, index at
/// `this+0x0c`) up; if the cam exists, resolve the attach target
/// (callee 3), bind it with mode 5 (callee 4, `this` = the target's word
/// at +0x114, args (5, 0, target)), link the cam to the bound handle
/// (`cam+0x128`, flag 0x20 at `cam+0x13c`), flag the handle 0x04 at
/// +0x13c, and register the viewport on it (callee 5).
///
/// No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B9580(this: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0128_E400;
        const CTX: u32 = 0x0103_E498;
        const FIELD_CAM: u32 = 0x08;
        const FIELD_VIEWPORT: u32 = 0x0c;
        const TARGET_BIND_WORD: u32 = 0x114;
        const CAM_LINK: u32 = 0x128;
        const OBJ_FLAGS: u32 = 0x13c;
        const MODE: u32 = 5;
        let cam: u32 = lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(MGR),
            ((this + FIELD_CAM) as *const u32).read_unaligned());
        if cam != 0 {
            let viewport: u32 = lf_checker_rt::callee_thiscall!(2, u32,
                lf_checker_rt::relocated(MGR),
                ((this + FIELD_VIEWPORT) as *const u32).read_unaligned());
            let target: u32 = lf_checker_rt::callee_thiscall!(3, u32,
                lf_checker_rt::relocated(CTX));
            let binder = ((target + TARGET_BIND_WORD) as *const u32)
                .read_unaligned();
            let handle: u32 = lf_checker_rt::callee_thiscall!(4, u32, binder,
                MODE, 0, target);
            let cam_flags = (cam + OBJ_FLAGS) as *mut u8;
            cam_flags.write(cam_flags.read() | 0x20);
            ((cam + CAM_LINK) as *mut u32).write_unaligned(handle);
            let h_flags = (handle + OBJ_FLAGS) as *mut u8;
            h_flags.write(h_flags.read() | 0x04);
            let _: u32 =
                lf_checker_rt::callee_thiscall!(5, u32, handle, viewport);
        }
        0
    }
});
