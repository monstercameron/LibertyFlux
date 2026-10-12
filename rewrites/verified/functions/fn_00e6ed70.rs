// original: 0x00E6ED70 network_cleanup_thread_local_objects

/// Visits four global object records in descending order. A record is eligible
/// only when its 16-bit state at `+6` and its object pointer at `+0` are both
/// nonzero. Each eligible object is passed to slot 3 of the manager vtable,
/// where the manager comes from TLS slot 0 at offset `+8`. The indirect target
/// is thiscall with one stack argument. EAX is not a
/// semantic result: the original may leave either its incoming EAX or the
/// last manager result, and this rewrite cannot read incoming EAX.
lf_checker_rt::export!(cdecl, rw_00e6ed70() -> u32 {
    const RECORDS_BASE_VA: u32 = 0x019D_2FF8;
    const RECORD_COUNT: u32 = 4;
    const RECORD_STRIDE: u32 = 8;
    const RECORD_OBJECT_OFFSET: u32 = 0;
    const RECORD_STATE_OFFSET: u32 = 6;
    const MANAGER_TLS_SLOT: usize = 0;
    const MANAGER_INTERFACE_OFFSET: u32 = 8;
    const MANAGER_VTABLE_OFFSET: u32 = 0;
    const CLEANUP_SLOT_OFFSET: u32 = 12;

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    // SAFETY: record and manager memory are seeded by the contract; the
    // target address is the recorder stub planted in the fabricated vtable.
    unsafe {
        for index in (0..RECORD_COUNT).rev() {
            let record_va = RECORDS_BASE_VA + index * RECORD_STRIDE;
            let state = read_u16(lf_checker_rt::relocated(record_va + RECORD_STATE_OFFSET));
            if state != 0 {
                let object = read_u32(lf_checker_rt::relocated(record_va + RECORD_OBJECT_OFFSET));
                if object != 0 {
                    let tls_array = lf_checker_rt::tls_slot(MANAGER_TLS_SLOT);
                    let manager = read_u32(tls_array.wrapping_add(MANAGER_INTERFACE_OFFSET));
                    let vtable = read_u32(manager.wrapping_add(MANAGER_VTABLE_OFFSET));
                    let cleanup = read_u32(vtable.wrapping_add(CLEANUP_SLOT_OFFSET));
                    let call: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(cleanup as usize);
                    let _ = call(manager, object);
                }
            }
        }
        0
    }
});
