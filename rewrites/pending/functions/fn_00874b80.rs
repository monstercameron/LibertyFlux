// original: 0x00874b80 filter_node_tag_if_present
/// Tags a filter-node object with its class tag when given a non-null
/// pointer, otherwise does nothing. Returns the argument.
export!(cdecl, rw_00874b80(node: u32) -> u32 {
    if node != 0 {
        unsafe {
            (node as *mut u32).write(relocated(0x00FE80D0));
        }
    }
    node
});
