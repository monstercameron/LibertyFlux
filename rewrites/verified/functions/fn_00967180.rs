// original: 0x00967180 or_worker_local_flags

/// ORs the stack argument into the current thread's 32-bit worker flags at
/// offset `0x8d0`. The TLS slot number comes from the relocated image global;
/// the checker runtime exposes the same fabricated TLS slot to the rewrite.
lf_checker_rt::export!(cdecl, rw_00967180(flags: u32) -> u32 {
    const TLS_INDEX_GLOBAL: u32 = 0x017aba14;
    const WORKER_FLAGS: u32 = 0x8d0;
    unsafe {
        let slot_index = lf_checker_rt::global::<u32>(TLS_INDEX_GLOBAL).read_unaligned() as usize;
        let thread_object = lf_checker_rt::tls_slot(slot_index);
        let flags_slot = thread_object.wrapping_add(WORKER_FLAGS) as *mut u32;
        let previous = flags_slot.read_unaligned();
        flags_slot.write_unaligned(previous | flags);
    }
    flags
});
