// original: 0x00e65b80 NIKO_NORMAL
/// Resolve and cache the hashed id of "NIKO_NORMAL".
///
/// Same shape as [`rw_00e65a60`]: hash, store in slot, return.
export!(cdecl, rw_00e65b80() -> u32 {
    unsafe {
        const NAME: u32 = 0x00E90C68; // "NIKO_NORMAL"
        const SLOT: u32 = 0x0128445C;
        let id = callee_cdecl!(1, u32, relocated(NAME), 0);
        *global::<u32>(SLOT) = id;
        id
    }
});
