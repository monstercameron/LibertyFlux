// original: 0x00da4850 remove_all_matching_key
// s10f19: drop every row whose tag matches, back to front (thiscall/1).
//
// Walks the pointer list from the end so removals keep later indexes valid.
// Each row whose dword at +0x10 equals the key is torn down and then removed
// through the list's own remove step. Always returns false (0).
export!(thiscall, rw_s10f19(this: *const u8, key: u32) -> u8 {
    unsafe {
        let begin = *(this as *const u32);
        let end = *(this.add(4) as *const u32);
        let mut i = ((end.wrapping_sub(begin) as i32) >> 2) - 1;
        let teardown: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let remove: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        while i >= 0 {
            let p = *((begin as *const u32).offset(i as isize));
            if *((p as *const u32).add(4)) == key {
                teardown(p);
                remove(this as u32, p);
            }
            i -= 1;
        }
        0
    }
});
