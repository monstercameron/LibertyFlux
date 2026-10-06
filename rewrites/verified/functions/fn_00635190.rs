// original: 0x00635190 bst_insert (proposed)

/// Insert `node` into the string-keyed binary search tree headed by `this`,
/// rejecting duplicates.
///
/// The tree layout is the one `rw_00635000` links: header holds the root
/// at `+ROOT` and the count at `+COUNT`, nodes their key at `+KEY`. The
/// find callee searches for the node's key and reports the would-be parent
/// through its out-pointer; the link callee attaches the node under that
/// parent and counts it. When the key is already present the tree is
/// untouched. When the tree is empty the node becomes the root and the
/// count is set to 1.
///
/// Returns 1 when the node was inserted, 0 for a duplicate, with the
/// original's partial-register leftovers: the duplicate path clears only
/// `al`, so the high 24 bits of the find result survive; the link path sets
/// only `al`, so the high 24 bits of the link result survive; the empty
/// path returns exactly 1.
///
/// Original: 0x00635190 (thiscall, one stack word, two calls).
lf_checker_rt::export!(thiscall, rw_00635190(this: u32, node: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x30;
        const ROOT: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const FIND: u32 = 1;
        const LINK: u32 = 2;
        const LOW_BYTE: u32 = 0xFFFF_FF00;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let keypp = node.wrapping_add(KEY);
        let mut parent = 0u32;
        let out = &mut parent as *mut u32 as u32;
        let found: u32 = lf_checker_rt::callee_thiscall!(FIND, u32, this, keypp, out);
        if found != 0 {
            return core::hint::black_box(found) & LOW_BYTE;
        }
        if parent != 0 {
            let lr: u32 = lf_checker_rt::callee_thiscall!(LINK, u32, this, node, parent);
            return (core::hint::black_box(lr) & LOW_BYTE) | 1;
        }
        wr32(this.wrapping_add(ROOT), node);
        wr32(this.wrapping_add(COUNT), 1);
        1
    }
});
