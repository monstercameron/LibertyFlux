// original: 0x00ba1940 SET_CHAR_PROP_INDEX_TEXTURE
//
// Forwards the character handle and three integers to the engine setter.
// No return value.
export!(cdecl, rw_00ba1940(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
