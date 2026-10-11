// original: 0x009BB410 new_input_node_d (proposed)

/// Allocate and initialise a 12-byte input node using this handler's vtable.
///
/// The caller's word is stored at `+0x08`, a zero flag byte is stored at
/// `+0x04`, and the vtable pointer is installed at `+0x00`. Allocation uses
/// the original 12-byte request. A null allocation returns zero without
/// writes; a successful allocation returns the node pointer.
lf_checker_rt::export!(stdcall, rw_009bb410(kept: u32) -> u32 {
    unsafe {
        const NODE_BYTES: u32 = 12;
        const VTABLE: u32 = 0x00e9_41a8;
        const ALLOCATE: u32 = 1;
        let node = lf_checker_rt::callee_cdecl!(ALLOCATE, u32, NODE_BYTES);
        if node == 0 {
            return 0;
        }
        (node as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (node.wrapping_add(4) as *mut u8).write(0);
        (node.wrapping_add(8) as *mut u32).write_unaligned(kept);
        node
    }
});
