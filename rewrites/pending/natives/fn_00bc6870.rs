// original: 0x00bc6870 IS_CAR_IN_AREA_3D
/// Script native `IS_CAR_IN_AREA_3D` (hash 0x289D3888).
///
/// Forwards a car handle, six area-bound float bit patterns and a boolean
/// flag (coerced with `arg != 0`) to the engine, and stores the low byte
/// of its answer (zero-extended) into the return slot.
///
/// Quirk (observed): the flag is coerced into the low byte of the
/// handler's own incoming stack slot and the whole dword is pushed, so its
/// high bytes repeat the context pointer. Reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00bc6870(ctx: *const u8) -> u32 {
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
            quirked
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
