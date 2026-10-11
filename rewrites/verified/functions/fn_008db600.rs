// original: 0x008DB600 task_ctor_copy_eight_words
/// Copies eight words from the source block into +0x08 through +0x24 and stores two trailing words at +0x28 and +0x2c. The destination is returned.
///
/// The original uses the 32-bit thiscall convention; all pointer fields are read or written unaligned.
lf_checker_rt::export!(thiscall, rw_008db600(this: u32, source: u32, mode: u32, flags: u32) -> u32 {
    unsafe {
        const COOKIE_MASK: u32 = 0x3fff;
        const COOKIE_MARKER: u32 = 0x00e7e048;
        const OBJECT_VTABLE: u32 = 0x00e81bf8;
        const COUNTER_FILE_VA: u32 = 0x010327a0;
        #[inline(always)] unsafe fn read_word(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)] unsafe fn write_word(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
        #[inline(always)] unsafe fn read_byte(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
        #[inline(always)] unsafe fn write_byte(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }
        let counter = lf_checker_rt::global::<u32>(COUNTER_FILE_VA);
        let current_count = counter.read_unaligned();
        let old_cookie = read_word(this.wrapping_add(4));
        write_word(this, lf_checker_rt::relocated(COOKIE_MARKER));
        write_word(this.wrapping_add(4), old_cookie ^ ((old_cookie ^ current_count) & COOKIE_MASK));
        counter.write_unaligned(current_count.wrapping_add(1));
        write_word(this, lf_checker_rt::relocated(OBJECT_VTABLE));
        for word_index in 0..8u32 {
            let value = read_word(source.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(8u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        write_word(this.wrapping_add(40u32), mode);
        write_word(this.wrapping_add(44u32), flags);
        this
    }
});
