// original: 0x00a10450 fixed_first_forward (proposed)
/// Forward three arguments to the worker with a fixed zero first argument.
///
/// Calls the callee as `callee(0, a, b, c)` and returns its result.
/// Cdecl, three stack arguments.
export!(cdecl, rw_00a10450(a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        const FWD: u32 = 1;
        callee_cdecl!(FWD, u32, 0, a, b, c)
    }
});
