// original: 0x00B9EC90 GET_CHAR_ANIM_CURRENT_TIME
//
// Forwards the character handle, the animation group and name plus the
// output slot to the engine query; like its vehicle twin it never touches
// the return slot itself.
export!(cdecl, rw_00b9ec90(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
