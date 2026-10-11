// original: 0x008DBF60 task_ctor_update_thread_range
/// Computes the signed difference between two endpoints, shifts it arithmetically right by one and doubles it, stores that result at object offset 0x08, temporarily changes the selected thread-local block's word at offset 0x08, invokes the shared range helper only for a nonzero result, and restores the thread-local word before returning.
/// This follows the original 32-bit thiscall convention and returns the destination pointer.
lf_checker_rt::export!(thiscall, rw_008dbf60(this: u32, range_start: u32, range_end: u32) -> u32 {
    unsafe {
        const COOKIE_MASK: u32 = 0x3fff;
        const COOKIE_MARKER: u32 = 0x00e7e048;
        const OBJECT_VTABLE: u32 = 0x00e81b50;
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
        const TLS_INDEX_FILE_VA: u32 = 0x017aba14;
        const TLS_MARKER: u32 = 0x010327a4;
        let tls_index = lf_checker_rt::global::<u32>(TLS_INDEX_FILE_VA).read_unaligned() as usize;
        let thread_block = lf_checker_rt::tls_slot(tls_index);
        let saved_thread_value = read_word(thread_block.wrapping_add(8));
        write_word(thread_block.wrapping_add(8), TLS_MARKER);
        let signed_difference = range_end.wrapping_sub(range_start) as i32;
        let adjusted_range = ((signed_difference >> 1) as u32).wrapping_add((signed_difference >> 1) as u32);
        write_word(this.wrapping_add(8), adjusted_range);
        if adjusted_range != 0 {
            const RANGE_CONTEXT_FILE_VA: u32 = 0x01175c58;
            let shared_context = lf_checker_rt::relocated(RANGE_CONTEXT_FILE_VA);
            let _ = lf_checker_rt::callee_thiscall!(1, u32, shared_context, range_start, adjusted_range);
        }
        write_word(thread_block.wrapping_add(8), saved_thread_value);
        this
    }
});
