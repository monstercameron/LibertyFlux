// original: 0x00b55c70 find_node_by_key_unchecked
/// Find a node by key without the float re-check (flag forced to 0).
///
/// Thin wrapper over fn_00b55c80: forwards the key and passes 0 for the flag
/// byte, so the first key match is returned directly.
export!(thiscall, rw_00b55c70(this: u32, key: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this, key, 0)
    }
});
