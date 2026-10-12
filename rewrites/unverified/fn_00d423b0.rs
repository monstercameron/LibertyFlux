// original: 0x00D423B0 task_node_676_reset

/// Clear the trailing word at offset +0x280 of a large task-node record.
/// ECX points to the record; all other bytes are unchanged and EAX returns
/// the original pointer.
lf_checker_rt::export!(thiscall, rw_D423B0(this: u32) -> u32 {
    const TRAILING_WORD: u32 = 0x280;
    unsafe { core::ptr::write_unaligned((this + TRAILING_WORD) as *mut u32, 0); }
    this
});
