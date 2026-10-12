// original: 0x008C4420 system_cache_avail_phys_mem

/// Refresh one cached memory statistic. It calls the update routine, reads the current
/// statistics context from its global pointer, copies the selected 32-bit field to a separate
/// global, and leaves that value in EAX. The call is intercepted by the contract.
lf_checker_rt::export!(cdecl, rw_008c4420() -> u32 {
    const UPDATE_CALLEE: u32 = 40;
    const CONTEXT_VA: u32 = 0x01168B98;
    const OUTPUT_VA: u32 = 0x01168B9C;
    const FIELD_OFFSET: usize = 0x0C;
    unsafe {
        let _ = lf_checker_rt::callee_cdecl!(UPDATE_CALLEE, u32,);
        let context_pointer = core::ptr::read_volatile(lf_checker_rt::global::<u32>(CONTEXT_VA));
        let value_pointer = (context_pointer as *const u8).wrapping_add(FIELD_OFFSET).cast::<u32>();
        let value = core::ptr::read_volatile(value_pointer);
        core::ptr::write_volatile(lf_checker_rt::global::<u32>(OUTPUT_VA), value);
        value
    }
});
