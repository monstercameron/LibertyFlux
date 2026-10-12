// original: 0x00E72D80 cleanup_flagged_pointer_b
/// Gate a global resource notification with a 16-bit flag and pass its paired 32-bit pointer when enabled.
lf_checker_rt::export!(cdecl, rw_fn_00E72D80() -> () {
    unsafe {
        const MESSAGE_POINTER: u32 = 0x01720930;
        const MESSAGE_ENABLED: u32 = 0x01720936;
        const MESSAGE_CALLEE: u32 = 1;

        let enabled = lf_checker_rt::global::<u16>(MESSAGE_ENABLED).read_unaligned();
        if enabled != 0 {
            let message = lf_checker_rt::global::<u32>(MESSAGE_POINTER).read_unaligned();
            let _ = lf_checker_rt::callee_cdecl!(MESSAGE_CALLEE, u32, message);
        }
    }
});
