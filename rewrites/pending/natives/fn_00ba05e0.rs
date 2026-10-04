// original: 0x00ba05e0 LOCATE_CHAR_IN_CAR_2D
/// Native handler `LOCATE_CHAR_IN_CAR_2D` (script context in, engine call out).
///
/// True if the char is in a car inside the 2D box; forwards handle, four floats and a coerced flag.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00ba05e0(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        let r = callee_cdecl!(1, u32, *a.add(0), *a.add(1), *a.add(2), *a.add(3), *a.add(4), u32::from(*a.add(5) != 0));
        // The original keeps only AL and zero-extends it into the slot.
        *(*ctx).ret = r & 0xFF;
        (*ctx).ret as u32
    }
});
