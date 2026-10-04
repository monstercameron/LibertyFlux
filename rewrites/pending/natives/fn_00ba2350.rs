// original: 0x00ba2350 SET_PED_COMPONENTS_TO_NETWORK_PLAYERSETTINGS_MODEL
/// Native handler `SET_PED_COMPONENTS_TO_NETWORK_PLAYERSETTINGS_MODEL`: forwards script args [arg0 (dword)]
/// to its engine function and returns nothing (void).
export!(cdecl, rw_00ba2350(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args.add(0));
        0
    }
});
