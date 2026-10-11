// original: 0x00AA0B00 register_loader_entry
/// If entry flag bit 5 is clear and the signed queue count is below
/// 32, calls the entry flag helper, stores the entry pointer in the
/// count-selected queue word, and increments the count. The second
/// variant also notifies its manager whenever bit 5 is clear, even
/// when the queue is full. The routine has no defined return value.
lf_checker_rt::export!(cdecl, rw_00AA0B00(entry: u32) -> () {
    unsafe {
        const ENTRY_FLAGS_OFFSET: u32 = 0x254;
        const QUEUE_COUNT_VA: u32 = 0x01396A00;
        const QUEUE_TABLE_VA: u32 = 0x01396980;
        let entry_flags = (entry.wrapping_add(ENTRY_FLAGS_OFFSET) as *const u32)
            .read_unaligned();
        if entry_flags & 0x20 == 0 {
            let count = lf_checker_rt::global::<u32>(QUEUE_COUNT_VA).read_unaligned();
            if (count as i32) < 32 {
                let _ = lf_checker_rt::callee_stdcall!(1, u32, entry);
                let slot = QUEUE_TABLE_VA.wrapping_add(count.wrapping_mul(4));
                lf_checker_rt::global::<u32>(slot).write_unaligned(entry);
                lf_checker_rt::global::<u32>(QUEUE_COUNT_VA)
                    .write_unaligned(count.wrapping_add(1));
            }
        }
    }
});
