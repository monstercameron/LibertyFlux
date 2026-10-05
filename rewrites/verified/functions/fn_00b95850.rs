// original: 0x00b95850 NativeImpl_CLEAR_ANGLED_AREA_OF_CARS

/// Clears cars from an angled area described by two overlapping triples.
///
/// Packs the seven float arguments into two frame triples (`a0`, `a1`,
/// `a2`) and (`a3`, `a4`, `a5`) — the `(an instruction of the original)` slot only reserves the
/// outgoing word that is then overwritten with `a6` — and calls
/// `CLEAR_CALLEE` with (second, first, `a6`, 0).
///
/// Both buffer pointers are skipped call arguments with snapshot-verified
/// contents (see `narrowed`).
///
/// Original: 0x00B95850 (cdecl, seven stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b95850(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    const CLEAR_CALLEE: u32 = 1;
    let mut first = [a3, a4, a5];
    let mut second = [a0, a1, a2];
    let _: u32 = lf_checker_rt::callee_cdecl!(
        CLEAR_CALLEE, u32, second.as_mut_ptr() as u32, first.as_mut_ptr() as u32, a6, 0
    );
    0
});
