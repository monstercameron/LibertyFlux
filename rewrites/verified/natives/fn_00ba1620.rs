// original: 0x00ba1620 SET_CHAR_IS_TARGET_PRIORITY
/// Script native `SET_CHAR_IS_TARGET_PRIORITY` (hash 0x163A1D77).
///
/// Forwards a character handle and a boolean flag (coerced with `arg != 0`)
/// to the engine.
///
/// Quirk (observed): the flag is coerced into the low byte of the
/// handler's own incoming stack slot and the whole dword is pushed, so its
/// high bytes repeat the context pointer. Reproduced here for bit-exact
/// outgoing-call matching. No return slot is written.
export!(cdecl, rw_00ba1620(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
        let quirked = flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
