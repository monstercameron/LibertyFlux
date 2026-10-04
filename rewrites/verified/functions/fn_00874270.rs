// original: 0x00874270 anim_node_init
/// Initialises a motion-node object in place: writes the class tag, a default
/// parameter block, unit weight (1.0) and a cleared animation slot. Returns
/// the object pointer.
export!(thiscall, rw_00874270(node: u32) -> u32 {
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
        node
    }
});
