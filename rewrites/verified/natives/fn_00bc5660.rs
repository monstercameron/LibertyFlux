// original: 0x00bc5660 DOES_CAR_HAVE_ROOF
/// Native handler `DOES_CAR_HAVE_ROOF` (script context in, engine call out).
///
/// True if the car has a roof; forwards the vehicle handle, stores the byte answer as dword.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00bc5660(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        let r = callee_cdecl!(1, u32, *a.add(0));
        // The original keeps only AL and zero-extends it into the slot.
        *(*ctx).ret = r & 0xFF;
        (*ctx).ret as u32
    }
});
