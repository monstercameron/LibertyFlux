// original: 0x00bc7bb0 SET_POLICE_FOCUS_WILL_TRACK_CAR
/// Script native `SET_POLICE_FOCUS_WILL_TRACK_CAR` (hash 0x0D374615).
///
/// Forwards a vehicle handle and a boolean coerced from a word. The
/// boolean reuses the incoming stack slot as its temporary, so the pushed
/// dword repeats the context pointer's high bytes; reproduced exactly
/// from the context pointer. No return slot is written.
export!(cdecl, rw_00bc7bb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ctxw = ctx as u32;
        let q1 = (ctxw & !0xFF) | ((*args.add(1) != 0) as u32);
        callee_cdecl!(1, u32, *args, q1)
    }
});
