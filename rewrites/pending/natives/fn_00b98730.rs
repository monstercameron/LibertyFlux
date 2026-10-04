// original: 0x00b98730 SHAKE_PAD
/// Script native `SHAKE_PAD` (hash 0x66CC16BD).
///
/// Forwards three rumble arguments (pad index, duration and intensity) to
/// the engine's input worker. No return slot is written.
export!(cdecl, rw_00b98730(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
