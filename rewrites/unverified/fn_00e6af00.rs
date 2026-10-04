// original: 0xe6af00 task_pool_init_x (proposed)

/// Initialise a pool of fixed-size records, then notify done.
///
/// First walks COUNT slots of STRIDE bytes from BASE, calling
/// the per-record routine with each slot address (object call,
/// no stack arguments); then calls the shared notifier once
/// with this unit's fixed address (cdecl, one stack word).
/// No arguments, no meaningful return value.
lf_checker_rt::export!(cdecl, rw_00e6af00() -> u32 {
    const BASE: u32 = 0x016d3a70;
    const COUNT: u32 = 40;
    const STRIDE: u32 = 0x00000200;
    const ARG_ADDR: u32 = 0x00e72aa0;
    unsafe {
        let mut obj = lf_checker_rt::relocated(BASE);
        let mut left = COUNT;
        while left > 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, obj);
            obj = obj.wrapping_add(STRIDE);
            left -= 1;
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(ARG_ADDR));
    }
    0
});
