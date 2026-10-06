// original: 0x00DB50C0 cursor_tree_insert_checked

/// Insert a node into the tree after a key check.
///
/// `this` is the tree and `node` the node to insert. A key hook at slot
/// `+0x48` of the node's virtual table is called with the node, its answer
/// passed through the interning helper (both scripted), and the tree
/// insertion helper (scripted) is called with the tree, an output area and
/// the interned key in this function's own frame, and the node. The
/// original also dead-stores the interned key over its own incoming
/// argument slot, which the contract's stack check skips. The insertion
/// helper's outputs are never read back; the function returns the
/// helper's answer.
///
/// Original: 0x00DB50C0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00db50c0(this: u32, node: u32) -> u32 {
    unsafe {
        const KEY_SLOT: u32 = 0x48;
        const INTERN: u32 = 2;
        const INSERT: u32 = 3;

        let vtable = (node as *const u32).read_unaligned();
        let key_hook: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable + KEY_SLOT) as *const u32).read_unaligned() as usize,
        );
        let key = key_hook(node);
        let interned: u32 = lf_checker_rt::callee_cdecl!(INTERN, u32, key);
        let mut out = [0u32; 4];
        let slot = interned;
        lf_checker_rt::callee_thiscall!(
            INSERT,
            u32,
            this,
            &mut out as *mut u32 as u32,
            &slot as *const u32 as u32,
            node
        )
    }
});
