// original: 0x009cbf20 MUTE_GAMEWORLD_AND_POSITIONED_RADIO_FOR_TV
/// Script native `MUTE_GAMEWORLD_AND_POSITIONED_RADIO_FOR_TV` (hash 0x79974E04).
///
/// Coerces one script argument to a boolean and forwards it to the engine. The original's pushed word reuses its own stack slot (see report); the rewrite passes the clean flag and the contract masks that call argument (call_skip). No return slot is written.
export!(cdecl, rw_009cbf20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = (*args != 0) as u32;
        let coerced = flag;
        callee_cdecl!(1, u32, coerced)
    }
});
