// original: 0x00DB5680 cursor_tree_lookup

/// Look up the node holding a key in the tree.
///
/// `this` is the tree and `key` the search key. Like `cursor_tree_contains`
/// this asks the lookup helper (scripted on both sides) through two
/// pointers into its own frame, but it returns the reported node pointer
/// rather than a boolean: null when the helper reports no node or when the
/// search key is below the reported candidate key, compared UNSIGNED
/// (`jb` in the original).
///
/// Original: 0x00DB5680 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00db5680(this: u32, key: u32) -> u32 {
    unsafe {
        const FIND: u32 = 1;
        let mut out = [0u32; 3];
        let slot = key;
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
        if key < out[0] {
            return 0;
        }
        node
    }
});
