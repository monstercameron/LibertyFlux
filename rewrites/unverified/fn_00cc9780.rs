// original: 0x00CC9780 fragInstGta::vf44

/// Returns true when the direct readiness helper reports success or when the
/// receiver's virtual query returns either accepted status, `14` or `15`.
/// The virtual function is loaded from vtable slot 1 at `+0x04` and may be
/// called twice: the first result is retried unless it equals one of those
/// statuses. The method has no stack arguments and returns its boolean in AL.
lf_checker_rt::export!(thiscall, rw_00cc9780(this: u32) -> u8 {
    const READY_HELPER: u32 = 1;
    const VTABLE_SLOT_1: u32 = 4;
    const READY_STATUS: u32 = 14;
    const ACTIVE_STATUS: u32 = 15;

    if lf_checker_rt::callee_thiscall!(READY_HELPER, u8, this) != 0 {
        return 1;
    }

    let vtable = unsafe { (this as *const u32).read_unaligned() };
    let method_address = unsafe {
        (vtable.wrapping_add(VTABLE_SLOT_1) as *const u32).read_unaligned()
    };
    let virtual_query: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(method_address as usize) };

    let first_status = virtual_query(this);
    if first_status == READY_STATUS || first_status == ACTIVE_STATUS {
        1
    } else {
        let second_status = virtual_query(this);
        u8::from(second_status == READY_STATUS || second_status == ACTIVE_STATUS)
    }
});
