// original: 0x00e6b300 task_pool_init_04 (proposed)

/// Initialise a pool of fixed-size records: call the record
/// initialiser once per slot, passing each slot's address.
///
/// Walks COUNT slots of STRIDE bytes from BASE, invoking the
/// per-record routine (object call, no stack arguments) with the
/// slot address. Takes no arguments (cdecl, empty stack list)
/// and returns nothing meaningful; the callee is intercepted by
/// the checker on both sides.
lf_checker_rt::export!(cdecl, rw_00e6b300() -> u32 {
    const BASE: u32 = 0x016ebcb0;
    const COUNT: u32 = 86;
    const STRIDE: u32 = 0x00000020;
    unsafe {
        let mut obj = lf_checker_rt::relocated(BASE);
        let mut left = COUNT;
        while left > 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, obj);
            obj = obj.wrapping_add(STRIDE);
            left -= 1;
        }
    }
    0
});
