// original: 0x008DB450 task_ctor_copy_status_from_subject
/// Initialises the object header, asks the shared task helper to update a
/// source counter at offset 0x74, then copies the word at source offset 0x70
/// into object offset 0x08. The source pointer addresses at least 0x78 bytes.
/// This is a 32-bit thiscall constructor and returns the destination pointer.
lf_checker_rt::export!(thiscall, rw_008db450(this: u32, source: u32) -> u32 {
    unsafe {
        const COOKIE_MASK: u32 = 0x3fff;
        const COOKIE_MARKER: u32 = 0x00e7e048;
        const OBJECT_VTABLE: u32 = 0x00e81dd4;
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

        let shared_context = lf_checker_rt::relocated(0x011d4ec8);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, shared_context, source);
        let copied_status = read_word(source.wrapping_add(0x70));
        write_word(this.wrapping_add(8), copied_status);
        this
    }
});
