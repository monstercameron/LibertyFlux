// original: 0x00C0FA20 UILayoutFrame::vf36
/// Replace the float bits at `this + 0xa4`. When the signed integer
/// identifier at `this + 0xa8` changes, update it and clear the cache at
/// `this + 0xac`; otherwise preserve the cache. The method takes `this` in
/// ECX and cleans the float word followed by the identifier word.
lf_checker_rt::export!(thiscall, rw_c0fa20(this: u32, value_bits: u32, identifier: u32) -> u32 {
    unsafe {
        const VALUE: u32 = 0x0a4;
        const IDENTIFIER: u32 = 0x0a8;
        const CACHE: u32 = 0x0ac;
        (this.wrapping_add(VALUE) as *mut u32).write_unaligned(value_bits);
        let old_identifier = (this.wrapping_add(IDENTIFIER) as *const u32).read_unaligned();
        if old_identifier != identifier {
            (this.wrapping_add(IDENTIFIER) as *mut u32).write_unaligned(identifier);
            (this.wrapping_add(CACHE) as *mut u32).write_unaligned(0);
        }
        0
    }
});
