// original: 0x008B95E0 menu_pair_flag_check

/// Test two keyed bytes of a menu record against the ASCII limit.
///
/// XORs the bytes at `FIRST` and `SECOND` with the key byte at `KEY` and
/// returns 1 only when the first result is ASCII (at most 0x7F) and the
/// second is not; otherwise 0. Only the low byte of the return is set; the
/// upper 24 bits keep whatever the caller left in EAX. Thiscall, object
/// in ECX.
lf_checker_rt::export!(thiscall, rw_008B95E0(this: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x04;
        const FIRST: u32 = 0x06;
        const SECOND: u32 = 0x07;
        const ASCII_MAX: u8 = 0x7F;
        let key = (this.wrapping_add(KEY) as *const u8).read_unaligned();
        let first = (this.wrapping_add(FIRST) as *const u8).read_unaligned() ^ key;
        if first > ASCII_MAX {
            return 0;
        }
        let second = (this.wrapping_add(SECOND) as *const u8).read_unaligned() ^ key;
        u32::from(second > ASCII_MAX)
    }
});
