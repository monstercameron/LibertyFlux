// original: 0x00e65a80 INHALE
/// Resolve and cache the hashed id of "INHALE".
///
/// Same shape as [`rw_00e65a60`]: hash, store in slot, return.
export!(cdecl, rw_00e65a80() -> u32 {
    unsafe {
        const NAME: u32 = 0x00E90C44; // "INHALE"
        const SLOT: u32 = 0x01284454;
        let id = callee_cdecl!(1, u32, relocated(NAME), 0);
        *global::<u32>(SLOT) = id;
        id
    }
});
