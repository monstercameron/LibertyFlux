// original: 0x00AAC030 list_pop_and_reset_node

/// Remove the list's tail node and clear its reset field when a node is returned.
///
/// The thiscall receiver is adjusted by `LIST_HEAD_OFFSET` and passed as ECX
/// to a no-argument tail-removal function. A null result is left alone. A
/// returned node has its 32-bit reset field at byte offset 112 cleared to zero.
lf_checker_rt::export!(thiscall, rw_00aac030(this: u32) -> () {
    const LIST_HEAD_OFFSET: u32 = 0x31020;
    const REMOVAL_ID: u32 = 1;
    const NODE_RESET_FIELD: usize = 0x70;

    let list_head = this.wrapping_add(LIST_HEAD_OFFSET);
    let node = lf_checker_rt::callee_thiscall!(REMOVAL_ID, u32, list_head);
    if node != 0 {
        unsafe {
            (node as *mut u8)
                .add(NODE_RESET_FIELD)
                .cast::<u32>()
                .write_unaligned(0);
        }
    }
});
