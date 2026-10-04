// original: 0x00e6c370 cache_moveflag_UnsetMeleeActionFlag0 (proposed)

/// Cache the id of the move flag "UnsetMeleeActionFlag0" in its dedicated global slot.
///
/// Calls the string-hasher callee (cdecl, two words: the flag-name string in
/// read-only data, then 0) and stores the returned id verbatim into the
/// `.data` slot for this flag. The value is returned in `eax` as well: the
/// original falls through to `ret` with the callee's answer still in `eax`.
///
/// Edge cases: there are no inputs (cdecl/0, no register or stack reads), so
/// the only variation across trials is the callee's scripted answer,
/// including 0 and 32-bit extremes, all stored and returned unchanged. The
/// callee lives in the encrypted region and never executes under the
/// checker; its call is intercepted and answered by script on both sides.
lf_checker_rt::export!(cdecl, rw_00e6c370() -> u32 {
    unsafe {
        const FLAG_NAME: u32 = 0x00EDA5E4;
        const SLOT: u32 = 0x0171C9DC;
        const RESOLVE_ID: u32 = 1;
        let id: u32 = lf_checker_rt::callee_cdecl!(
            RESOLVE_ID,
            u32,
            lf_checker_rt::relocated(FLAG_NAME),
            0u32
        );
        *lf_checker_rt::global::<u32>(SLOT) = id;
        id
    }
});
