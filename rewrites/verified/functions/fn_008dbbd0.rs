// original: 0x008DBBD0 task_ctor_register_job
/// Initialises the object header, registers the masked cookie with a shared manager, resets the common constructor counter to one, and returns the destination.
/// This follows the original 32-bit thiscall convention and returns the destination pointer.
lf_checker_rt::export!(thiscall, rw_008dbbd0(this: u32) -> u32 {
    unsafe {
        const COOKIE_MASK: u32 = 0x3fff;
        const COOKIE_MARKER: u32 = 0x00e7e048;
        const OBJECT_VTABLE: u32 = 0x00e81840;
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
        const REGISTRY_CONTEXT: u32 = 0x011d4ec8;
        let masked_cookie = read_word(this.wrapping_add(4)) & COOKIE_MASK;
        let shared_context = lf_checker_rt::relocated(REGISTRY_CONTEXT);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, shared_context, masked_cookie);
        counter.write_unaligned(1);
        this
    }
});
