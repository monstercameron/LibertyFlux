// original: 0x00DA2330 write_stream_detail
/// Copies two words from a source pair into a six-field record, stores the flag and two scalar values, and returns the copied pair's second word.
lf_checker_rt::export!(thiscall, rw_s675_00da2330(this: u32, flag_word: u32, first_value: u32, second_value: u32, pair: u32, tail_bits: u32) -> u32 {
    const FLAG: u32 = 0;
    const FIRST_VALUE: u32 = 4;
    const SECOND_VALUE: u32 = 8;
    const PAIR_FIRST: u32 = 12;
    const PAIR_SECOND: u32 = 16;
    const TAIL: u32 = 20;
    let pair_first = unsafe { (pair as *const u32).read_unaligned() };
    let pair_second = unsafe { (pair.wrapping_add(4) as *const u32).read_unaligned() };
    unsafe {
        (this.wrapping_add(FLAG) as *mut u8).write(flag_word as u8);
        (this.wrapping_add(FIRST_VALUE) as *mut u32).write_unaligned(first_value);
        (this.wrapping_add(SECOND_VALUE) as *mut u32).write_unaligned(second_value);
        (this.wrapping_add(PAIR_FIRST) as *mut u32).write_unaligned(pair_first);
        (this.wrapping_add(PAIR_SECOND) as *mut u32).write_unaligned(pair_second);
        (this.wrapping_add(TAIL) as *mut u32).write_unaligned(tail_bits);
    }
    pair_second
});
