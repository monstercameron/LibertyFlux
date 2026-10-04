// original: 0x00E67300 setup_two_banks_and_register

/// Set up two fixed memory banks, then register a fixed stub.
///
/// For each of the two banks (`BASE + k * STRIDE`, `k` in 0..2): call the
/// fill callee (stdcall, four stack arguments) twice, first with
/// `(bank - 0x610, 0x610, 1, SRC_A)` then with `(bank, 0x9C0, 1, SRC_B)`, then
/// call the per-bank callee with `bank + 0x9C4` in ECX. Finally register the
/// constant stub `STUB` with the registrar callee and return its answer. All
/// addresses are loader-relocated in the original and derived from the
/// relocated image base here.
///
/// Original: 0x00E67300 (cdecl, no arguments, seven outgoing calls, returns last result).
lf_checker_rt::export!(cdecl, rw_00e67300() -> u32 {
    const BASE: u32 = 0x012E03C0;
    const STRIDE: u32 = 0x1090;
    const SRC_A: u32 = 0x00A34570;
    const SRC_B: u32 = 0x00847600;
    const STUB: u32 = 0x00E721C0;
    for k in 0..2u32 {
        let bank = lf_checker_rt::relocated(BASE).wrapping_add(k.wrapping_mul(STRIDE));
        lf_checker_rt::callee_stdcall!(1, u32, bank.wrapping_sub(0x610), 0x610, 1,
                                       lf_checker_rt::relocated(SRC_A));
        lf_checker_rt::callee_stdcall!(1, u32, bank, 0x9c0, 1, lf_checker_rt::relocated(SRC_B));
        lf_checker_rt::callee_thiscall!(2, u32, bank.wrapping_add(0x9c4));
    }
    lf_checker_rt::callee_cdecl!(3, u32, lf_checker_rt::relocated(STUB))
});
