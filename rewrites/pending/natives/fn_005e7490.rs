// original: 0x005e7490 HAS_POOL_OBJECT_COLLIDED_WITH_CUSHION
/// Script native `HAS_POOL_OBJECT_COLLIDED_WITH_CUSHION` (hash 0x3E8D7D3F).
///
/// Resolves a pool-object handle through the pool header (reached via a
/// global pointer): the handle's high bits index an id-byte array which must
/// match the handle's low byte. On a match the engine is called with the
/// computed record address (`base + stride * index`); otherwise it is called
/// with null. Stores the low byte of the engine answer (zero-extended) into
/// the return slot.
export!(cdecl, rw_005e7490(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let handle = *args;
        let pool = *global::<u32>(0x01632c60) as *const u32;
        let id_bytes = *pool.add(1) as *const u8;
        let index = ((handle as i32 >> 8) as u32) as usize;
        let record = if *id_bytes.add(index) == handle as u8 {
            let stride = *pool.add(3);
            let base = *pool;
            stride.wrapping_mul(index as u32).wrapping_add(base)
        } else {
            0
        };
        let answer = callee_cdecl!(1, u32, record);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
