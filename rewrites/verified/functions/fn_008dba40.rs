// original: 0x008DBA40 task_ctor_copy_eight_pairs
/// Copies eight two-word source records into +0x08 through +0x44, then stores two trailing words at +0x48 and +0x4c. The destination is returned.
///
/// The original uses the 32-bit thiscall convention; all pointer fields are read or written unaligned.
lf_checker_rt::export!(thiscall, rw_008dba40(this: u32, pair_1: u32, pair_2: u32, pair_3: u32, pair_4: u32, pair_5: u32, pair_6: u32, pair_7: u32, pair_8: u32, tail_a: u32, tail_b: u32) -> u32 {
    unsafe {
        const COOKIE_MASK: u32 = 0x3fff;
        const COOKIE_MARKER: u32 = 0x00e7e048;
        const OBJECT_VTABLE: u32 = 0x00e81c84;
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
            let value = read_word(pair_1.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(8u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        for word_index in 0..2u32 {
            let value = read_word(pair_2.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(16u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        for word_index in 0..2u32 {
            let value = read_word(pair_3.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(24u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        for word_index in 0..2u32 {
            let value = read_word(pair_4.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(32u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        for word_index in 0..2u32 {
            let value = read_word(pair_5.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(40u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        for word_index in 0..2u32 {
            let value = read_word(pair_6.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(48u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        for word_index in 0..2u32 {
            let value = read_word(pair_7.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(56u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        for word_index in 0..2u32 {
            let value = read_word(pair_8.wrapping_add(0u32).wrapping_add(word_index.wrapping_mul(4)));
            write_word(this.wrapping_add(64u32).wrapping_add(word_index.wrapping_mul(4)), value);
        }
        write_word(this.wrapping_add(72u32), tail_b);
        write_word(this.wrapping_add(76u32), tail_a);
        this
    }
});
