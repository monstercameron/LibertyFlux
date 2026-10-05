// original: 0x00AC5BA0 stream_node_release_chain (proposed)

/// Release a chain of streaming nodes through the free callee.
///
/// The original walks `node = node->next` (`+8`), calling the child-release
/// callee with each node's payload word (`+0xc`) and then the free callee
/// with the node itself, until the null terminator (thiscall, one stack
/// pointer; a null head releases nothing). The child-release call is the
/// function's own recursive call site, answered by the checker. No value
/// is returned.
lf_checker_rt::export!(thiscall, rw_00AC5BA0(this: u32, node: u32) -> u32 {
    unsafe {
        const RELEASE_CHILD: u32 = 1;
        const FREE: u32 = 2;
        const NEXT: u32 = 8;
        const PAYLOAD: u32 = 0x0c;
        let mut cur = node;
        while cur != 0 {
            let payload = (cur.wrapping_add(PAYLOAD) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(RELEASE_CHILD, u32, this, payload);
            let next = (cur.wrapping_add(NEXT) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(FREE, u32, cur);
            cur = next;
        }
        0
    }
});
