// original: 0x005e63b0 NativeImpl_DRAW_SPRITE_FRONT_BUFF
// rw_005e63b0: queue a front-buffer sprite for drawing.
//
// Resolves the front-buffer target, then forwards the sprite's geometry and
// color words with it to the sprite batcher. (Same dead frame stores as its
// photo sibling; omitted for the same reason.)
export!(cdecl, rw_005e63b0(
    a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32,
    a8: u32,
) -> u32 {
    unsafe {
        let handle: u32 = *global::<u32>(0x11A28F4);
        let target: u32 = callee_thiscall!(1, u32, handle);
        callee_cdecl!(
            2, u32, target, 0xFFFFFFFE, a0, a1, a2, a3, a4, a5, a6, a7, a8, 0,
            6
        )
    }
});
