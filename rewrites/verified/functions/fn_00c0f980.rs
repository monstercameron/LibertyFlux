// original: 0x00C0F980 UILayoutFrame::vf31
/// Replace the first cached float, then the second cached float, and clear
/// each cache word only when its signed integer identifier changes. The
/// stack words are ordered as first float bits, second float bits, then the
/// shared identifier. The method takes `this` in ECX and cleans three words.
lf_checker_rt::export!(thiscall, rw_c0f980(this: u32, first_value_bits: u32, second_value_bits: u32, identifier: u32) -> u32 {
    unsafe {
        const FIRST_VALUE: u32 = 0x0a4;
        const FIRST_ID: u32 = 0x0a8;
        const FIRST_CACHE: u32 = 0x0ac;
        const SECOND_VALUE: u32 = 0x0b0;
        const SECOND_ID: u32 = 0x0b4;
        const SECOND_CACHE: u32 = 0x0b8;
        (this.wrapping_add(FIRST_VALUE) as *mut u32).write_unaligned(first_value_bits);
        let old_first_id = (this.wrapping_add(FIRST_ID) as *const u32).read_unaligned();
        if old_first_id != identifier {
            (this.wrapping_add(FIRST_ID) as *mut u32).write_unaligned(identifier);
            (this.wrapping_add(FIRST_CACHE) as *mut u32).write_unaligned(0);
        }
        (this.wrapping_add(SECOND_VALUE) as *mut u32).write_unaligned(second_value_bits);
        let old_second_id = (this.wrapping_add(SECOND_ID) as *const u32).read_unaligned();
        if old_second_id != identifier {
            (this.wrapping_add(SECOND_ID) as *mut u32).write_unaligned(identifier);
            (this.wrapping_add(SECOND_CACHE) as *mut u32).write_unaligned(0);
        }
        0
    }
});
