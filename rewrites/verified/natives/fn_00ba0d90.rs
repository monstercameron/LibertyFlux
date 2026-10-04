// original: 0x00ba0d90 REVIVE_INJURED_PED
/// Revive an injured ped.
///
/// Forwards the ped handle (argument 0) to the engine implementation. Returns
/// whatever the engine call returned.
export!(cdecl, rw_00ba0d90(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
