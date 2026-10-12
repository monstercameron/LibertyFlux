// original: 0x00E71370 clear_viewport_children_and_notify
/// Release the viewport and its child list in order, then emit each of three notifications whose 16-bit gate is nonzero.
lf_checker_rt::export!(cdecl, rw_fn_00E71370() -> () {
    unsafe {
        const VIEWPORT_ROOT: u32 = 0x0118D850;
        const CHILD_LIST: u32 = 0x0118D828;
        let _ = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(VIEWPORT_ROOT));
        let _ = lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(CHILD_LIST));
        const MESSAGE_POINTER_0: u32 = 0x0118D828;
        const MESSAGE_ENABLED_0: u32 = 0x0118D82E;
        if lf_checker_rt::global::<u16>(MESSAGE_ENABLED_0).read_unaligned() != 0 {
            let message = lf_checker_rt::global::<u32>(MESSAGE_POINTER_0).read_unaligned();
            let _ = lf_checker_rt::callee_cdecl!(3, u32, message);
        }
        const MESSAGE_POINTER_1: u32 = 0x0118D7F8;
        const MESSAGE_ENABLED_1: u32 = 0x0118D7FE;
        if lf_checker_rt::global::<u16>(MESSAGE_ENABLED_1).read_unaligned() != 0 {
            let message = lf_checker_rt::global::<u32>(MESSAGE_POINTER_1).read_unaligned();
            let _ = lf_checker_rt::callee_cdecl!(3, u32, message);
        }
        const MESSAGE_POINTER_2: u32 = 0x0118D7F0;
        const MESSAGE_ENABLED_2: u32 = 0x0118D7F6;
        if lf_checker_rt::global::<u16>(MESSAGE_ENABLED_2).read_unaligned() != 0 {
            let message = lf_checker_rt::global::<u32>(MESSAGE_POINTER_2).read_unaligned();
            let _ = lf_checker_rt::callee_cdecl!(3, u32, message);
        }
    }
});
