// original: 0x00e6c930 veh_array_init_30 (proposed)

/// Run a shared per-slot initializer over 8 fixed object slots: `for i in 0..8: CALLEE(this=BASE+i*STRIDE)`.
///
/// The original walks a counter down from 7, calling the shared routine with each
/// slot address in turn, and returns the last call's answer. Takes no arguments.
///
/// Original: 0x00e6c930 (cdecl, no arguments, returns eax).
lf_checker_rt::export!(cdecl, rw_00e6c930() -> u32 {
    unsafe {
        const BASE: u32 = 0x0171fb20;
        const COUNT: u32 = 8;
        const STRIDE: u32 = 0x00000150;
        let mut r = 0u32;
        let mut slot = lf_checker_rt::relocated(BASE);
        let mut i = COUNT;
        loop {
            r = lf_checker_rt::callee_thiscall!(1, u32, slot);
            slot = slot.wrapping_add(STRIDE);
            i -= 1;
            if i == 0 { break; }
        }
        r
    }
});
