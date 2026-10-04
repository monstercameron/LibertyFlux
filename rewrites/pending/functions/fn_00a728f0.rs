// original: 0x00a728f0 gate_by_registry_probe (proposed)
/// True (the low bit of the word at `+0x274` shifted right by 2) when two
/// registry probes of the attached detail both pass, else 0.
///
/// `cdecl`, no stack words. Fetches the player record (callee 1); a null
/// record returns 0. Resolves the detail (callee 2, thiscall on
/// `record+0x2b0`); a null detail skips the probes and returns the bit.
/// Otherwise probes the key at `detail+0x18` twice: the first probe's word
/// at `+4` equal to 6 returns 0, the second probe's word at `+4` equal to
/// 0 returns 0, any other combination returns the bit.
lf_checker_rt::export!(cdecl, rw_00a728f0() -> u8 {
    unsafe {
        const DETAIL_OFF: u32 = 0x2b0;
        const KEY_OFF: u32 = 0x18;
        const PROBE_OFF: u32 = 0x4;
        const FLAG_OFF: u32 = 0x274;
        const VETO_A: u32 = 6;
        let rec = lf_checker_rt::callee_cdecl!(1, u32,);
        if rec == 0 {
            return 0;
        }
        let det = lf_checker_rt::callee_thiscall!(2, u32, rec.wrapping_add(DETAIL_OFF));
        if det == 0 {
            return ((((rec + FLAG_OFF) as *const u32).read_unaligned() >> 2) & 1) as u8;
        }
        let key = ((det + KEY_OFF) as *const u32).read_unaligned();
        let r1 = lf_checker_rt::callee_cdecl!(3, u32, key);
        if ((r1 + PROBE_OFF) as *const u32).read_unaligned() == VETO_A {
            return 0;
        }
        let r2 = lf_checker_rt::callee_cdecl!(3, u32, key);
        if ((r2 + PROBE_OFF) as *const u32).read_unaligned() == 0 {
            return 0;
        }
        ((((rec + FLAG_OFF) as *const u32).read_unaligned() >> 2) & 1) as u8
    }
});
