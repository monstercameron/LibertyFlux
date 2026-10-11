// original: 0x00A638F0 has_task_type_0xf1

/// Search the linked task list rooted at `this + 0x2e0` for a task whose
/// type word at `+4` is `0xf1`. The link is at `+0x0c`; return a one-byte
/// boolean in AL. Each node's unchanged flags word is read twice by the
/// original, so its two extracted three-bit values are equal and the type
/// comparison is reached for every node.
lf_checker_rt::export!(thiscall, rw_00a638f0(this: u32) -> u32 {
    const TASK_HEAD: u32 = 0x2e0;
    const TASK_TYPE: u32 = 0x04;
    const TASK_NEXT: u32 = 0x0c;
    const TARGET_TYPE: u32 = 0xf1;
    let mut task = unsafe { (this.wrapping_add(TASK_HEAD) as *const u32).read_unaligned() };
    while task != 0 {
        let task_type = unsafe { (task.wrapping_add(TASK_TYPE) as *const u32).read_unaligned() };
        if task_type == TARGET_TYPE { return 1; }
        task = unsafe { (task.wrapping_add(TASK_NEXT) as *const u32).read_unaligned() };
    }
    0
});
