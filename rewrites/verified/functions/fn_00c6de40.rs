// original: 0x00c6de40 push_heap_hole
/// Insert a (key, value) pair into the element heap at `base`, starting from
/// slot `hole` and percolating up toward slot `top`. Elements compare by
/// signed 32-bit key. Returns `val` (left in EAX by the original).
export!(cdecl, rw_00c6de40(base: u32, hole: u32, top: u32, key: u32, val: u32, _extra: u32) -> u32 {
    unsafe {
        let w = base as *mut u32;
        let mut h = hole as i32;
        let t = top as i32;
        let k = key as i32;
        let mut p = (h - 1) / 2;
        while h > t {
            if (*w.add((p as usize) * 2) as i32) >= k {
                break;
            }
            *w.add((h as usize) * 2) = *w.add((p as usize) * 2);
            *w.add((h as usize) * 2 + 1) = *w.add((p as usize) * 2 + 1);
            h = p;
            p = (h - 1) / 2;
        }
        *w.add((h as usize) * 2) = key;
        *w.add((h as usize) * 2 + 1) = val;
        val
    }
});
