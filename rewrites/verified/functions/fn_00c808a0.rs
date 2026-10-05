// original: 0x00c808a0 CONV_SMOKE_STATE

/// Name the smoke-state string for scenario kind `this+0x34` and flag `arg`.
///
/// The kind word selects a pair of read-only strings (kinds 1, 2, 4, 8 and
/// `0x100` each have their own pair; any other kind yields the fallback
/// string): a zero `arg` byte picks the second of the pair, a non-zero byte
/// the first. Returns a pointer to the chosen string.
///
/// Original: thiscall, one stack word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c808a0(this: u32, arg: u32) -> u32 {
    unsafe {
        const KIND_OFF: u32 = 0x34;
        const S_BIZ_STATE: u32 = 0xed4e18;
        const S_BIZ_RESULT: u32 = 0xed4e3c;
        const S_SMOKE_A: u32 = 0xed4ddc;
        const S_SMOKE_B: u32 = 0xed4df0;
        const S_D4_A: u32 = 0xed4ecc;
        const S_D4_B: u32 = 0xed4eec;
        const S_D8_A: u32 = 0xed4e8c;
        const S_D8_B: u32 = 0xed4ea4;
        const S_D100_A: u32 = 0xed4e68;
        const S_D100_B: u32 = 0xed4e78;
        const S_FALLBACK: u32 = 0xed4efc;
        let rel = lf_checker_rt::relocated;
        let kind = ((this + KIND_OFF) as *const u32).read_unaligned();
        let pick = (arg as u8) == 0;
        let (first, second) = match kind {
            1 => (S_SMOKE_A, S_SMOKE_B),
            2 => (S_BIZ_STATE, S_BIZ_RESULT),
            4 => (S_D4_A, S_D4_B),
            8 => (S_D8_A, S_D8_B),
            0x100 => (S_D100_A, S_D100_B),
            _ => return rel(S_FALLBACK),
        };
        rel(if pick { second } else { first })
    }
});
