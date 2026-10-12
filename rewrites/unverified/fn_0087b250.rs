// original: 0x0087B250 rage::crmtNodeAddSubtract::initialize_if_nonnull

/// Initialize a non-null add/subtract node.
///
/// The cdecl argument is the node pointer. A null pointer returns without a
/// write or call. Otherwise the base initializer runs with type 6, then the
/// add/subtract vtable at file VA `0x00FE837C` is stored at `node+0`. The base
/// initializer's EAX value is left as the return value. The null path leaves
/// EAX unspecified by the original.
///
/// Original: cdecl with one stack pointer; the base initializer is intercepted.
lf_checker_rt::export!(cdecl, rw_0087b250(node: u32) -> u32 {
    const NODE_VTABLE: u32 = 0x00FE837C;
    const NODE_TYPE: u32 = 6;
    const BASE_INITIALIZER: u32 = 1;

    if node == 0 {
        return 0;
    }

    unsafe {
        let result = lf_checker_rt::callee_thiscall!(BASE_INITIALIZER, u32, node, NODE_TYPE);
        (node as *mut u32).write_unaligned(lf_checker_rt::relocated(NODE_VTABLE));
        result
    }
});
