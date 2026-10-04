// original: 0x00ba1e10 SET_DECISION_MAKER_ATTRIBUTE_CAN_CHANGE_TARGET
/// Script native `SET_DECISION_MAKER_ATTRIBUTE_CAN_CHANGE_TARGET`
/// (hash 0x51F54148).
///
/// Forwards a handle and a boolean flag (`arg != 0`) to the engine.
/// No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred). The rewrite passes the clean 0/1 flag; the pushed
/// word carries context-pointer high bytes on the original side, so
/// the contract skips that call argument (v2 has no low-byte compare).
export!(cdecl, rw_00ba1e10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);

        callee_cdecl!(1, u32, *args, flag)
    }
});
