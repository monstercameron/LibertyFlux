// original: 0x005e6f80 CODE_WANTS_MOBILE_PHONE_REMOVED_FOR_WEAPON_SWITCHING
/// Script native `CODE_WANTS_MOBILE_PHONE_REMOVED_FOR_WEAPON_SWITCHING` (hash 0x736027E6).
///
/// Tests two flag bits of an engine state byte and stores whether either is
/// set (1 or 0) into the return slot. It takes no script arguments and makes
/// no engine call.
export!(cdecl, rw_005e6f80(ctx: *const u8) -> u32 {
    unsafe {
        let flags = lf_k2_rt::global::<u8>(0x018b6ed8);
        let hit = u32::from(*flags & 0xC0 != 0);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = hit;
        slot as u32
    }
});
