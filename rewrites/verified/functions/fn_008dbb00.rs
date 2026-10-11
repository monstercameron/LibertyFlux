// original: 0x008DBB00 task_ctor_copy_bounded_pair_arrays
/// Copies `count` pairs from each of two source arrays into destination arrays starting at +0x08 and +0x38, then stores the trailing words at +0x68, +0x6c, +0x70 and +0x74. Counts 0 through 6 keep both arrays within the documented destination extent.
///
/// The original uses the 32-bit thiscall convention; all pointer fields are read or written unaligned.
lf_checker_rt::export!(thiscall, rw_008dbb00(this: u32, source_a: u32, source_b: u32, tail_a: u32, tail_b: u32, tail_c: u32, count: u32) -> u32 {
    unsafe {
        const COOKIE_MASK: u32 = 0x3fff;
        const COOKIE_MARKER: u32 = 0x00e7e048;
        const OBJECT_VTABLE: u32 = 0x00e81c30;
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
        for pair_index in 0..count {
            let pair_offset = pair_index.wrapping_mul(8);
            let first_low = read_word(source_a.wrapping_add(pair_offset));
            let first_high = read_word(source_a.wrapping_add(pair_offset).wrapping_add(4));
            write_word(this.wrapping_add(8).wrapping_add(pair_offset), first_low);
            write_word(this.wrapping_add(12).wrapping_add(pair_offset), first_high);
            let second_low = read_word(source_b.wrapping_add(pair_offset));
            let second_high = read_word(source_b.wrapping_add(pair_offset).wrapping_add(4));
            write_word(this.wrapping_add(0x38).wrapping_add(pair_offset), second_low);
            write_word(this.wrapping_add(0x3c).wrapping_add(pair_offset), second_high);
        }
        write_word(this.wrapping_add(104u32), tail_a);
        write_word(this.wrapping_add(108u32), tail_b);
        write_word(this.wrapping_add(112u32), count);
        write_word(this.wrapping_add(116u32), tail_c);
        this
    }
});
