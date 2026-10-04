// original: 0x00bc7100 PLAY_CAR_ANIM
/// Script native `PLAY_CAR_ANIM` (hash 0x03EE5F1C).
///
/// Forwards six script arguments: three integers (handles), one float
/// bit-pattern, and two booleans coerced from words. The first boolean's
/// temporary reuses a slot holding the caller's entry ECX, so its pushed
/// high bytes are register leftovers, not behaviour: the contract masks
/// that call argument to its low byte and the rewrite passes the clean
/// boolean. The second boolean reuses the incoming stack slot (reproduced
/// exactly from the context pointer). Stores the low byte of the engine
/// answer (zero-extended) into the return slot.
export!(cdecl, rw_00bc7100(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ctxw = ctx as u32;
        let q5 = (ctxw & !0xFF) | ((*args.add(5) != 0) as u32);
        let b4 = (*args.add(4) != 0) as u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            b4,
            q5,
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
