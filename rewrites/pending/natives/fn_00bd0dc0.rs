// original: 0x00BD0DC0 FORCE_CHAR_TO_DROP_WEAPON
//
// Forwards the character handle (argument 0) to the engine routine that
// makes the character drop its weapon. No return value.
export!(cdecl, rw_00bd0dc0(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        callee_cdecl!(1, u32, *args)
    }
});
