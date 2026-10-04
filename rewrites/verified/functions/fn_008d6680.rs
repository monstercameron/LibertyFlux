// original: 0x008d6680 flag_probe_or_table_fallback
// Takes an ignored word and a flag word. When the flag's low byte is clear
// it probes the subsystem, returning null early when the probe reports a
// live record. Otherwise it resolves through a host table, returning a
// fallback slot address unless the host record selects the query path.
export!(cdecl, rw_008d6680(_unused: u32, flag: u32) -> u32 {
    unsafe {
        if flag & 0xFF == 0 {
            let probe: u32 = callee_cdecl!(1, u32, 0);
            if probe != 0 && *((probe + 0x210) as *const u8) != 0 {
                return 0;
            }
        }
        let host: u32 = callee_thiscall!(2, u32, relocated(0x0103_E498));
        if host == 0 || *((host + 0x1C4) as *const u8) & 8 != 0 {
            return relocated(0x0118_5C08);
        }
        let sel = *global::<u8>(0x0103_E4B4) as u32;
        callee_cdecl!(3, u32, sel)
    }
});
