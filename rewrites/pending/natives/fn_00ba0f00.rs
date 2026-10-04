// original: 0x00BA0F00 SET_CHAR_ALL_ANIMS_SPEED
// SET_CHAR_ALL_ANIMS_SPEED: forward (ped, speed-bits) to the engine call.
// The speed travels as raw float bits.
export!(cdecl, rw_00BA0F00(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, *a.add(1))
    }
});
