// original: 0x006A4B60 forward_args_if_first_nonnull

/// If the first cdecl argument is non-null, forwards it in ECX and the
/// second stack argument to the helper. A null first argument returns without a
/// call. The helper's return register is not a declared result of this wrapper.
lf_checker_rt::export!(cdecl, rw_006a4b60(
    first_argument: u32,
    second_argument: u32,
) -> () {
    const HELPER_ID: u32 = 1;
    if first_argument != 0 {
        let _ = lf_checker_rt::callee_thiscall!(HELPER_ID, u32, first_argument, second_argument);
    }
});
