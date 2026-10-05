// original: 0x00e6c5d0 veh_array_init_d0 (proposed)

/// Run a shared per-slot initializer over 2 fixed object slots: `for i in 0..2: CALLEE(this=BASE+i*STRIDE)`.
///
/// The original walks a counter down from 1, calling the shared routine with each
/// slot address in turn, and returns the last call's answer. Takes no arguments.
///
/// Original: 0x00e6c5d0 (cdecl, no arguments, returns eax).
lf_checker_rt::export!(cdecl, rw_00e6c5d0() -> u32 {
    unsafe {
        const BASE: u32 = 0x0171ca40;
        const COUNT: u32 = 2;
        const STRIDE: u32 = 0x00000068;
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
