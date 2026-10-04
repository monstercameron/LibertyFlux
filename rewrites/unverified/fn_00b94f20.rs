// original: 0x00b94f20 SET_MISSION_FLAG
/// Script native `SET_MISSION_FLAG` (hash 0x4FE923DC).
///
/// Forwards a single boolean coerced from a word. The boolean reuses the
/// incoming stack slot as its temporary, so the pushed dword repeats the
/// context pointer's high bytes; reproduced exactly from the context
/// pointer. No return slot is written.
export!(cdecl, rw_00b94f20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ctxw = ctx as u32;
        let q0 = (ctxw & !0xFF) | ((*args != 0) as u32);
        callee_cdecl!(1, u32, q0)
    }
});
