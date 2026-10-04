// original: 0x00874410 anim_node_init_if_present
/// Initialises a motion-node object only when given a non-null pointer,
/// otherwise does nothing. Returns the argument; the original leaves an
/// unspecified value in eax on the null path.
export!(cdecl, rw_00874410(node: u32) -> u32 {
    if node != 0 {
        unsafe {
            let base = node as *mut u32;
            base.add(1).write(0x00040000);
            for i in 2..12usize {
                base.add(i).write(0);
            }
            base.write(relocated(0x00FE8094));
            base.add(12).write(0x3F800000);
            base.add(13).write(0);
            ((node + 0x38) as *mut u8).write(0);
        }
    }
    node
});
