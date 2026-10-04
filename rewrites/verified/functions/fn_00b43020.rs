// original: 0x00b43020 accumulate_bounds_over_chain
/// Recompute an object's bound words over its node chain.
///
/// Seeds the six slots with the widest inverted range, then folds every
/// node of the chain in: each min slot keeps the smaller value, each max
/// slot the larger (signed compares). Always returns 0.
export!(thiscall, rw_b43020(obj: u32) -> u32 {
    unsafe {
        (obj as *mut u32).byte_add(0x14).write(0x83007d00);
        (obj as *mut u32).byte_add(0x10).write(0x83007d00);
        (obj as *mut u32).byte_add(0x0c).write(0x83007d00);
        let mut node = (obj as *const u32).byte_add(0x18).read();
        if node == 0 {
            return 0;
        }
        let mut lo0: i16 = 0x7d00;
        let mut lo1: i16 = 0x7d00;
        let mut lo2: i16 = 0x7d00;
        let mut hi0: i16 = 0x8300u16 as i16;
        let mut hi1: i16 = 0x8300u16 as i16;
        let mut hi2: i16 = 0x8300u16 as i16;
        loop {
            let v = (node as *const i16).byte_add(0x260).read();
            if v < lo0 {
                lo0 = v;
                (obj as *mut i16).byte_add(0x0c).write(v);
            }
            let v = (node as *const i16).byte_add(0x264).read();
            if v < lo1 {
                lo1 = v;
                (obj as *mut i16).byte_add(0x10).write(v);
            }
            let v = (node as *const i16).byte_add(0x268).read();
            if v < lo2 {
                lo2 = v;
                (obj as *mut i16).byte_add(0x14).write(v);
            }
            let v = (node as *const i16).byte_add(0x262).read();
            if v > hi0 {
                hi0 = v;
                (obj as *mut i16).byte_add(0x0e).write(v);
            }
            let v = (node as *const i16).byte_add(0x266).read();
            if v > hi1 {
                hi1 = v;
                (obj as *mut i16).byte_add(0x12).write(v);
            }
            let v = (node as *const i16).byte_add(0x26a).read();
            if v > hi2 {
                hi2 = v;
                (obj as *mut i16).byte_add(0x16).write(v);
            }
            node = (node as *const u32).byte_add(0x258).read();
            if node == 0 {
                break;
            }
        }
        0
    }
});
