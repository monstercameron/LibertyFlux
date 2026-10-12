// original: 0x00E62CF0 register_network_global_pointer_initializer_d
/// Registers the relocated callback with the C runtime and returns the
/// registration result. The function has no parameters and leaves argument
/// cleanup to its caller.
const CALLBACK_VA: u32 = 0x00E71250;

lf_checker_rt::export!(cdecl, rw_00E62CF0() -> u32 {
    let callback = lf_checker_rt::relocated(CALLBACK_VA);
    lf_checker_rt::callee_cdecl!(1u32, u32, callback)
});
