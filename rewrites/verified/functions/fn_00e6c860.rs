// original: 0x00e6c860 veh_multi_init_60 (proposed)

/// Initialize 12 fixed slots, then look up one fixed key and return its answer.
///
/// First `for i in 0..12: CALLEE1(this=BASE+i*STRIDE)` (counter down from 11),
/// then `return CALLEE2(ARG2)` (cdecl/1). Takes no arguments.
///
/// Original: 0x00e6c860 (cdecl, no arguments, returns eax).
lf_checker_rt::export!(cdecl, rw_00e6c860() -> u32 {
    unsafe {
        const BASE: u32 = 0x0171f9d0;
        const COUNT: u32 = 12;
        const STRIDE: u32 = 0x00000014;
        const ARG2: u32 = 0x00e72d10;
        let mut slot = lf_checker_rt::relocated(BASE);
        let mut i = COUNT;
        loop {
            lf_checker_rt::callee_thiscall!(1, u32, slot);
            slot = slot.wrapping_add(STRIDE);
            i -= 1;
            if i == 0 { break; }
        }
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(ARG2))
    }
});
