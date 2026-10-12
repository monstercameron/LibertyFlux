// original: 0x009B7AB0 input_flag_byte_of_slot (proposed)

/// Reads one word from the callee's answer block and stores whether it equals 1 into the caller's byte. Forwards its incoming ECX (which it never otherwise uses) to the callee with the key, compares the word at +0x80 of the answer, writes the 0/1 result through the output pointer and returns that pointer.
///
/// Original: 0x009B7AB0 (thiscall).
lf_checker_rt::export!(thiscall, rw_009B7AB0(this: u32, key: u32, out: u32) -> u32 {
    const WORD_OFFSET: u32 = 0x80;
    const HIT_VALUE: u32 = 1;

    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn write_byte(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_byte(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    unsafe {
        let answer = lf_checker_rt::callee_thiscall!(1, u32, this, key);
        let hit = (read_word(answer + WORD_OFFSET) == HIT_VALUE) as u8;
        write_byte(out, hit);
        out
    }
});
