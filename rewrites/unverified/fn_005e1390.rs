// original: 0x005E1390 CRenderPhasePhoneModel::vf0
/// Run the phase setup helper, then conditionally notify a thread-local
/// render object. The stack flag's low bit gates the notification; the object
/// must also be non-null. The TLS record's pointer at offset `0x08` supplies
/// an object whose vtable callback is at offset `0x0c`. The method returns
/// the original phase pointer and consumes one stack word.
lf_checker_rt::export!(thiscall, rw_005e1390(this: u32, initialize_flag: u32) -> u32 {
    const SETUP_ID: u32 = 1;
    const TLS_SLOT: usize = 0;
    const TLS_RENDER_OBJECT_OFFSET: u32 = 0x08;
    const VTABLE_CALLBACK_OFFSET: u32 = 0x0c;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    let _ = lf_checker_rt::callee_thiscall!(SETUP_ID, u32, this);
    if initialize_flag & 1 != 0 && this != 0 {
        let tls_record = lf_checker_rt::tls_slot(TLS_SLOT);
        let render_object = unsafe { read_u32(tls_record.wrapping_add(TLS_RENDER_OBJECT_OFFSET)) };
        let vtable = unsafe { read_u32(render_object) };
        let callback = unsafe { read_u32(vtable.wrapping_add(VTABLE_CALLBACK_OFFSET)) };
        let notify: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(callback as usize) };
        let _ = notify(render_object);
    }
    this
});
