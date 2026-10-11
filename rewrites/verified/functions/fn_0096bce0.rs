// original: 0x0096BCE0 has_timing_transition

/// Returns true when either of the two adjacent transition bytes at object
/// offsets `0x1268` and `0x1269` is nonzero. The result is an AL boolean.
lf_checker_rt::export!(thiscall, rw_0096bce0(this: u32) -> u32 {
    unsafe {
        let first = (this.wrapping_add(0x1268) as *const u8).read();
        let second = (this.wrapping_add(0x1269) as *const u8).read();
        u32::from(first != 0 || second != 0)
    }
});
