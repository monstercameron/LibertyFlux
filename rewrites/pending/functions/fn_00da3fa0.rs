// original: 0x00da3fa0 find_obj_by_key_pair
// s10f05: find a row object by two key fields (thiscall/2).
//
// Scans the object's pointer list for the first entry whose dwords at +0x10
// and +0x14 equal the two keys. Returns the entry or null. The count is a
// signed shift, so a malformed range yields no search.
export!(thiscall, rw_s10f05(this: *const u8, k1: u32, k2: u32) -> u32 {
    unsafe {
        let begin = *(this as *const u32);
        let end = *(this.add(4) as *const u32);
        let n = (end.wrapping_sub(begin) as i32) >> 2;
        let mut i = 0i32;
        while i < n {
            let p = *((begin as *const u32).offset(i as isize));
            if *((p as *const u32).add(4)) == k1 && *((p as *const u32).add(5)) == k2 {
                return p;
            }
            i += 1;
        }
        0
    }
});
