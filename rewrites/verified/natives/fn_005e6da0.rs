// original: 0x005e6da0 GET_MOBILE_PHONE_SCALE
/// Script native `GET_MOBILE_PHONE_SCALE` (hash 0x1E951606).
///
/// Copies the engine's mobile-phone scale factor (a single float held in a
/// global) into the return slot. Takes no script arguments and makes no calls.
export!(cdecl, rw_005e6da0(ctx: *const u8) -> u32 {
    unsafe {
        let scale = *global::<f32>(0x018b6eec);
        let slot = *(ctx as *const u32) as *mut f32;
        *slot = scale;
        slot as u32
    }
});
