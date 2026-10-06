// original: 0x008728E0 child_broadcast
/// Broadcast a message to every node in the child list headed at `this`.
///
/// The head pointer lives at `this + 0x14` and each node links to the next
/// through the word at `+0x0C`. Every node is visited in order; each one is
/// notified through virtual slot `0x0C` of its own table, called as
/// thiscall with the node as `this` and the message as the single stack
/// argument. The callee answers are ignored. An empty list makes no calls.
/// The function has no meaningful return value (the original leaves
/// whatever the last call, or the entry state, left in `eax`), so the
/// contract compares no return channel.
///
/// Original: 0x008728E0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_008728E0(this: u32, msg: u32) -> u32 {
    unsafe {
        const CHILD_HEAD: u32 = 0x14;
        const NEXT_OFF: u32 = 0x0C;
        const NOTIFY_SLOT: u32 = 0x0C;
        let mut node = ((this + CHILD_HEAD) as *const u32).read_unaligned();
        while node != 0 {
            let vtable = (node as *const u32).read_unaligned();
            let next = ((node + NEXT_OFF) as *const u32).read_unaligned();
            let target = ((vtable + NOTIFY_SLOT) as *const u32).read_unaligned();
            let notify: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            notify(node, msg);
            node = next;
        }
    }
    0
});
