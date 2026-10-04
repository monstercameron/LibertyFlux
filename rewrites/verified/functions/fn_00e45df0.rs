// original: 0x00e45df0 net_info_header_info1
// net info header init, info1 variant.
// Same shape with kind 5, size 4 and the "NET_INFO1" second key.
export!(thiscall, rw_00e45df0(this_obj: u32) -> u32 {
    unsafe {
        const KEY0: u32 = 0x00F1_5890;
        const KEY1: u32 = 0x00F1_58A0;
        *((this_obj.wrapping_add(0)) as *mut u32) = 5;
        callee_thiscall!(1, u32, this_obj, relocated(KEY0));
        let ans: u32 = callee_thiscall!(2, u32, this_obj, relocated(KEY1));
        *((this_obj.wrapping_add(8)) as *mut u8) = 1;
        *((this_obj.wrapping_add(4)) as *mut u32) = 4;
        ans
    }
});
