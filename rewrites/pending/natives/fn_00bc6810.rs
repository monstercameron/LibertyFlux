// original: 0x00bc6810 IS_CAR_IN_AREA_2D
/// Script native `IS_CAR_IN_AREA_2D` (hash 0x7EA03481).
///
/// Forwards 6 script arguments (a vehicle handle, four area-bound floats and a boolean flag) to the engine.
/// Float arguments are copied as raw bits, so the forward is bit-exact.
/// The flag argument is coerced with `arg != 0`.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the plain 0/1 flag and the contract masks
/// this call argument (`call_skip`); only the low byte is meaningful.
/// Stores the low byte of the engine answer (zero-extended) into the return slot.
export!(cdecl, rw_00bc6810(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(5) != 0);
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), flag);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
