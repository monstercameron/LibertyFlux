// original: 0x009B8690 veh_set_53c_new_float (proposed)

/// Writes one float's bits into the receiver first, then allocates a 12-byte node tagged with its vtable, clears its flag byte and stores the value there too. Returns the node, or null when allocation fails (the receiver write still happened).
///
/// Original: 0x009B8690 (thiscall, 1 stack word).
lf_checker_rt::export!(thiscall, rw_009B8690(this: u32, value: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00E944D8;
    const ALLOC_SIZE: u32 = 0xC;
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
        write_word(this + 0x53C, value);
        let block = lf_checker_rt::callee_cdecl!(1, u32, ALLOC_SIZE);
        if block == 0 { return 0; }
        let vtable = lf_checker_rt::relocated(VTABLE_VA);
        write_byte(block + FLAG_OFFSET, 0);
        write_word(block, vtable);
        write_word(block + 0x08, value);
        block
    }
});
