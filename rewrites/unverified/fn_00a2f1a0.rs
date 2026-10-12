// original: 0x00A2F1A0 rage::sysEmergencyAllocator::vf9

/// Dispatches the thread allocator's virtual method in byte slot 0x24.
///
/// The no-argument cdecl wrapper reads the allocated TLS slot index, follows
/// the thread record to its allocator object, loads that object's vtable
/// entry, and calls it with the allocator object in ECX. The method's EAX
/// result is returned unchanged. The proof contract plants a scripted stub
/// in this specific table slot, so the rewrite makes the same indirect call.
lf_checker_rt::export!(cdecl, rw_00a2f1a0() -> u32 {
    const TLS_INDEX: u32 = 0x017A_BA14;
    const THREAD_RECORD_ALLOCATOR: usize = 4;
    const METHOD_SLOT_WORD_INDEX: usize = 9;

    unsafe {
        let slot = lf_checker_rt::global::<u32>(TLS_INDEX).read_unaligned() as usize;
        let thread_record = lf_checker_rt::tls_slot(slot) as *const u32;
        let allocator = thread_record.add(THREAD_RECORD_ALLOCATOR).read_unaligned();
        let vtable = (allocator as *const u32).read_unaligned();
        let method_address = (vtable as *const u32)
            .add(METHOD_SLOT_WORD_INDEX)
            .read_unaligned();
        let method: unsafe extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(method_address as usize);
        method(allocator)
    }
});
