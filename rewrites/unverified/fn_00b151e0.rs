// original: 0x00B151E0 hash_table_clear

/// Free each node in every bucket, free the bucket array, and reset the
/// table's pointer and 16-bit bucket count. Bucket heads are read from the
/// array at table offset 0; the unsigned 16-bit count is at offset 4. Each
/// node's next pointer is at offset 0x10. The free calls are external cdecl
/// effects intercepted by the proof contract. The method uses thiscall and
/// returns zero after clearing the table.
lf_checker_rt::export!(thiscall, rw_00B151E0(table: u32) -> u32 {
    unsafe {
        const BUCKETS_POINTER_OFFSET: usize = 0;
        const BUCKET_COUNT_OFFSET: usize = 4;
        const BUCKET_POINTER_SIZE: usize = 4;
        const NODE_NEXT_OFFSET: usize = 0x10;

        let table_bytes = table as *mut u8;
        let buckets = core::ptr::read_unaligned(
            table_bytes.add(BUCKETS_POINTER_OFFSET).cast::<u32>(),
        );
        let bucket_count = core::ptr::read_unaligned(
            table_bytes.add(BUCKET_COUNT_OFFSET).cast::<u16>(),
        );
        for bucket_index in 0..bucket_count as usize {
            let mut node = core::ptr::read_unaligned(
                (buckets as *const u8)
                    .add(bucket_index * BUCKET_POINTER_SIZE)
                    .cast::<u32>(),
            );
            while node != 0 {
                let next = core::ptr::read_unaligned(
                    (node as *const u8).add(NODE_NEXT_OFFSET).cast::<u32>(),
                );
                let _ = lf_checker_rt::callee_cdecl!(1, u32, node);
                node = next;
            }
        }
        let _ = lf_checker_rt::callee_cdecl!(1, u32, buckets);
        core::ptr::write_unaligned(table_bytes.add(BUCKETS_POINTER_OFFSET).cast::<u32>(), 0);
        core::ptr::write_unaligned(table_bytes.add(BUCKET_COUNT_OFFSET).cast::<u16>(), 0);
        0
    }
});
