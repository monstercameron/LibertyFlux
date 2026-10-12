// original: 0x009B90F0 input_new_14_triple (proposed)

/// Allocates a 20-byte node, tags it with its vtable, clears its flag byte and stores three words in shuffled slots. Returns the node, or null when allocation fails.
///
/// Original: 0x009B90F0 (stdcall, 3 stack words).
lf_checker_rt::export!(stdcall, rw_009B90F0(first: u32, second: u32, third: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00E94188;
    const ALLOC_SIZE: u32 = 0x14;
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
        write_word(block + 0x10, third);
        write_byte(block + FLAG_OFFSET, 0);
        write_word(block, vtable);
        write_word(block + 0x0C, second);
        block
    }
});
