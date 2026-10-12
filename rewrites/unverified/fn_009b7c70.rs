// original: 0x009B7C70 input_attach_entry_to_list (proposed)

/// Creates a list entry for the object and links it in. Resolves the inner object at +0x118 of the first argument, creates the entry through the factory callee, marks its flag byte with bits 0xc, attaches it through the link callee with (first, second, this-word, 0, 1), then initialises three header words. Returns the link callee's answer.
///
/// Original: 0x009B7C70 (thiscall).
lf_checker_rt::export!(thiscall, rw_009B7C70(this: u32, first: u32, second: u32) -> u32 {
    const INNER_OFFSET: u32 = 0x118;
    const THIS_WORD: u32 = 0x528;
    const FLAG_OFFSET: u32 = 0x13C;

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
        let inner = read_word(first + INNER_OFFSET);
        let target = lf_checker_rt::callee_thiscall!(1, u32, inner, 3, 0);
        write_byte(target + FLAG_OFFSET, read_byte(target + FLAG_OFFSET) | 0x0C);
        let extra = read_word(this + THIS_WORD);
        let answer = lf_checker_rt::callee_thiscall!(2, u32, target, first, second, extra, 0, 1);
        write_word(target + 0x16C, 1);
        write_word(target + 0x164, 1);
        write_word(target + 0x168, 0);
        answer
    }
});
