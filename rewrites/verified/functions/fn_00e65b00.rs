// original: 0x00e65b00 LUIS_ANGRY
/// Resolve and cache the hashed id of "LUIS_ANGRY".
///
/// Same shape as [`rw_00e65a60`]: hash, store in slot, return.
export!(cdecl, rw_00e65b00() -> u32 {
    unsafe {
        const NAME: u32 = 0x00E90CC4; // "LUIS_ANGRY"
        const SLOT: u32 = 0x0128459C;
        let id = callee_cdecl!(1, u32, relocated(NAME), 0);
        *global::<u32>(SLOT) = id;
        id
    }
});
