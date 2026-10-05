// original: 0x00b57090 drain_list_then_base_cleanup
/// List teardown: release every node, close the list, hand off.
///
/// Takes the object pointer in ECX. Walks the owned node chain, releasing
/// each node's payload through the node release entry, then closes the
/// list head through the list close entry and transfers control to the
/// shared successor with the same object pointer, returning whatever that
/// call answers. Only this function's own bytes are rewritten: the listed
/// size runs past its end into the following bytes.
export!(thiscall, rw_00b57090(this: u32) -> u32 {
    unsafe {
        let mut node = *(this.wrapping_add(0x1A28) as *const u32);
        while node != 0 {
            let next = *(node.wrapping_add(0x8C) as *const u32);
            callee_thiscall!(2, u32, node.wrapping_add(4));
            node = next;
        }
        callee_thiscall!(3, u32, this.wrapping_add(0x1A28));
        callee_thiscall!(1, u32, this)
    }
});
