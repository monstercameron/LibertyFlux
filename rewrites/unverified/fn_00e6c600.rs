// original: 0x00e6c600 veh_array_init_00 (proposed)

/// Run a shared per-slot initializer over 10 fixed object slots: `for i in 0..10: CALLEE(this=BASE+i*STRIDE)`.
///
/// The original walks a counter down from 9, calling the shared routine with each
/// slot address in turn, and returns the last call's answer. Takes no arguments.
///
/// Original: 0x00e6c600 (cdecl, no arguments, returns eax).
lf_checker_rt::export!(cdecl, rw_00e6c600() -> u32 {
    unsafe {
        const BASE: u32 = 0x0171d0a8;
        const COUNT: u32 = 10;
        const STRIDE: u32 = 0x0000009c;
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
