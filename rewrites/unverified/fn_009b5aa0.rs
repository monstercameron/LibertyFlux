// original: 0x009B5AA0 script_state_copy_selected_vectors

/// Update packed script-state flags and copy two selected vector records.
///
/// `this` is the script-state object. Two direct helper calls receive four
/// stack arguments and return full EAX values; their low bits replace flag
/// bits 7 and 0 at offsets `0xd1` and `0xd2`. When the first returned low bit
/// is set, two manager lookups (selectors `0x27` and `0x26`) return source
/// records. Each source contributes four 32-bit words from `+0x40..+0x4c` to
/// the corresponding destination at `+0xa0..+0xac` or `+0xb0..+0xbc`.
/// Copied floating-point fields are moved as raw bits. The routine returns
/// the second helper's EAX value when the copy path is skipped, or the final
/// source record's last word when the copy path runs.
lf_checker_rt::export!(thiscall, rw_009b5aa0(this: u32) -> u32 {
    unsafe {
        const FLAG_A: u32 = 0xd1;
        const FLAG_B: u32 = 0xd2;
        const GLOBAL_MANAGER: u32 = 0x0103_e49c;
        const SOURCE_WORDS: [u32; 4] = [0x40, 0x44, 0x48, 0x4c];
        const FIRST_DESTINATION: [u32; 4] = [0xa0, 0xa4, 0xa8, 0xac];
        const SECOND_DESTINATION: [u32; 4] = [0xb0, 0xb4, 0xb8, 0xbc];

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn copy_words(destination: u32, source: u32, destination_words: [u32; 4]) {
            unsafe {
                for (source_offset, destination_offset) in SOURCE_WORDS.into_iter().zip(destination_words) {
                    let word = read_u32(source.wrapping_add(source_offset));
                    write_u32(destination.wrapping_add(destination_offset), word);
                }
            }
        }

        let first_answer = lf_checker_rt::callee_thiscall!(1, u32, this, 0x27u32, 5u32, 0u32, 0u32);
        let old_flag_a = read_u8(this.wrapping_add(FLAG_A));
        let new_flag_a = (old_flag_a & 0x7f) | (first_answer as u8).wrapping_shl(7);
        write_u8(this.wrapping_add(FLAG_A), new_flag_a);

        let second_answer = lf_checker_rt::callee_thiscall!(2, u32, this, 0x26u32, 5u32, 0u32, 0u32);
        let old_flag_b = read_u8(this.wrapping_add(FLAG_B));
        let differing_low_bit = (old_flag_b ^ second_answer as u8) & 1;
        write_u8(this.wrapping_add(FLAG_B), old_flag_b ^ differing_low_bit);

        if read_u8(this.wrapping_add(FLAG_A)) & 0x80 == 0 {
            return second_answer;
        }

        let manager = read_u32(lf_checker_rt::relocated(GLOBAL_MANAGER));
        let first_source = lf_checker_rt::callee_thiscall!(3, u32, manager, 0x27u32, 0u32);
        copy_words(this, first_source, FIRST_DESTINATION);

        let second_source = lf_checker_rt::callee_thiscall!(4, u32, manager, 0x26u32, 0u32);
        copy_words(this, second_source, SECOND_DESTINATION);
        read_u32(second_source.wrapping_add(0x4c))
    }
});
