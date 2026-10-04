// original: 0x00e65ae0 JOHNNY
/// Resolve and cache the hashed id of "JOHNNY".
///
/// Same shape as [`rw_00e65a60`]: hash, store in slot, return.
export!(cdecl, rw_00e65ae0() -> u32 {
    unsafe {
        const NAME: u32 = 0x00E90C80; // "JOHNNY"
        const SLOT: u32 = 0x012844E8;
        let id = callee_cdecl!(1, u32, relocated(NAME), 0);
        *global::<u32>(SLOT) = id;
        id
    }
});
