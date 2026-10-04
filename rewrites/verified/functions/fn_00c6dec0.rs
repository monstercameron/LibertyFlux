// original: 0x00c6dec0 unguarded_insert_element
/// Insert (key, value) ending at `pos`, shifting elements with larger signed
/// keys one slot up. The caller guarantees a smaller-or-equal key exists
/// below, so the scan always terminates. Returns `val`.
export!(cdecl, rw_00c6dec0(pos: u32, key: u32, val: u32) -> u32 {
    unsafe {
        let mut d = pos as *mut u32;
        if (key as i32) < (*(pos.wrapping_sub(8) as *const u32) as i32) {
            let mut s = pos.wrapping_sub(8) as *mut u32;
            loop {
                *d = *s;
                *d.add(1) = *s.add(1);
                d = s;
                s = (s as u32).wrapping_sub(8) as *mut u32;
                if (key as i32) >= (*s as i32) {
                    break;
                }
            }
        }
        *d = key;
        *d.add(1) = val;
        val
    }
});
