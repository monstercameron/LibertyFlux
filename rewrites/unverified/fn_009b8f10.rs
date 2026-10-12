// original: 0x009B8F10 input_lookup_new_key_float (proposed)

/// Looks the key up first and returns null when the lookup misses; otherwise allocates a 16-byte node tagged with its vtable, clears its flag byte and stores a key word and one float's bits. Returns the node, or null when allocation fails.
///
/// Original: 0x009B8F10 (stdcall, 2 stack words).
lf_checker_rt::export!(stdcall, rw_009B8F10(key: u32, value: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00E944E8;
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
        let target = lf_checker_rt::callee_stdcall!(2, u32, key);
        if target == 0 { return 0; }
        let block = lf_checker_rt::callee_cdecl!(1, u32, ALLOC_SIZE);
        if block == 0 { return 0; }
        let vtable = lf_checker_rt::relocated(VTABLE_VA);
        write_word(block + 0x08, key);
        write_byte(block + FLAG_OFFSET, 0);
        write_word(block, vtable);
        write_word(block + 0x0C, value);
        block
    }
});
