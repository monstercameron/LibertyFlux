// original: 0x009B9200 input_new_24_mixed7 (proposed)

/// Allocates a 36-byte node, tags it with its vtable, clears its flag byte and stores mixed words and floats in shuffled slots. Returns the node, or null when allocation fails.
///
/// Original: 0x009B9200 (stdcall, 7 stack words).
lf_checker_rt::export!(stdcall, rw_009B9200(head: u32, f0: u32, f1: u32, f2: u32, f3: u32, tail0: u32, tail1: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00E94298;
    const ALLOC_SIZE: u32 = 0x24;
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
        write_word(block + 0x0C, f0);
        write_word(block + 0x10, f1);
        write_word(block + 0x1C, tail0);
        write_word(block + 0x14, f2);
        write_word(block + 0x20, tail1);
        write_byte(block + FLAG_OFFSET, 0);
        write_word(block, vtable);
        write_word(block + 0x08, head);
        write_word(block + 0x18, f3);
        block
    }
});
