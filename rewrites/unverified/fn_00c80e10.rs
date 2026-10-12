// original: 0x00C80E10 scenario_bounds_contain_point

/// Search the runtime scenario-entry table for a match with the supplied
/// point. A signed nonpositive entry count returns false. Each candidate is
/// passed with the point to the helper; the first nonzero low byte returns
/// true, and exhausting the table returns false. The table count is read from
/// the runtime-filled global word at `0x016F80C4`.
lf_checker_rt::export!(cdecl, rw_00c80e10(point: u32) -> u32 {
    const ENTRY_COUNT_GLOBAL: u32 = 0x016F_80C4;
    const ENTRY_TABLE: u32 = 0x016F_80D0;
    const ENTRY_STRIDE: u32 = 0x20;
    const MATCH_CANDIDATE: u32 = 1;

    let entry_count = unsafe {
        lf_checker_rt::global::<i32>(ENTRY_COUNT_GLOBAL).read_unaligned()
    };
    if entry_count <= 0 {
        return 0;
    }

    for index in 0..entry_count as u32 {
        let candidate = ENTRY_TABLE.wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
        let matched = lf_checker_rt::callee_thiscall!(
            MATCH_CANDIDATE,
            u32,
            candidate,
            point,
        );
        if matched as u8 != 0 {
            return 1;
        }
    }
    0
});
