// original: 0x00897EA0 input_slot_table_init

/// Initialises the input slot state in the globals: clears the flag byte at 0x115F804 and
/// sets the first slot-table word at 0x115F808 to one, with the word after it at
/// 0x115F80C set to zero. Cdecl with no arguments and no return value.
lf_checker_rt::export!(cdecl, rw_00897ea0() -> () {
    unsafe {
        lf_checker_rt::global::<u8>(0x0115_F804).write(0);
        lf_checker_rt::global::<u32>(0x0115_F808).write(1);
        lf_checker_rt::global::<u32>(0x0115_F80C).write(0);
    }
});
