// original: 0x00B80D70 ped_task_manager_last_active_node

/// Inspect three task-list heads at `+0x2c`, `+0x30`, and `+0x34` in order.
/// The first head whose node flag at `+0x0c` has bit zero set selects that
/// list; return its last linked node by following `+0x08`. Return null when
/// none of the three heads has an active flag.
lf_checker_rt::export!(thiscall, rw_b80d70(this: u32) -> u32 {
    unsafe {
        const HEADS: u32 = 0x2c;
        const HEAD_COUNT: u32 = 3;
        const NODE_NEXT: u32 = 0x08;
        const NODE_FLAGS: u32 = 0x0c;
        const ACTIVE: u8 = 1;

        for index in 0..HEAD_COUNT {
            let head = ((this + HEADS + index * 4) as *const u32).read_unaligned();
            if head == 0 || (((head + NODE_FLAGS) as *const u8).read() & ACTIVE) == 0 {
                continue;
            }
            let mut last = head;
            loop {
                let next = ((last + NODE_NEXT) as *const u32).read_unaligned();
                if next == 0 {
                    return last;
                }
                last = next;
            }
        }
        0
    }
});
