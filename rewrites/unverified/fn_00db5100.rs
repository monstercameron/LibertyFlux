// original: 0x00DB5100 cursor_tree_contains

/// Test whether a key is present in the tree.
///
/// `this` is the tree and `key` the search key. The lookup helper
/// (scripted on both sides) is asked through two pointers into this
/// function's own frame: an output area for its three-word answer and the
/// key slot. The key counts as present when the helper reports a node and
/// the search key is not below the reported candidate key, compared
/// UNSIGNED (`jb` in the original); since the helper returns the first
/// node at or above the key, that means equality.
///
/// The contract skips the two frame-pointer call arguments, snapshots the
/// key word, and scripts the helper's three written words; the low byte of
/// the return is compared (`al`: the original sets only `al`).
///
/// Original: 0x00DB5100 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00db5100(this: u32, key: u32) -> u32 {
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
        1
    }
});
