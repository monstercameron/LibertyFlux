// original: 0x00949900 reemit_valid_pairs (proposed)

/// Re-emit every fully valid handle pair of the 10-pair global table.
///
/// Reads 10 dword pairs from `TABLE`. A pair is skipped when either word
/// is -1 (each compared as a full word against -1); otherwise callee 1
/// runs with (`first`, `second`). No return channel is compared.
///
/// Original: 0x00949900 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00949900() -> u32 {
    unsafe {
        const TABLE: u32 = 0x11D95A0;
        const PAIRS: u32 = 10;
        const INVALID: u32 = 0xFFFF_FFFF;
        const EMIT: u32 = 1;
        let mut i = 0u32;
        while i < PAIRS {
            let p = lf_checker_rt::relocated(TABLE).wrapping_add(i.wrapping_mul(8));
            let a = (p as *const u32).read();
            if a != INVALID {
                let b = (p.wrapping_add(4) as *const u32).read();
                if b != INVALID {
                    lf_checker_rt::callee_cdecl!(EMIT, u32, a, b);
                }
            }
            i += 1;
        }
        0
    }
});
