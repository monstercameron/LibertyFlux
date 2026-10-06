// original: 0x005B2EE0 teardown_control_state (proposed)

/// Shut down the control subsystem and release its shared object.
///
/// Raises each stage callee in order with fixed arguments (seven cdecl
/// calls), parks the stage globals (flag byte cleared, index zeroed, handle
/// set to -1, mode word set to 4 with its flag byte cleared, status word
/// cleared), then, when the shared-object global is non-null, runs its
/// shutdown method and releases it through the thread heap manager reached
/// via TLS slot 0 (slot -> +8 -> vtable -> slot +0xc, the free entry, which
/// takes the object pointer and pops it), clearing the global afterwards.
/// A null shared object skips both the method and the release. Returns the
/// release answer, or the last stage answer when there was nothing to free.
lf_checker_rt::export!(cdecl, rw_005B2EE0() -> u32 {
    unsafe {
        const FLAG: u32 = 0x01160_C3A;
        const INDEX: u32 = 0x01160_C40;
        const HANDLE: u32 = 0x01030_BA8;
        const MODE: u32 = 0x018E5_1E4;
        const MODE_FLAG: u32 = 0x018E5_1CD;
        const STATUS: u32 = 0x018E5_1E1;
        const SHARED: u32 = 0x018B6_E84;
        const STAGE_A: u32 = 1;
        const STAGE_B: u32 = 2;
        const STAGE_C: u32 = 3;
        const STAGE_D: u32 = 4;
        const STAGE_E: u32 = 5;
        const STAGE_F: u32 = 6;
        const STAGE_G: u32 = 7;
        const SHUTDOWN: u32 = 8;

        let _ = lf_checker_rt::callee_cdecl!(STAGE_A, u32, 5u32);
        (lf_checker_rt::relocated(FLAG) as *mut u8).write(0);
        let _ = lf_checker_rt::callee_cdecl!(STAGE_B, u32, 0u32);
        lf_checker_rt::global::<u32>(INDEX).write(0);
        lf_checker_rt::global::<u32>(HANDLE).write(0xFFFF_FFFF);
        lf_checker_rt::global::<u32>(MODE).write(4);
        (lf_checker_rt::relocated(MODE_FLAG) as *mut u8).write(0);
        let _ = lf_checker_rt::callee_cdecl!(STAGE_C, u32, 0u32, 4u32, 8u32);
        let _ = lf_checker_rt::callee_cdecl!(STAGE_D, u32, 0u32, 8u32);
        let _ = lf_checker_rt::callee_cdecl!(STAGE_E, u32, 0u32, 1u32);
        let _ = lf_checker_rt::callee_cdecl!(STAGE_F, u32, 1u32);
        let mut last = lf_checker_rt::callee_cdecl!(STAGE_G, u32, 1u32);
        let shared = lf_checker_rt::global::<u32>(SHARED).read();
        (lf_checker_rt::relocated(STATUS) as *mut u16).write(0);
        if shared != 0 {
            lf_checker_rt::callee_thiscall!(SHUTDOWN, u32, shared);
            let slot = lf_checker_rt::tls_slot(0);
            let mgr = (slot.wrapping_add(8) as *const u32).read();
            let vtable = (mgr as *const u32).read();
            let entry = (vtable.wrapping_add(0xC) as *const u32).read();
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(entry as usize);
            last = free(mgr, shared);
            lf_checker_rt::global::<u32>(SHARED).write(0);
        }
        last
    }
});
