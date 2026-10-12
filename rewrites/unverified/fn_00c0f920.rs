// original: 0x00C0F920 UILayoutFrame::vf30
/// Replace two cached float values and their signed integer identifiers.
/// Each cached dword is cleared only when its identifier changes. The four
/// stack words follow the original order: float bits, first identifier,
/// float bits, second identifier. The method takes `this` in ECX and cleans
/// all four stack words.
lf_checker_rt::export!(thiscall, rw_c0f920(this: u32, first_value_bits: u32, first_id: u32, second_value_bits: u32, second_id: u32) -> u32 {
    unsafe {
        const FIRST_VALUE: u32 = 0x0a4;
        const FIRST_ID: u32 = 0x0a8;
        const FIRST_CACHE: u32 = 0x0ac;
        const SECOND_VALUE: u32 = 0x0b0;
        const SECOND_ID: u32 = 0x0b4;
        const SECOND_CACHE: u32 = 0x0b8;
        (this.wrapping_add(FIRST_VALUE) as *mut u32).write_unaligned(first_value_bits);
        let old_first_id = (this.wrapping_add(FIRST_ID) as *const u32).read_unaligned();
        if old_first_id != first_id {
            (this.wrapping_add(FIRST_ID) as *mut u32).write_unaligned(first_id);
            (this.wrapping_add(FIRST_CACHE) as *mut u32).write_unaligned(0);
        }
        (this.wrapping_add(SECOND_VALUE) as *mut u32).write_unaligned(second_value_bits);
        let old_second_id = (this.wrapping_add(SECOND_ID) as *const u32).read_unaligned();
        if old_second_id != second_id {
            (this.wrapping_add(SECOND_ID) as *mut u32).write_unaligned(second_id);
            (this.wrapping_add(SECOND_CACHE) as *mut u32).write_unaligned(0);
        }
        0
    }
});
