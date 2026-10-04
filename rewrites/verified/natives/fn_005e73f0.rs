// original: 0x005e73f0 DRAW_FRONTEND_HELPER_TEXT
/// Script native `DRAW_FRONTEND_HELPER_TEXT` (hash 0x44E14770).
///
/// Forwards three script arguments (two integers and a boolean flag) to the
/// engine, unless a global frontend state byte is nonzero, in which case the
/// handler returns early without calling. The flag is coerced with
/// `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching. No return slot is written.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_005e73f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(2) != 0);
        // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
        let quirked = flag;
        if *global::<u8>(0x011609F6) != 0 {
            return args as u32;
        }
        callee_cdecl!(1, u32, *args, *args.add(1), quirked)
    }
});
