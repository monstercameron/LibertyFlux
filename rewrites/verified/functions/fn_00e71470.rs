// original: 0x00E71470 notify_code_pointer_3
/// Gate a code-pointer notification with a 16-bit flag and pass the paired 32-bit message pointer when enabled.
lf_checker_rt::export!(cdecl, rw_fn_00E71470() -> () {
    unsafe {
        const MESSAGE_POINTER: u32 = 0x011A9D18;
        const MESSAGE_ENABLED: u32 = 0x011A9D1E;
        const MESSAGE_CALLEE: u32 = 1;

        let enabled = lf_checker_rt::global::<u16>(MESSAGE_ENABLED).read_unaligned();
        if enabled != 0 {
            let message = lf_checker_rt::global::<u32>(MESSAGE_POINTER).read_unaligned();
            let _ = lf_checker_rt::callee_cdecl!(MESSAGE_CALLEE, u32, message);
        }
    }
});
