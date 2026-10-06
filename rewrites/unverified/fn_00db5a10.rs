// original: 0x00DB5A10 cursor_tree_count_ref

/// Count matching tree entries for one key, passed by reference.
///
/// `this` is the tree and `key` the search key. The function forwards the
/// address of its own incoming argument slot to the counting helper
/// (scripted on both sides) and returns the helper's answer. The pointer
/// itself differs between the two sides, so the contract skips that call
/// argument and compares the one pointed-to word instead.
///
/// Original: 0x00DB5A10 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00db5a10(this: u32, key: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 1;
        let slot = key;
        lf_checker_rt::callee_thiscall!(COUNT, u32, this, &slot as *const u32 as u32)
    }
});
