// original: 0x00da42c0 index_of_dword_or_neg1
// s10f09: index of a dword in a list, or -1 (thiscall/1).
//
// Linear search over the begin/end dword range at +0x24/+0x28. An empty or
// malformed range (signed count <= 0) misses.
export!(thiscall, rw_s10f09(this: *const u8, val: u32) -> i32 {
    unsafe {
        let begin = *(this.add(0x24) as *const u32);
        let end = *(this.add(0x28) as *const u32);
        let n = (end.wrapping_sub(begin) as i32) >> 2;
        let mut i = 0i32;
        while i < n {
            if *((begin as *const u32).offset(i as isize)) == val {
                return i;
            }
            i += 1;
        }
        -1
    }
});
