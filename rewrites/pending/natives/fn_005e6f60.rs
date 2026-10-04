// original: 0x005e6f60 CODE_WANTS_MOBILE_PHONE_REMOVED
/// `CODE_WANTS_MOBILE_PHONE_REMOVED` (native hash `0x63DA2195`): store whether the engine's
/// mobile-phone-removed request flag is currently set (nonzero) into the
/// context return slot. Reads one game-global dword; makes no calls.
export!(cdecl, rw_005e6f60(ctx: *const crate::NativeCtx08) -> () {
    unsafe {
        let raised: u32 = *global(0x18b6ed8);
        *(*ctx).ret_slot = u32::from(raised != 0);
    }
});
