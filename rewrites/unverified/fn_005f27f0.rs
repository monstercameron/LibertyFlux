// original: 0x005f27f0 CRenderPhaseHtml::vf0

/// Calls the base phase handler, then conditionally dispatches a TLS object's
/// virtual method when flag bit zero is set and this phase pointer is non-null.
/// TLS slot zero contains a block pointer; the object is at block `+0x08`, and
/// its function table entry is at slot `+0x0c`. The function returns `this`.
lf_checker_rt::export!(thiscall, rw_005f27f0(this: u32, flags: u32) -> u32 {
    const TLS_SLOT: usize = 0;
    const TLS_OBJECT: u32 = 0x08;
    const VTABLE_METHOD: u32 = 0x0c;
    const ACTIVE_FLAG: u32 = 1;

    let _ = lf_checker_rt::callee_thiscall!(2, u32, this);
    if flags & ACTIVE_FLAG != 0 && this != 0 {
        let tls_block = lf_checker_rt::tls_slot(TLS_SLOT);
        let callback_object = unsafe { (tls_block.wrapping_add(TLS_OBJECT) as *const u32).read_unaligned() };
        let vtable = unsafe { (callback_object as *const u32).read_unaligned() };
        let target = unsafe { (vtable.wrapping_add(VTABLE_METHOD) as *const u32).read_unaligned() };
        // SAFETY: the contract plants an intercepted thiscall stub at this table slot.
        let callback: extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(target as usize) };
        let _ = callback(callback_object, this);
    }
    this
});
