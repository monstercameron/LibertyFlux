// original: 0x00bc7fc0 SET_VEH_INTERIORLIGHT
/// Script native `SET_VEH_INTERIORLIGHT` (hash 0x49EA22C8).
///
/// Forwards two script arguments (a vehicle handle and a boolean flag) to
/// the engine. The flag is coerced with `arg != 0`. No return slot is
/// written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes a clean 0/1 and the contract masks
/// that call argument, so the leftover bytes are neither reproduced nor
/// compared.
export!(cdecl, rw_00bc7fc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, flag)
    }
});
