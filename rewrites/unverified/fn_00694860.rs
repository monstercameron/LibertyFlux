// original: 0x00694860 allocate_zeroed_track_records_00694860 (proposed)

/// Allocate a signed count of zero-initialized track records.
///
/// The signed record count is the sole stdcall argument. TLS slot zero leads
/// to an allocator whose vtable slot `+8` receives the wrapping 32-bit byte
/// size `count * 20`, tag `0x10`, and a zero option. The allocator result is
/// returned unchanged when the count is nonpositive or the result is null.
/// For a positive count and nonnull result, each 20-byte record has its first
/// 16-bit word, the word at `+4`, the two words at `+8` and `+0x0c`, and the
/// word at `+0x10` cleared; bytes `+2` and `+3` are left untouched.
lf_checker_rt::export!(stdcall, rw_00694860(count: i32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const OWNER_ALLOCATOR_OFFSET: u32 = 8;
        const ALLOCATOR_VTABLE_SLOT: u32 = 8;
        const RECORD_SIZE: u32 = 20;
        const ALLOCATION_TAG: u32 = 0x10;

        let owner = lf_checker_rt::tls_slot(TLS_SLOT);
        let allocator = *((owner.wrapping_add(OWNER_ALLOCATOR_OFFSET)) as *const u32);
        let vtable = *((allocator) as *const u32);
        let allocate_address = *((vtable.wrapping_add(ALLOCATOR_VTABLE_SLOT)) as *const u32);
        let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocate_address as usize);
        let byte_size = (count as u32).wrapping_mul(RECORD_SIZE);
        let records = allocate(allocator, byte_size, ALLOCATION_TAG, 0);

        if count <= 0 || records == 0 {
            return records;
        }

        let mut record_index = 0u32;
        while record_index < count as u32 {
            let record = records.wrapping_add(record_index * RECORD_SIZE) as *mut u8;
            (record as *mut u16).write_unaligned(0);
            *((record.add(4)) as *mut u32) = 0;
            *((record.add(8)) as *mut u64) = 0;
            *((record.add(0x10)) as *mut u32) = 0;
            record_index += 1;
        }
        records
    }
});
