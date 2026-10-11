// original: 0x008DBEC0 task_ctor_attach_to_scheduler
/// Stores the supplied scheduler value at object offset 0x0c, snapshots the current task identifier into offset 0x08, records the supplied value in a global slot, increments the task identifier, and calls the shared scheduler with that value and the preceding global cursor minus 0x10.
/// This follows the original 32-bit thiscall convention and returns the destination pointer.
lf_checker_rt::export!(thiscall, rw_008dbec0(this: u32, scheduler_value: u32) -> u32 {
    unsafe {
        const COOKIE_MASK: u32 = 0x3fff;
        const COOKIE_MARKER: u32 = 0x00e7e048;
        const OBJECT_VTABLE: u32 = 0x00e81824;
        const COUNTER_FILE_VA: u32 = 0x010327a0;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let counter = lf_checker_rt::global::<u32>(COUNTER_FILE_VA);
        let current_count = counter.read_unaligned();
        let old_cookie = read_word(this.wrapping_add(4));
        write_word(this, lf_checker_rt::relocated(COOKIE_MARKER));
        write_word(
            this.wrapping_add(4),
            old_cookie ^ ((old_cookie ^ current_count) & COOKIE_MASK),
        );
        counter.write_unaligned(current_count.wrapping_add(1));
        write_word(this, lf_checker_rt::relocated(OBJECT_VTABLE));
        const TASK_ID_FILE_VA: u32 = 0x01175c50;
        const TASK_VALUE_FILE_VA: u32 = 0x01175c84;
        const TASK_CURSOR_FILE_VA: u32 = 0x01175c74;
        write_word(this.wrapping_add(0x0c), scheduler_value);
        let task_id = lf_checker_rt::global::<u32>(TASK_ID_FILE_VA);
        let current_id = task_id.read_unaligned();
        write_word(this.wrapping_add(8), current_id);
        lf_checker_rt::global::<u32>(TASK_VALUE_FILE_VA).write_unaligned(scheduler_value);
        let previous_cursor = lf_checker_rt::global::<u32>(TASK_CURSOR_FILE_VA)
            .read_unaligned().wrapping_sub(0x10);
        let shared_context = lf_checker_rt::relocated(0x011d4ec8);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, shared_context, scheduler_value, previous_cursor);
        task_id.write_unaligned(current_id.wrapping_add(1));
        this
    }
});
