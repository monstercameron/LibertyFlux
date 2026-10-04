// original: 0x00bc7a50 SET_LOAD_COLLISION_FOR_CAR_FLAG
/// Script native `SET_LOAD_COLLISION_FOR_CAR_FLAG` (hash 0x1E5C50B5).
///
/// Forwards a vehicle handle and a boolean-coerced flag to the engine. The flag word reuses the handler's own stack slot, so its high bytes repeat the context pointer (see report). No return slot is written.
export!(cdecl, rw_00bc7a50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = (*args.add(1) != 0) as u32;
        let coerced = (ctx as u32 & !0xFF) | flag;
        callee_cdecl!(1, u32, *args.add(0), coerced)
    }
});
