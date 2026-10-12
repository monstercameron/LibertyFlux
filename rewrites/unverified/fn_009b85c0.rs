// original: 0x009B85C0 veh_new_0c_float_a (proposed)

/// Allocates a 12-byte node, tags it with its vtable, clears its flag byte and stores one float's bits. Returns the node, or null when allocation fails.
///
/// Original: 0x009B85C0 (stdcall, 1 stack word).
lf_checker_rt::export!(stdcall, rw_009B85C0(value: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00E946A8;
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
        let block = lf_checker_rt::callee_cdecl!(1, u32, ALLOC_SIZE);
        if block == 0 { return 0; }
        let vtable = lf_checker_rt::relocated(VTABLE_VA);
        write_byte(block + FLAG_OFFSET, 0);
        write_word(block, vtable);
        write_word(block + 0x08, value);
        block
    }
});
