// original: 0x00874b30 filter_node_init_ptr
/// Initialises a filter-node object through a pointer (class tag, default
/// mode word, cleared parameter block) when given a non-null pointer,
/// otherwise does nothing. Returns the argument.
export!(cdecl, rw_00874b30(node: u32) -> u32 {
    if node != 0 {
        unsafe {
            let base = node as *mut u32;
            base.add(1).write(0x00070000);
            for i in 2..8usize {
                base.add(i).write(0);
            }
            base.write(relocated(0x00FE80D0));
            base.add(8).write(0);
        }
    }
    node
});
