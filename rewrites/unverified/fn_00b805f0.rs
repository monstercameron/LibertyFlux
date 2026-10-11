// original: 0x00B805F0 CTaskComplexCarDriveWander::vf21

/// Run the inherited driving-task method, conditionally notify the embedded
/// vehicle when both global suppression bytes and the local `+0x49` byte are
/// zero, and call the engine-state helper with false. Finally copy the global
/// timer word to the embedded vehicle's `+0x0e38` field and return that word.
lf_checker_rt::export!(thiscall, rw_b805f0(this: u32) -> u32 {
    unsafe {
        const EMBEDDED_VEHICLE: u32 = 0x40;
        const LOCAL_SUPPRESSION: u32 = 0x49;
        const TIMER_FIELD: u32 = 0x0e38;
        const TIMER_GLOBAL: u32 = 0x011735b4;
        const SUPPRESSION_GLOBALS: u32 = 0x01600148;
        const BASE_DRIVE_METHOD: u32 = 1;
        const NOTIFY_VEHICLE: u32 = 2;
        const SET_ENGINE_STATE: u32 = 3;

        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_DRIVE_METHOD, u32, this);
        let vehicle = ((this + EMBEDDED_VEHICLE) as *const u32).read_unaligned();
        let suppress = (lf_checker_rt::relocated(SUPPRESSION_GLOBALS + 1) as *const u8).read()
            | (lf_checker_rt::relocated(SUPPRESSION_GLOBALS + 2) as *const u8).read()
            | ((this + LOCAL_SUPPRESSION) as *const u8).read();
        if suppress == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(NOTIFY_VEHICLE, u32, vehicle);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(SET_ENGINE_STATE, u32, vehicle, 0u32);
        let timer = (lf_checker_rt::relocated(TIMER_GLOBAL) as *const u32).read_unaligned();
        ((vehicle + TIMER_FIELD) as *mut u32).write_unaligned(timer);
        timer
    }
});
