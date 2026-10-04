// original: 0x00e65b60 NIKO_ANGRY
/// Resolve and cache the hashed id of "NIKO_ANGRY".
///
/// Same shape as [`rw_00e65a60`]: hash, store in slot, return.
export!(cdecl, rw_00e65b60() -> u32 {
    unsafe {
        const NAME: u32 = 0x00E90C74; // "NIKO_ANGRY"
        const SLOT: u32 = 0x01284444;
        let id = callee_cdecl!(1, u32, relocated(NAME), 0);
        *global::<u32>(SLOT) = id;
        id
    }
});
