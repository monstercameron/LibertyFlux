// original: 0x00BA2090 SET_GROUP_FOLLOW_STATUS
// SET_GROUP_FOLLOW_STATUS: forward (group, follow?) with the bool coerced
// through the incoming stack slot.
export!(cdecl, rw_00BA2090(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, coerced(ctx, *a.add(1)))
    }
});
