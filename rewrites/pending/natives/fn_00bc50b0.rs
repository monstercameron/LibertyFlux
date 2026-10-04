// original: 0x00bc50b0 ATTACH_CAR_TO_CAR
/// Script native `ATTACH_CAR_TO_CAR` (hash 0x64146142).
///
/// Attaches two vehicles together. Rotation are in radians
///
/// Forwards 9 script arguments (self: Car, car: Car, carPartIndex: int, offsetX: float, offsetY: float, offsetZ: float, rotX: float, rotY: float, rotZ: float) to the engine in order.
/// Float arguments are forwarded as raw bit patterns (bit-exact).
/// No return slot is written.
export!(cdecl, rw_00bc50b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            *args.add(6),
            *args.add(7),
            *args.add(8),
        )
    }
});
