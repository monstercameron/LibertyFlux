// original: 0x005B6030 txd_slot_alloc
/// Allocates a texture-dictionary pool node, resolves the streaming id for
/// the name, links them, and returns the node's index in the pool array.
export!(cdecl, rw_005B6030(name: u32) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x011764C0);
        let node = callee_thiscall!(1, u32, pool);
        let id = callee_cdecl!(2, u32, name);
        let np = node as *mut u32;
        np.add(0).write(0);
        np.add(1).write(0);
        np.add(2).write(id);
        np.add(3).write(0xFFFF_FFFF);
        let poolp = (*global::<u32>(0x011764C0)) as *const u32;
        let base = poolp.add(0).read();
        let stride = poolp.add(3).read() as i32;
        ((node.wrapping_sub(base)) as i32).wrapping_div(stride) as u32
    }
});
