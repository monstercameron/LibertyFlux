// original: 0x00a016c0 LOCATE_OBJECT_3D
/// Script native `LOCATE_OBJECT_3D` (hash 0x6DB47487).
///
/// Forwards 8 script arguments to the engine in order.
/// Float arguments are forwarded as raw bit patterns (bit-exact).
///
/// Quirk (observed): the handler coerces argument 7 with
/// `arg != 0` into the low byte of its own incoming stack slot
/// and pushes the whole dword, so the pushed word's high bytes
/// repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for
/// bit-exact outgoing-call matching.
/// Stores the low byte of the engine answer (zero-extended)
/// into the return slot.
export!(cdecl, rw_00a016c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(7) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            *args.add(6),
            quirked,
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
