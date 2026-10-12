// original: 0x00BDF190 CTaskList::CTaskList

/// Initializes a task list by storing its relocated vtable address and clearing
/// every 64-bit word from byte offset 4 through 0x43. The 32-bit thiscall
/// receiver is also returned in EAX.
lf_checker_rt::export!(thiscall, rw_bdf190(this: u32) -> u32 {
    unsafe {
        const TASK_LIST_VTABLE: u32 = 0x00EB90F0;
        const FIRST_FIELD_OFFSET: u32 = 4;
        const FIELD_WORDS: usize = 8;

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_LIST_VTABLE));
        let fields = this.wrapping_add(FIRST_FIELD_OFFSET) as *mut u64;
        for index in 0..FIELD_WORDS {
            fields.add(index).write_unaligned(0);
        }
        this
    }
});
