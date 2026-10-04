// original: 0x00E67450 init_256_pairs_and_register

/// Run two callees over 256 fixed object pairs, then register a fixed stub.
///
/// For each `k` in 0..256 with `obj = BASE + k * STRIDE`: call the first
/// callee with `obj` in ECX and the second with `obj + SECOND_OFF` in ECX
/// (neither takes stack arguments). Finally register the constant stub `STUB`
/// and return the registrar's answer: 513 outgoing calls per invocation.
/// Addresses are loader-relocated in the original and derived from the
/// relocated image base here.
///
/// Original: 0x00E67450 (cdecl, no arguments, 513 outgoing calls, returns last result).
lf_checker_rt::export!(cdecl, rw_00e67450() -> u32 {
    const BASE: u32 = 0x012E2520;
    const COUNT: u32 = 256;
    const STRIDE: u32 = 0xE0;
    const SECOND_OFF: u32 = 0x84;
    const STUB: u32 = 0x00E72210;
    let mut obj = lf_checker_rt::relocated(BASE);
    let mut k: u32 = 0;
    while k < COUNT {
        lf_checker_rt::callee_thiscall!(1, u32, obj);
        lf_checker_rt::callee_thiscall!(2, u32, obj.wrapping_add(SECOND_OFF));
        obj = obj.wrapping_add(STRIDE);
        k += 1;
    }
    lf_checker_rt::callee_cdecl!(3, u32, lf_checker_rt::relocated(STUB))
});
