// original: 0x00AF5DE0 weapon_model_add_reference

/// Add one reference to a model-pool entry. The model id selects a byte in the
/// pool's flag table and an entry using the pool's stored stride. If flag bit
/// 7 is set, the original faults while attempting a reference update through
/// a null address; otherwise the entry's 32-bit reference count wraps upward.
lf_checker_rt::export!(cdecl, rw_00af5de0(model_id: u32) -> u32 {
    const POOL_POINTER: u32 = 0x016EC7B8;
    const POOL_ENTRIES: u32 = 0x00;
    const POOL_FLAGS: u32 = 0x04;
    const POOL_ENTRY_STRIDE: u32 = 0x0C;
    const FLAG_REFERENCE_LOCKED: u8 = 0x80;
    const REFERENCE_COUNT: u32 = 0x04;

    unsafe {
        let pool = lf_checker_rt::global::<u32>(POOL_POINTER).read_volatile();
        let flags = (pool.wrapping_add(POOL_FLAGS) as *const u32).read_unaligned();
        let flag = (flags.wrapping_add(model_id) as *const u8).read();
        if flag & FLAG_REFERENCE_LOCKED != 0 {
            let invalid = 0x04usize as *mut u32;
            let old_count = invalid.read_volatile();
            invalid.write_volatile(old_count.wrapping_add(1));
            return 0;
        }

        let entries = (pool.wrapping_add(POOL_ENTRIES) as *const u32).read_unaligned();
        let stride = (pool.wrapping_add(POOL_ENTRY_STRIDE) as *const u32).read_unaligned();
        let entry = entries.wrapping_add(model_id.wrapping_mul(stride));
        let count = (entry.wrapping_add(REFERENCE_COUNT) as *mut u32);
        count.write_unaligned(count.read_unaligned().wrapping_add(1));
        entry
    }
});
