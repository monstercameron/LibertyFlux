// original: 0x00C4EDC0 task_content_stream_gate

/// Check whether a task record is eligible for the content streaming path.
///
/// The global mode word must equal 2. The record then needs a low-nibble
/// category of at least 2, a nonzero resource id at byte offset 0x1bc, and
/// a clear exclusion bit (bit 5) in the word at byte offset 0x24. The
/// function returns its boolean in AL; the upper EAX bytes are not defined.
lf_checker_rt::export!(cdecl, rw_00c4edc0(record: u32) -> u32 {
    unsafe {
        const MODE_VA: u32 = 0x11d6_fd4;
        const CATEGORY: u32 = 0x1e2;
        const RESOURCE_ID: u32 = 0x1bc;
        const FLAGS: u32 = 0x24;
        let mode = lf_checker_rt::global::<u32>(MODE_VA).read_unaligned();
        if mode != 2 { return 1; }
        let category = (record.wrapping_add(CATEGORY) as *const u8).read();
        if category & 0x0f < 2 { return 1; }
        let resource_id = (record.wrapping_add(RESOURCE_ID) as *const u32).read_unaligned();
        if resource_id == 0 { return 1; }
        let flags = (record.wrapping_add(FLAGS) as *const u32).read_unaligned();
        u32::from(flags & (1 << 5) == 0)
    }
});
