// original: 0x006352c0 bst_find_parent (proposed)

/// Find the node whose key equals `*keypp` in the string-keyed binary search
/// tree rooted at `[this]`, reporting the last node visited.
///
/// Keys are NUL-terminated byte strings compared as SIGNED bytes (see
/// `rw_00635000`); node layout is the same (`+KEY` key pointer, `+PARENT`
/// parent, `+LEFT`/`+RIGHT` children). The walk descends left while the
/// wanted key is below the node's key and right while above, and returns
/// the node on an exact match or null when the walk leaves the tree. When
/// `out` is non-null the last non-null node visited (the match's parent
/// when the key is absent, unchanged from the previous visit when present)
/// is stored there; a null `out` skips the store. An empty tree returns
/// null and stores null.
///
/// Returns the found node or null.
///
/// Original: 0x006352c0 (thiscall, two stack words, no calls, no globals).
lf_checker_rt::export!(thiscall, rw_006352c0(this: u32, keypp: u32, out: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x30;
        const LEFT: u32 = 0x38;
        const RIGHT: u32 = 0x3c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// Signed NUL-terminated byte-string order. The original reaches it
        /// with two one-sided scans; the single loop reads the same bytes.
        unsafe fn cmp_key(want: u32, have: u32) -> core::cmp::Ordering {
            unsafe {
                let mut x = want;
                let mut y = have;
                loop {
                    let a = (x as *const u8).read() as i8;
                    let b = (y as *const u8).read() as i8;
                    if a != b {
                        return a.cmp(&b);
                    }
                    if a == 0 {
                        return core::cmp::Ordering::Equal;
                    }
                    x = x.wrapping_add(1);
                    y = y.wrapping_add(1);
                }
            }
        }

        let want = rd32(keypp);
        let mut node = rd32(this);
        let mut parent = 0u32;
        while node != 0 {
            match cmp_key(want, rd32(node + KEY)) {
                core::cmp::Ordering::Less => {
                    parent = node;
                    node = rd32(node + LEFT);
                }
                core::cmp::Ordering::Greater => {
                    parent = node;
                    node = rd32(node + RIGHT);
                }
                core::cmp::Ordering::Equal => break,
            }
        }
        if out != 0 {
            wr32(out, parent);
        }
        node
    }
});
