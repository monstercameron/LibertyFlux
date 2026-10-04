// original: 0x00e45dc0 net_info_header_info2
// net info header init, info2 variant.
// Same shape with kind 3, size 4 and the "NET_INFO2" second key.
export!(thiscall, rw_00e45dc0(this_obj: u32) -> u32 {
    unsafe {
        const KEY0: u32 = 0x00F1_58C8;
        const KEY1: u32 = 0x00F1_58D8;
        *((this_obj.wrapping_add(0)) as *mut u32) = 3;
        callee_thiscall!(1, u32, this_obj, relocated(KEY0));
        let ans: u32 = callee_thiscall!(2, u32, this_obj, relocated(KEY1));
        *((this_obj.wrapping_add(8)) as *mut u8) = 1;
        *((this_obj.wrapping_add(4)) as *mut u32) = 4;
        ans
    }
});
