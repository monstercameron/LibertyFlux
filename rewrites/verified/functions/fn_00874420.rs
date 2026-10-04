// original: 0x00874420 anim_node_init_fields
/// Initialises the trailing field block of a motion-node object (class tag,
/// cleared slots, unit weight) when given a non-null pointer, otherwise does
/// nothing. Returns the argument.
export!(cdecl, rw_00874420(node: u32) -> u32 {
    if node != 0 {
        unsafe {
            let base = node as *mut u32;
            base.write(relocated(0x00FE8094));
            for i in 7..12usize {
                base.add(i).write(0);
            }
            base.add(12).write(0x3F800000);
            base.add(13).write(0);
        }
    }
    node
});
