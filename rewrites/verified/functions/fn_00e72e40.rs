// original: 0x00E72E40 release_global_entity_resource
/// Send a notification when the global entity pointer is nonnull, then tail-call the release helper for the global entity slot.
lf_checker_rt::export!(cdecl, rw_fn_00E72E40() -> u32 {
    unsafe {
        const MESSAGE_POINTER: u32 = 0x017A64F0;
        const GLOBAL_ENTITY: u32 = 0x017A64F8;
        let message = lf_checker_rt::global::<u32>(MESSAGE_POINTER).read_unaligned();
        if message != 0 {
            let _ = lf_checker_rt::callee_cdecl!(1, u32, message);
        }
        lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(GLOBAL_ENTITY))
    }
});
