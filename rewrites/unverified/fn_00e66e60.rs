// original: 0x00e66e60 init_pair_call (proposed)
/// Call a thiscall callee with a fixed global address as `this`, then call a
/// cdecl callee with one fixed image address, returning the second result.
///
/// Arguments: none (cdecl/0). Reads no globals; the two callees are
/// intercepted by the checker. The first call's return value is discarded;
/// the second call's return value is left in eax as the result.
/// Calling convention: cdecl, callee 1 is thiscall/0, callee 2 is cdecl/1.
lf_checker_rt::export!(cdecl, rw_00e66e60() -> u32 {
    unsafe {
        const THIS_ADDR: u32 = 0x012B9170;
        const PUSHED_ADDR: u32 = 0x00E72150;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(THIS_ADDR));
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(PUSHED_ADDR))
    }
});
