// original: 0x00b55c80 find_node_by_key_with_float_check
/// Find the first live node whose +0x10 field equals the key, with re-check.
///
/// Walks the +0x1a28 list (link at +0x8c) for a live node (u16 at +0x48 is 1,
/// +0x44 field non-zero) matching the key. When the flag byte is clear the
/// first key match wins; otherwise a helper is consulted on the candidate and
/// the match only stands when its float answer is >= 0 or NaN (the original
/// compares 0.0 against it with comiss and continues on below). Returns node
/// address + 4 or null.
export!(thiscall, rw_00b55c80(this: u32, key: u32, flag: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x1a28;
        let mut node = ((this + HEAD) as *const u32).read();
        if node == 0 {
            return 0;
        }
        let recheck = (flag & 0xff) != 0;
        loop {
            let typed = ((node + 0x48) as *const u16).read() == 1;
            let base = node.wrapping_add(4);
            node = ((node + 0x8c) as *const u32).read();
            if typed
                && ((base + 0x40) as *const u32).read() != 0
                && ((base + 0x0c) as *const u32).read() == key
            {
                if !recheck {
                    return base;
                }
                let v: f32 = callee_thiscall!(1, f32, base);
                if !(v < 0.0) {
                    return base;
                }
            }
            if node == 0 {
                return 0;
            }
        }
    }
});
