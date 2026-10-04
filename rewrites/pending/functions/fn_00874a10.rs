// original: 0x00874a10 filter_node_init
/// Initialises a filter-node object in place: writes the class tag, a default
/// mode word and a cleared parameter block. Returns the object pointer.
export!(thiscall, rw_00874a10(node: u32) -> u32 {
    unsafe {
        let base = node as *mut u32;
        base.add(1).write(0x00070000);
        for i in 2..8usize {
            base.add(i).write(0);
        }
        base.write(relocated(0x00FE80D0));
        base.add(8).write(0);
        node
    }
});
