// original: 0x00E72370 cleanup_global_stream_resource
/// If the global stream object exists, call its vtable method at byte slot eight; then send the optional gated cleanup notification.
lf_checker_rt::export!(cdecl, rw_fn_00E72370() -> () {
    unsafe {
        const OBJECT_GLOBAL: u32 = 0x0139C254;
        const NOTIFY_POINTER: u32 = 0x01396978;
        const NOTIFY_ENABLED: u32 = 0x0139697E;
        const VTABLE_METHOD_SLOT: u32 = 8;

        let object = lf_checker_rt::global::<u32>(OBJECT_GLOBAL).read_unaligned();
        if object != 0 {
            let vtable = (object as *const u32).read_unaligned();
            let method_address = ((vtable.wrapping_add(VTABLE_METHOD_SLOT)) as *const u32).read_unaligned();
            let method: unsafe extern "thiscall" fn(u32) -> u32 = core::mem::transmute(method_address as usize);
            let _ = method(object);
        }
        if lf_checker_rt::global::<u16>(NOTIFY_ENABLED).read_unaligned() != 0 {
            let message = lf_checker_rt::global::<u32>(NOTIFY_POINTER).read_unaligned();
            let _ = lf_checker_rt::callee_cdecl!(2, u32, message);
        }
    }
});
