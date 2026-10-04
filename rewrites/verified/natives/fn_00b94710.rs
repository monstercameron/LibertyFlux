// original: 0x00b94710 GET_STRING_WIDTH
/// Script native `GET_STRING_WIDTH` (hash 0x64660709).
///
/// Forwards one script argument (a string reference) to the engine text
/// measurement routine, which answers with a float. The float bits are
/// stored into the return slot.
///
/// Quirk (observed): the handler parks the float in its own incoming stack
/// slot before copying it to the return slot, so the slot is clobbered; a
/// rewrite cannot reproduce that clobber and the contract compares the
/// stored value instead.
export!(cdecl, rw_00b94710(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const u32) as *mut f32;
        let width: f32 = callee_cdecl!(1, f32, *args);
        *slot = width;
        slot as u32
    }
});
