// original: 0x00635340 bst_find (proposed)

/// Find the node whose key equals `*keypp` in the string-keyed binary search
/// tree rooted at `[this]`.
///
/// Same signed-key walk as `rw_006352c0` but without the parent out-pointer:
/// descend left while the wanted key is below the node's key, right while
/// above, and return the node on an exact match or null when the walk
/// leaves the tree or the tree is empty. The second stack word is accepted
/// and ignored (the original cleans two words).
///
/// Returns the found node or null.
///
/// Original: 0x00635340 (thiscall, two stack words, no calls, no globals).
lf_checker_rt::export!(thiscall, rw_00635340(this: u32, keypp: u32, _w1: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x30;
        const LEFT: u32 = 0x38;
        const RIGHT: u32 = 0x3c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
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
        while node != 0 {
            match cmp_key(want, rd32(node + KEY)) {
                core::cmp::Ordering::Less => {
                    node = rd32(node + LEFT);
                }
                core::cmp::Ordering::Greater => {
                    node = rd32(node + RIGHT);
                }
                core::cmp::Ordering::Equal => break,
            }
        }
        node
    }
});
