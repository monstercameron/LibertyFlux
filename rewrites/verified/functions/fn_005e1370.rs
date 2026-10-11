// original: 0x005E1370 CPedDataCop::vf6
/// Return whether the 32-bit status word at offset `0x54` differs from the
/// all-ones sentinel. The thiscall method consumes two unused stack words
/// with the callee pops 8 bytes; it does not modify the object or call another function.
lf_checker_rt::export!(thiscall, rw_005e1370(this: u32, _arg0: u32, _arg1: u32) -> u32 {
    const STATUS_OFFSET: u32 = 0x54;
    const INVALID_STATUS: u32 = u32::MAX;

    let status = unsafe {
        (this.wrapping_add(STATUS_OFFSET) as *const u32).read_unaligned()
    };
    u32::from(status != INVALID_STATUS)
});
