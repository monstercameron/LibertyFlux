// original: 0x009cbf20 MUTE_GAMEWORLD_AND_POSITIONED_RADIO_FOR_TV
/// Script native `MUTE_GAMEWORLD_AND_POSITIONED_RADIO_FOR_TV` (hash 0x79974E04).
///
/// Coerces one script argument to a boolean and forwards it to the engine. The pushed word reuses the handler's own stack slot, so its high bytes repeat the context pointer (see report). No return slot is written.
export!(cdecl, rw_009cbf20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = (*args != 0) as u32;
        let coerced = (ctx as u32 & !0xFF) | flag;
        callee_cdecl!(1, u32, coerced)
    }
});
