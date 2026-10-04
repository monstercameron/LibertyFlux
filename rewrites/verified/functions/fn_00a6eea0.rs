// original: 0x00a6eea0 CTaskSimpleMoveInAir::vf1

/// Clone hook of the move-in-air task: builds a fresh task object from the
/// manager's allocator and stamps it with this class's vtables.
///
/// Fetches the current manager through the anchor at `MANAGER_ANCHOR`
/// (callee 1); a null manager yields null. Otherwise runs the base
/// initialiser (callee 2) with the fresh object in ecx and a constant `1`
/// argument, writes the primary vtable `VTABLE` at the object's head and
/// the secondary vtable `VTABLE2` at `+ SECONDARY`, and returns the object.
///
/// Original: thiscall, no stack arguments. The incoming object pointer is
/// unused: every field of the clone is a constant.
lf_checker_rt::export!(thiscall, rw_00a6eea0(_this: u32) -> u32 {
    unsafe {
        const MANAGER_ANCHOR: u32 = 0x0167_e2a0;
        const VTABLE: u32 = 0x00e9_ec1c;
        const SECONDARY: u32 = 0x14;
        const VTABLE2: u32 = 0x00e9_ec70;
        const GET_MANAGER: u32 = 1;
        const BASE_INIT: u32 = 2;

        let anchor = (lf_checker_rt::relocated(MANAGER_ANCHOR) as *const u32).read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
        if mgr == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(BASE_INIT, u32, mgr, 1);
        // Both stamps are file VAs the loader relocates.
        (mgr as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((mgr as *mut u32).wrapping_byte_offset(SECONDARY as isize))
            .write_unaligned(lf_checker_rt::relocated(VTABLE2));
        mgr
    }
});
