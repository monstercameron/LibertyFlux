// original: 0x009B7D00 input_new_10_pair_b (proposed)

/// Allocates a 16-byte node, tags it with its vtable, clears its flag byte and stores two words. Returns the node, or null when allocation fails.
///
/// Original: 0x009B7D00 (stdcall, 2 stack words).
lf_checker_rt::export!(stdcall, rw_009B7D00(first: u32, second: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00E94418;
    const ALLOC_SIZE: u32 = 0x10;
    const FLAG_OFFSET: u32 = 0x04;

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

    unsafe {
        let block = lf_checker_rt::callee_cdecl!(1, u32, ALLOC_SIZE);
        if block == 0 { return 0; }
        let vtable = lf_checker_rt::relocated(VTABLE_VA);
        write_word(block + 0x08, first);
        write_byte(block + FLAG_OFFSET, 0);
        write_word(block, vtable);
        write_word(block + 0x0C, second);
        block
    }
});
