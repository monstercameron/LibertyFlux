// original: 0x00ba1db0 SET_CREATE_RANDOM_COPS
/// Script native `SET_CREATE_RANDOM_COPS` (hash 0x5C832C1F).
///
/// Forwards a single boolean coerced from a word. The boolean reuses the
/// incoming stack slot as its temporary, so the pushed dword repeats the
/// context pointer's high bytes; reproduced exactly from the context
/// pointer. No return slot is written.
export!(cdecl, rw_00ba1db0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ctxw = ctx as u32;
        let q0 = (ctxw & !0xFF) | ((*args != 0) as u32);
        callee_cdecl!(1, u32, q0)
    }
});
