// original: 0x008DB970 task_ctor_copy_four_float_records_b
/// Copies four 16-byte source records into +0x10 through +0x4c and stores two trailing words at +0x50 and +0x54. Float fields are copied bit-for-bit.
///
/// The original uses the 32-bit thiscall convention; all pointer fields are read or written unaligned.
lf_checker_rt::export!(thiscall, rw_008db970(this: u32, record_a: u32, record_b: u32, record_c: u32, record_d: u32, tail_a: u32, tail_b: u32) -> u32 {
    unsafe {
        const COOKIE_MASK: u32 = 0x3fff;
        const COOKIE_MARKER: u32 = 0x00e7e048;
        const OBJECT_VTABLE: u32 = 0x00e81cf4;
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
        for word_index in 0..4u32 {
            let value = read_word(record_a.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(16u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        for word_index in 0..4u32 {
            let value = read_word(record_b.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(32u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        for word_index in 0..4u32 {
            let value = read_word(record_c.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(48u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        for word_index in 0..4u32 {
            let value = read_word(record_d.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(64u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        write_word(this.wrapping_add(80u32), tail_b);
        write_word(this.wrapping_add(84u32), tail_a);
        this
    }
});
