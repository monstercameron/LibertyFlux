// original: 0x00e45d90 net_info_header_error3_ps3
// net info header init, error3_ps3 variant.
// Same shape as 0x00e45d60 with the "NET_ERROR3_PS3" second key.
export!(thiscall, rw_00e45d90(this_obj: u32) -> u32 {
    unsafe {
        const KEY0: u32 = 0x00F1_5870;
        const KEY1: u32 = 0x00F1_5880;
        *((this_obj.wrapping_add(0)) as *mut u32) = 4;
        callee_thiscall!(1, u32, this_obj, relocated(KEY0));
        let ans: u32 = callee_thiscall!(2, u32, this_obj, relocated(KEY1));
        *((this_obj.wrapping_add(8)) as *mut u8) = 1;
        *((this_obj.wrapping_add(4)) as *mut u32) = 0x10;
        ans
    }
});
