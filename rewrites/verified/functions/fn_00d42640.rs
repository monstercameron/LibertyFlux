// original: 0x00d42640 task_node_create_a8_init (proposed)

/// Allocate a 168-byte task node, construct it, initialise it, table it.
///
/// Calls the allocator for 168 bytes; a null result skips construction (the
/// initialiser still runs on the null pointer through the intercepted stub,
/// as in the original). Otherwise constructs the node in place, runs the
/// initialiser on it with the incoming `arg`, appends it through the table
/// helper (fixed object, fixed size word 16), and returns the node pointer.
///
/// Original: cdecl, one stack word (arg), caller cleans up. Allocator is
/// cdecl/1, constructor thiscall/0 returning the node, initialiser
/// thiscall/1, table helper thiscall/1 returning the slot. All intercepted.
lf_checker_rt::export!(cdecl, rw_00d42640(arg: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        const INIT: u32 = 3;
        const INSERT: u32 = 4;
        const NODE_SIZE: u32 = 0xa8;
        const TABLE_OBJ: u32 = 0x017208f0;
        const INSERT_ARG: u32 = 0x10;
        let mem: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, NODE_SIZE);
        let node: u32 = if mem == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(CTOR, u32, mem)
        };
        let _: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, node, arg);
        let slot: u32 =
            lf_checker_rt::callee_thiscall!(INSERT, u32, lf_checker_rt::relocated(TABLE_OBJ), INSERT_ARG);
        (slot as *mut u32).write_unaligned(node);
        node
    }
});
