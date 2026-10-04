// original: 0x00cf47f0 climb_task_find_marked (proposed)

/// Walks the climb-task list from the given head, polling each node's virtual
/// slot 0xc with the node: the first node answering 0xfe is forwarded to the
/// marked-task handler (a tail call whose answer is returned), a null list or
/// a list with no such node returns 0.
///
/// Original: 0x00cf47f0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00cf47f0(head: u32) -> u32 {
    unsafe {
        const POLL_SLOT: u32 = 0xc;
        const MARKED: u32 = 0xfe;
        const NEXT_OFFSET: u32 = 8;
        const HANDLER_CALLEE: u32 = 2;
        type Poll = extern "thiscall" fn(u32) -> u32;
        let mut node = head;
        if node == 0 {
            return 0;
        }
        loop {
            let vt = (node as *const u32).read_unaligned();
            let poll: Poll = core::mem::transmute(
                ((vt + POLL_SLOT) as *const u32).read_unaligned() as usize);
            if poll(node) == MARKED {
                return lf_checker_rt::callee_thiscall!(HANDLER_CALLEE, u32, node);
            }
            node = ((node + NEXT_OFFSET) as *const u32).read_unaligned();
            if node == 0 {
                return 0;
            }
        }
    }
});
