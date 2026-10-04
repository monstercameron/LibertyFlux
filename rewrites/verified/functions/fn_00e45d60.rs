// original: 0x00e45d60 net_info_header_error4_pc
// net info header init, error4_pc variant.
// Seeds the header fields (kind 4, size 0x10, present flag) and appends the
// "NET_INFO_HEADER" then "NET_ERROR4_PC" keys. Key addresses carry
// relocation entries, so they are resolved for the loaded image base.
// Returns the second append's answer.
export!(thiscall, rw_00e45d60(this_obj: u32) -> u32 {
    unsafe {
        const KEY0: u32 = 0x00F1_58E4;
        const KEY1: u32 = 0x00F1_58F4;
        *((this_obj.wrapping_add(0)) as *mut u32) = 4;
        callee_thiscall!(1, u32, this_obj, relocated(KEY0));
        let ans: u32 = callee_thiscall!(2, u32, this_obj, relocated(KEY1));
        *((this_obj.wrapping_add(8)) as *mut u8) = 1;
        *((this_obj.wrapping_add(4)) as *mut u32) = 0x10;
        ans
    }
});
