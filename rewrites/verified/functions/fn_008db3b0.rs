// original: 0x008DB3B0 task_ctor_copy_five_pairs
/// Copies five two-word source records to consecutive fields at +0x08 through +0x2c, then stores three trailing words at +0x30, +0x34 and +0x38. The destination is returned. Every pointer argument addresses two readable words.
///
/// The original uses the 32-bit thiscall convention; all pointer fields are read or written unaligned.
lf_checker_rt::export!(thiscall, rw_008db3b0(this: u32, pair_a: u32, pair_b: u32, pair_c: u32, pair_d: u32, pair_e: u32, tail_a: u32, tail_b: u32, tail_c: u32) -> u32 {
    unsafe {
        const COOKIE_MASK: u32 = 0x3fff;
        const COOKIE_MARKER: u32 = 0x00e7e048;
        const OBJECT_VTABLE: u32 = 0x00e81cd8;
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
        for word_index in 0..2u32 {
            let value = read_word(pair_a.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(8u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        for word_index in 0..2u32 {
            let value = read_word(pair_b.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(16u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        for word_index in 0..2u32 {
            let value = read_word(pair_c.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(24u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        for word_index in 0..2u32 {
            let value = read_word(pair_d.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(32u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        for word_index in 0..2u32 {
            let value = read_word(pair_e.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(40u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        write_word(this.wrapping_add(48u32), tail_a);
        write_word(this.wrapping_add(52u32), tail_b);
        write_word(this.wrapping_add(56u32), tail_c);
        this
    }
});
