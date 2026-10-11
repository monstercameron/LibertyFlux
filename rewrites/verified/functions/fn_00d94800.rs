// original: 0x00D94800 mainloop_timing_test_record_state

/// Return whether the optional record pointer satisfies the state predicate.
///
/// The fifth cdecl stack argument points to two words. A null pointer passes.
/// Otherwise the first word must have either bit `0x2000` or `0x4000` set,
/// and bit 28 of the second word must be clear. The result is an 8-bit boolean;
/// the caller removes all five argument words from the stack.
lf_checker_rt::export!(cdecl, rw_00d94800(_unused0: u32, _unused1: u32, _unused2: u32, _unused3: u32, record: u32) -> u32 {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    if record == 0 {
        return 1;
    }
    unsafe {
        const STATUS_BITS: u32 = 0x6000;
        const ATTRIBUTE_BIT: u32 = 0x1000_0000;
        let status = read_u32(record);
        if status & STATUS_BITS == 0 {
            return 1;
        }
        u32::from(read_u32(record.wrapping_add(4)) & ATTRIBUTE_BIT == 0)
    }
});
