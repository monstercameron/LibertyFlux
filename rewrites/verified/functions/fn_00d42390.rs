// original: 0x00D42390 task_node_12_init

/// Initialize a short task-node record: clear its two link words at +4 and
/// +8, store the invalid id at +0, and return the record pointer from ECX.
lf_checker_rt::export!(thiscall, rw_D42390(this: u32) -> u32 {
    const NODE_TYPE: u32 = 0;
    const LINK_A: u32 = 4;
    const LINK_B: u32 = 8;
    unsafe {
        core::ptr::write_unaligned((this + LINK_B) as *mut u32, 0);
        core::ptr::write_unaligned((this + LINK_A) as *mut u32, 0);
        core::ptr::write_unaligned((this + NODE_TYPE) as *mut u32, u32::MAX);
    }
    this
});
