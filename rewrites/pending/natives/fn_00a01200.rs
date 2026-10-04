// original: 0x00a01200 HAS_OBJECT_COLLIDED_WITH_ANYTHING
/// Script native `HAS_OBJECT_COLLIDED_WITH_ANYTHING` (hash 0x106811E4).
///
/// Forwards one script argument (an object handle) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00a01200(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
