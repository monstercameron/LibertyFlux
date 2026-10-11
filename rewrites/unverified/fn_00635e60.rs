// original: 0x00635e60 rage::pgDictionary<rage::ptxRule>::vf0
/// Runs the specialization's base cleanup, then conditionally calls the
/// cleanup method stored in the worker TLS context's third virtual slot.
/// The second argument is a bit field: bit zero requests that virtual
/// cleanup when `this` is non-null. The function returns the original
/// object pointer. The callback receives the context object in ECX and
/// the original object as its sole stack argument.
lf_checker_rt::export!(thiscall, rw_00635e60(this: u32, flags: u32) -> u32 {
    const DESTROY_FLAG: u32 = 1;
    const TLS_CONTEXT_SLOT: usize = 0;
    const CONTEXT_OBJECT_OFFSET: u32 = 8;
    const VTABLE_CLEANUP_SLOT_OFFSET: u32 = 12;

    let _base_cleanup = lf_checker_rt::callee_thiscall!(1, u32, this);
    if flags & DESTROY_FLAG != 0 && this != 0 {
        unsafe {
            let tls_context = lf_checker_rt::tls_slot(TLS_CONTEXT_SLOT);
            let context_object = (tls_context.wrapping_add(CONTEXT_OBJECT_OFFSET) as *const u32)
                .read_unaligned();
            let vtable = (context_object as *const u32).read_unaligned();
            let callback_address = vtable.wrapping_add(VTABLE_CLEANUP_SLOT_OFFSET) as *const u32;
            let callback_address = callback_address.read_unaligned();
            let callback: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(callback_address as usize);
            let _callback_result = callback(context_object, this);
        }
    }
    this
});
