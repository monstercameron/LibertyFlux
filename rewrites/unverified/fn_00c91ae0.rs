// original: 0x00C91AE0 StoreIkTaskFloatsAndFlags

/// Write two float bit patterns and an enabled-flags word into the thiscall receiver. The first stack word is stored at byte offset 0xF8, the second at 0xF4, and the third is ORed with ENABLED_FLAG before it is stored at 0xF0 and returned in EAX. The callee removes all three stack words.
lf_checker_rt::export!(thiscall, rw_00c91ae0(this: u32, first_float_bits: u32, second_float_bits: u32, flags: u32) -> u32 {
    const FIRST_FLOAT: u32 = 0xF8;
    const SECOND_FLOAT: u32 = 0xF4;
    const FLAGS: u32 = 0xF0;
    const ENABLED_FLAG: u32 = 1;
    let active_flags = flags | ENABLED_FLAG;
    unsafe {
        (this.wrapping_add(FIRST_FLOAT) as *mut u32).write_unaligned(first_float_bits);
        (this.wrapping_add(FLAGS) as *mut u32).write_unaligned(active_flags);
        (this.wrapping_add(SECOND_FLOAT) as *mut u32).write_unaligned(second_float_bits);
    }
    active_flags
});
