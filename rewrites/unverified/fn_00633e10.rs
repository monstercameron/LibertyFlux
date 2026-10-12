// original: 0x00633E10 rage::fragDrawable::~fragDrawable__deleting

/// Search the tree embedded at `this + 0xD4` using the key pointer supplied
/// by the caller. The native helper receives a pointer to the incoming key
/// slot and the original object pointer as its two stack arguments, returns
/// its search result in EAX, and removes both arguments. This rewrite keeps a
/// local key cell for the call; the contract compares that cell's contents
/// with a call-time snapshot because its address is necessarily different.
lf_checker_rt::export!(thiscall, rw_00633E10(this: u32, key_pointer: u32) -> u32 {
    const BST_FIND_CALLEE: u32 = 1;

    let mut key_pointer_cell = key_pointer;
    let key_cell_address = core::ptr::addr_of_mut!(key_pointer_cell) as u32;
    let tree = this.wrapping_add(0xD4);
    lf_checker_rt::callee_thiscall!(BST_FIND_CALLEE, u32, tree, key_cell_address, this)
});
