// original: 0x009bb470 new_input_node_b (proposed)

/// Allocate and initialise a 12-byte input callback node.
///
/// Calls the game's 12-byte allocator (one intercepted callee, cdecl, single
/// size argument). On success writes the node's vtable pointer at `+0x00`,
/// a zero flag byte at `+0x04` and the caller's stored word `kept` at
/// `+0x08`, and returns the node address. When the allocator answers null
/// (out of memory) writes nothing and returns 0.
///
/// Edge cases: a null allocator answer takes the failure path with no
/// stores; any non-null answer is treated as 12 writable bytes.
///
/// Original: stdcall, one stack word (`kept`).
lf_checker_rt::export!(stdcall, rw_009bb470(kept: u32) -> u32 {
    unsafe {
        const NODE_BYTES: u32 = 12;
        const VTABLE: u32 = 0x00e94118;
        const FLAG_OFF: u32 = 0x04;
        const KEPT_OFF: u32 = 0x08;
        const ALLOC: u32 = 1;
        let node: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, NODE_BYTES);
        if node == 0 {
            return 0;
        }
        ((node + FLAG_OFF) as *mut u8).write(0);
        (node as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((node + KEPT_OFF) as *mut u32).write_unaligned(kept);
        node
    }
});
