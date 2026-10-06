// original: 0x00bc8340 TURN_OFF_VEHICLE_EXTRA
/// Script native `TURN_OFF_VEHICLE_EXTRA`.
///
/// Forwards 3 script arguments to the engine: argument 0 forwarded unchanged; argument 1 forwarded unchanged; argument 2 coerced to 0/1 in the low byte of a word whose upper bytes repeat the context pointer.
///
/// No return slot is written; the engine answer is returned.
export!(cdecl, rw_00bc8340(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), (ctx as u32 & 0xFFFF_FF00) | u32::from(*args.add(2) != 0))
    }
});
