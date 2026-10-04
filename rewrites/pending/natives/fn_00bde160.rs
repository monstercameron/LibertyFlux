// original: 0x00bde160 IS_PLACE_CAR_BOMB_ACTIVE
/// Native handler `IS_PLACE_CAR_BOMB_ACTIVE` (script context in, engine call out).
///
/// True while the place-car-bomb task is active; zero-arg engine query, stores the byte answer.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00bde160(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        let r = callee_cdecl!(1, u32, );
        // The original keeps only AL and zero-extends it into the slot.
        *(*ctx).ret = r & 0xFF;
        (*ctx).ret as u32
    }
});
