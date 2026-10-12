// original: 0x00B80D70 ped_task_manager_last_active_node

/// Inspect the three task lists in order. Skip null heads and heads whose low status bit is set. For the first head whose bit is clear, follow its links to the null terminator and return that terminal node. Return zero when all three heads are skipped.
lf_checker_rt::export!(thiscall, rw_b80d70(this: u32) -> u32 {
    unsafe {
        const HEADS_OFFSET: u32 = 0x2c;
        const HEAD_COUNT: u32 = 3;
        const NODE_NEXT_OFFSET: u32 = 0x08;
        const NODE_FLAGS_OFFSET: u32 = 0x0c;
        const SKIP_BIT: u8 = 1;

        for list_index in 0..HEAD_COUNT {
            let head = ((this + HEADS_OFFSET + list_index * 4) as *const u32).read_unaligned();
            if head == 0 {
                continue;
            }
            let flags = ((head + NODE_FLAGS_OFFSET) as *const u8).read();
            if (flags & SKIP_BIT) != 0 {
                continue;
            }

            let mut last_node = head;
            loop {
                let next_node = ((last_node + NODE_NEXT_OFFSET) as *const u32).read_unaligned();
                if next_node == 0 {
                    return last_node;
                }
                last_node = next_node;
            }
        }
        0
    }
});
