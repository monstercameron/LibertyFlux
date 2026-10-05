// original: 0x00e6cda0 veh_array_init_a0 (proposed)

/// Run a shared per-slot initializer over 3640 fixed object slots: `for i in 0..3640: CALLEE(this=BASE+i*STRIDE)`.
///
/// The original walks a counter down from 3639, calling the shared routine with each
/// slot address in turn, and returns the last call's answer. Takes no arguments.
///
/// Original: 0x00e6cda0 (cdecl, no arguments, returns eax).
lf_checker_rt::export!(cdecl, rw_00e6cda0() -> u32 {
    unsafe {
        const BASE: u32 = 0x017475f0;
        const COUNT: u32 = 3640;
        const STRIDE: u32 = 0x00000050;
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
