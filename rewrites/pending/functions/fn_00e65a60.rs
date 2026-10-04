// original: 0x00e65a60 HIGH_FALL
/// Resolve and cache the hashed id of "HIGH_FALL".
///
/// Hashes the name through the string-hash helper (cdecl/2, stubbed by the
/// checker), stores the id in its global slot and returns it.
export!(cdecl, rw_00e65a60() -> u32 {
    unsafe {
        const NAME: u32 = 0x00E90BE8; // "HIGH_FALL"
        const SLOT: u32 = 0x012844B0;
        let id = callee_cdecl!(1, u32, relocated(NAME), 0);
        *global::<u32>(SLOT) = id;
        id
    }
});
