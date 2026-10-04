// original: 0x00bc8380 VEHICLE_CAN_BE_TARGETTED_BY_HS_MISSILE
/// Script native `VEHICLE_CAN_BE_TARGETTED_BY_HS_MISSILE` (hash 0x27607F64).
///
/// Forwards two script arguments (a vehicle handle and a boolean flag
/// coerced with `arg != 0`) to the engine. No return slot is written.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the plain 0/1 flag and the contract masks
/// this call argument (`call_skip`); only the low byte is meaningful.
export!(cdecl, rw_00bc8380(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, flag)
    }
});
