// original: 0x0094fb00 list_append
/// Append a node to a doubly-linked list.
///
/// An empty list (null tail) becomes just `node` in both the head and tail
/// slots. Otherwise the node is spliced after the tail through the list
/// worker and becomes the new tail; the worker's answer is returned.
export!(thiscall, rw_0094fb00(list: *mut u32, node: u32) -> u32 {
    unsafe {
        let tail = *list.add(1);
        if tail == 0 {
            *list = node;
            *list.add(1) = node;
            node
        } else {
            let answer = callee_thiscall!(1, u32, tail, node);
            *list.add(1) = node;
            answer
        }
    }
});
