// original: 0x00ba0e40 SET_ANIM_GROUP_FOR_CHAR
/// Native handler `SET_ANIM_GROUP_FOR_CHAR` (script context in, engine call out).
///
/// Sets the char's animation group; forwards ped handle and group-name pointer.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00ba0e40(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0), *a.add(1))
    }
});
