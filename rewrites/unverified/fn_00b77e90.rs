// original: 0x00b77e90 slot_has_key (proposed)

/// Report whether any slot holds an object with `key`.
///
/// Calls the slot search and normalises its result to exactly 1 (found) or
/// 0 (not found).
///
/// Original: cdecl with one stack word, plain `ret`.
lf_checker_rt::export!(cdecl, rw_00b77e90(key: u32) -> u32 {
    unsafe {
        const SEARCH: u32 = 1;
        let p: u32 = lf_checker_rt::callee_cdecl!(SEARCH, u32, key);
        u32::from(p != 0)
    }
});
