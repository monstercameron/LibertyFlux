// original: 0x0086CE30 Script_ProgramTableAlloc

/// Allocate and clear the key words for the shared script program table.
///
/// The stdcall argument is a 32-bit entry count. The allocator object is
/// reached through TLS slot zero, and its method is selected from vtable
/// slot two. The requested byte size is `count * 8`, saturated to all ones
/// when the original unsigned multiply overflows. The returned table pointer
/// is stored globally; positive signed counts clear each entry's key word,
/// while zero or negative counts skip the loop. The used count is reset to
/// zero and the allocation pointer remains in EAX.
lf_checker_rt::export!(stdcall, rw_0086CE30(entry_count: u32) -> u32 {
    const TABLE_BASE_VA: u32 = 0x0111_1030;
    const TABLE_COUNT_VA: u32 = 0x0111_1034;
    const USED_COUNT_VA: u32 = 0x0111_103C;
    const TLS_HEAP_OBJECT_OFFSET_WORDS: usize = 2;
    const VTABLE_ALLOCATOR_SLOT_WORDS: usize = 2;
    const ENTRY_BYTES: u32 = 8;
    const ALLOCATOR_FLAGS: u32 = 0x10;

    unsafe {
        let widened_size = (entry_count as u64) * (ENTRY_BYTES as u64);
        let allocation_bytes = if widened_size > u32::MAX as u64 {
            u32::MAX
        } else {
            widened_size as u32
        };

        lf_checker_rt::global::<u32>(TABLE_COUNT_VA).write_unaligned(entry_count);

        let tls_block = lf_checker_rt::tls_slot(0);
        let engine_object = (tls_block as *const u32)
            .add(TLS_HEAP_OBJECT_OFFSET_WORDS)
            .read_unaligned();
        let vtable = (engine_object as *const u32).read_unaligned();
        let allocator_address = (vtable as *const u32)
            .add(VTABLE_ALLOCATOR_SLOT_WORDS)
            .read_unaligned();
        let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocator_address as usize);
        let table_base = allocate(engine_object, allocation_bytes, ALLOCATOR_FLAGS, 0);

        lf_checker_rt::global::<u32>(TABLE_BASE_VA).write_unaligned(table_base);
        if (entry_count as i32) > 0 {
            let table = table_base as *mut u32;
            for index in 0..(entry_count as usize) {
                table.add(index.wrapping_mul(2)).write_unaligned(0);
            }
        }
        lf_checker_rt::global::<u32>(USED_COUNT_VA).write_unaligned(0);
        table_base
    }
});
