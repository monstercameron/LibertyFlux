// original: 0x00DB56F0 cursor_tree_lookup_interned

/// Look up the node holding an interned key in the tree.
///
/// `this` is the tree and `key` the raw search key. The key is first
/// passed through the interning helper (scripted on both sides); the
/// original also dead-stores the interned key over its own incoming
/// argument slot, which the contract's stack check skips. The interned key
/// is then looked up exactly as in `cursor_tree_lookup`: null when the
/// helper reports no node or when the interned key is below the reported
/// candidate key, compared UNSIGNED (`jb` in the original).
///
/// Original: 0x00DB56F0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00db56f0(this: u32, key: u32) -> u32 {
    unsafe {
        const INTERN: u32 = 1;
        const FIND: u32 = 2;
        let interned: u32 = lf_checker_rt::callee_cdecl!(INTERN, u32, key);
        let mut out = [0u32; 3];
        let slot = interned;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            FIND,
            u32,
            this,
            &mut out as *mut u32 as u32,
            &slot as *const u32 as u32,
            0
        );
        let node = out[1];
        if node == 0 {
            return 0;
        }
        if interned < out[0] {
            return 0;
        }
        node
    }
});
