// original: 0x00ba1f90 SET_DECISION_MAKER_ATTRIBUTE_WEAPON_ACCURACY
/// `SET_DECISION_MAKER_ATTRIBUTE_WEAPON_ACCURACY` (native hash `0x21B8337F`): forward 2 script arguments to the
/// `NativeImpl_SET_DECISION_MAKER_ATTRIBUTE_WEAPON_ACCURACY` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00ba1f90(ctx: *const crate::NativeCtx) -> () {
    unsafe {
        let args = (*ctx).args;
        callee_cdecl!(1, u32, *args, *args.add(1));
    }
});
