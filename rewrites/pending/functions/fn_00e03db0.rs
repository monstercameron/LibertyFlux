// original: 0x00e03db0 reload_stale_entries
/// Reloads entries 0xFC and 0xFF when the provider reports them stale.
///
/// Queries the state callee; when it reports ready (1), or reports empty
/// (0) while the refresh flag at 0x17ABC58 is set, reloads both entries
/// through the load callee. Returns the last observed value.
export!(cdecl, rw_00e03db0() -> u32 {
    unsafe {
        if callee_cdecl!(1, u32, 3) != 1 {
            let second = callee_cdecl!(1, u32, 3);
            if second != 0 {
                return second;
            }
            if *global::<u32>(0x17ABC58) != 1 {
                return 0;
            }
        }
        callee_cdecl!(2, u32, 0xFC);
        callee_cdecl!(2, u32, 0xFF)
    }
});
