// original: 0x005e6320 NativeImpl_DRAW_SPRITE_PHOTO
// rw_005e6320: queue a photo sprite for drawing.
//
// Forwards the sprite's geometry and color words, together with the current
// draw-list handle, to the sprite batcher. (The original also stages two
// constant pairs and copies the geometry words through frame scratch space;
// both are dead stores that never reach the call, so they are omitted.)
export!(cdecl, rw_005e6320(
    a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32,
    a8: u32,
) -> u32 {
    unsafe {
        let list = *global::<u32>(0x18B6EE8);
        callee_cdecl!(
            1, u32, list, 0xFFFFFFFE, a0, a1, a2, a3, a4, a5, a6, a7, a8, 0, 6
        )
    }
});
