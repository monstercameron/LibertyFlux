// original: 0x00bc76c0 SET_CAR_NOT_DAMAGED_BY_RELATIONSHIP_GROUP
/// Script native `SET_CAR_NOT_DAMAGED_BY_RELATIONSHIP_GROUP` (hash 0x3AAD447A).
///
/// Forwards three script arguments (a vehicle handle, a boolean flag and a
/// relationship group id) to the engine. The flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the contract masks
/// that call argument (checks.call_skip). No return slot is written.
export!(cdecl, rw_00bc76c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, flag, *args.add(2))
    }
});
