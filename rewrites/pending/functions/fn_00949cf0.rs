// original: 0x00949cf0 emitter_trigger_active
/// Fire all live emitters: for each of 16 nodes with stride 0x30 whose
/// enable byte is set, call the engine worker with the node's four float
/// bit-patterns, a unit flag and four constant words.
export!(cdecl, rw_00949cf0() -> u32 {
    unsafe {
        for i in 0..16u32 {
            let node = global::<u8>(0x011EE2C4).add((i * 0x30) as usize);
            if *node.sub(0x14) == 0 {
                continue;
            }
            let g0 = *(node.sub(4) as *const u32);
            let g4 = *(node as *const u32);
            let g8 = *(node.add(4) as *const u32);
            let gc = *(node.add(0xC) as *const u32);
            callee_cdecl!(
                1, u32,
                g0, g4, g8, gc, 1,
                0x3F80_0000, 0x3F00_8081, 0, 0x3F32_B2B3, 0,
            );
        }
        0
    }
});
