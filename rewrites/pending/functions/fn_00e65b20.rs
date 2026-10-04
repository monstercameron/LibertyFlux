// original: 0x00e65b20 LUIS_NORMAL
/// Resolve and cache the hashed id of "LUIS_NORMAL".
///
/// Same shape as [`rw_00e65a60`]: hash, store in slot, return.
export!(cdecl, rw_00e65b20() -> u32 {
    unsafe {
        const NAME: u32 = 0x00E90CB0; // "LUIS_NORMAL"
        const SLOT: u32 = 0x01284420;
        let id = callee_cdecl!(1, u32, relocated(NAME), 0);
        *global::<u32>(SLOT) = id;
        id
    }
});
