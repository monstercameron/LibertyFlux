// original: 0x00ba1600 SET_CHAR_IN_CUTSCENE
/// Script native `SET_CHAR_IN_CUTSCENE` (hash 0x12850007).
///
/// Forwards a character handle and a boolean coerced from a word. The
/// boolean reuses the incoming stack slot as its temporary, so the pushed
/// dword repeats the context pointer's high bytes; reproduced exactly
/// from the context pointer. No return slot is written.
export!(cdecl, rw_00ba1600(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ctxw = ctx as u32;
        let q1 = (ctxw & !0xFF) | ((*args.add(1) != 0) as u32);
        callee_cdecl!(1, u32, *args, q1)
    }
});
