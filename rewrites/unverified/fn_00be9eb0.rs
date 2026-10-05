// original: 0x00be9eb0 code_dispatch_33_6c (proposed)

/// Dispatch a small integer code to a global, a fetch call, or zero.
///
/// Takes the code. Codes 0x33 and 0x34 read the dword at file address
/// `0x1231314`; codes 0x4E and 0x6C fetch through callee 1 (cdecl, one word:
/// the relocated constant 0xEB99D4) and return its answer; every other code
/// returns 0. (The original implements this as a byte-indexed jump table over
/// codes 0x33..0x6C with a range guard; the rewrite states the mapping
/// directly, which the jump-table bytes verify.)
///
/// Original: cdecl, one stack word, plain `ret`, returns `eax`.
lf_checker_rt::export!(cdecl, rw_00be9eb0(code: u32) -> u32 {
    unsafe {
        const VALUE_GLOBAL: u32 = 0x1231314;
        const FETCH_ARG_FILE: u32 = 0xeb99d4;
        const FETCH: u32 = 1;

        match code {
            0x33 | 0x34 => lf_checker_rt::global::<u32>(VALUE_GLOBAL).read_unaligned(),
            0x4E | 0x6C => {
                lf_checker_rt::callee_cdecl!(FETCH, u32, lf_checker_rt::relocated(FETCH_ARG_FILE))
            }
            _ => 0,
        }
    }
});
