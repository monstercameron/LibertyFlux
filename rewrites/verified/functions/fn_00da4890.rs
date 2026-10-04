// original: 0x00da4890 global_list_remove_value
// s10f20: remove a value from the global dword list (stdcall/1).
//
// Finds the value between the global begin/end pointers, closes the gap with
// the shared move step and pulls the end pointer back one slot. A miss or an
// empty list does nothing. Returns nothing meaningful.
export!(stdcall, rw_s10f20(val: u32) -> u32 {
    unsafe {
        let g1 = global::<u32>(0x17A64F0);
        let g2 = global::<u32>(0x17A64F4);
        let mut cur = *g1;
        let end = *g2;
        if cur == end {
            return 0;
        }
        loop {
            if *(cur as *const u32) == val {
                break;
            }
            cur = cur.wrapping_add(4);
            if cur == end {
                return 0;
            }
        }
        // cur points at the hit.
        let src = cur.wrapping_add(4);
        if src != end {
            let n = end.wrapping_sub(src);
            if n != 0 {
                let _: u32 = callee_cdecl!(1, u32, cur, src, n);
            }
        }
        *g2 = end.wrapping_sub(4);
        0
    }
});
