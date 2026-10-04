// original: 0x00d425d0 task_node_create_44 (proposed)

/// Allocate a 44-byte task node, construct it, tag it, append it to its table.
///
/// Calls the allocator for 44 bytes; a null result skips construction and the
/// later tag store faults exactly as the original does. Otherwise constructs
/// the node in place, stores the incoming `tag` in its first dword, appends
/// the node through the table helper (fixed object, fixed size word 16), and
/// returns the node pointer.
///
/// Original: cdecl, one stack word (tag), caller cleans up. Allocator is
/// cdecl/1, constructor thiscall/0 returning the node, table helper
/// thiscall/1 returning the slot. All three are intercepted.
lf_checker_rt::export!(cdecl, rw_00d425d0(tag: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        const INSERT: u32 = 3;
        const NODE_SIZE: u32 = 0x2c;
        const TABLE_OBJ: u32 = 0x01720924;
        const INSERT_ARG: u32 = 0x10;
        let mem: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, NODE_SIZE);
        let node: u32 = if mem == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(CTOR, u32, mem)
        };
        (node as *mut u32).write_unaligned(tag);
        let slot: u32 =
            lf_checker_rt::callee_thiscall!(INSERT, u32, lf_checker_rt::relocated(TABLE_OBJ), INSERT_ARG);
        (slot as *mut u32).write_unaligned(node);
        node
    }
});
