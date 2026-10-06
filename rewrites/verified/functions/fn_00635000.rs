// original: 0x00635000 bst_link_child (proposed)

/// Link `node` as a child of `slot` in a binary search tree keyed by a
/// NUL-terminated byte string, and count the insertion.
///
/// `this` points to the tree header (insertion count at `+COUNT`, read and
/// incremented). Each node holds its key pointer at `+KEY`, its parent link
/// at `+PARENT` and its children at `+LEFT`/`+RIGHT`. The keys are compared
/// byte by byte as SIGNED bytes (`jge` in the original: a byte of 0x80 or
/// above sorts before any byte below 0x80); the scan stops at the first
/// differing byte or NUL. When the node's key is below the slot's key the
/// node becomes the slot's left child, otherwise (equal included) the right
/// child; the node's parent is set to the slot either way. The slot's other
/// child and the node's own children are untouched.
///
/// Returns the leftover the original leaves in eax: the advanced key
/// pointer with its low byte replaced by the key byte at that position
/// (the original reloads `al` from `[eax]` for the final compare, so the
/// low 8 bits of the return are the first differing-or-NUL byte, not the
/// pointer's own low byte).
///
/// Original: 0x00635000 (thiscall, two stack words, no calls, no globals).
lf_checker_rt::export!(thiscall, rw_00635000(this: u32, node: u32, slot: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x30;
        const PARENT: u32 = 0x34;
        const LEFT: u32 = 0x38;
        const RIGHT: u32 = 0x3c;
        const COUNT: u32 = 0x04;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let mut ka = rd32(node + KEY);
        let mut kb = rd32(slot + KEY);
        while rd8(ka) == rd8(kb) && rd8(ka) != 0 {
            ka = ka.wrapping_add(1);
            kb = kb.wrapping_add(1);
        }
        if (rd8(ka) as i8) < (rd8(kb) as i8) {
            wr32(slot + LEFT, node);
        } else {
            wr32(slot + RIGHT, node);
        }
        wr32(this + COUNT, rd32(this + COUNT).wrapping_add(1));
        wr32(node + PARENT, slot);
        // The original's final '(an instruction of the original)
        // the advanced pointer with the key byte it just compared.
        // black_box: without it LLVM folds (ka & mask) away and returns
        // the byte alone (observed in the built DLL).
        (core::hint::black_box(ka) & 0xFFFF_FF00) | (rd8(ka) as u32)
    }
});
