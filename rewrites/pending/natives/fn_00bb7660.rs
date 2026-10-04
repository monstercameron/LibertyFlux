// original: 0x00BB7660 START_LOAD_SCENE
/// F23 START_LOAD_SCENE: forwards args[0..2] by value (bitwise float words).
export!(cdecl, rn10_start_load_scene(ctx: *const u32) -> u32 {
    unsafe {
        let (_, args) = ctx_parts(ctx);
        callee_cdecl!(2, u32, *args, *args.add(1), *args.add(2));
        0
    }
});
