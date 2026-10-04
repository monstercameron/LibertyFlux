// original: 0x00da4600 dword_list_contains
// s10f14: whether a dword list contains a value (thiscall/1).
export!(thiscall, rw_s10f14(this: *const u8, val: u32) -> u8 {
    unsafe {
        let begin = *(this as *const u32);
        let end = *(this.add(4) as *const u32);
        let n = (end.wrapping_sub(begin) as i32) >> 2;
        let mut i = 0i32;
        while i < n {
            if *((begin as *const u32).offset(i as isize)) == val {
                return 1;
            }
            i += 1;
        }
        0
    }
});
