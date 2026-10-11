// original: 0x008C5C70 fn_008c5c70
/// Hash the supplied label through the intercepted helper, then search the
/// active sixteen-byte records by exact key. Return the zero-based match index
/// or 0xffffffff when the active prefix contains no matching key.
lf_checker_rt::export!(thiscall, rw_008c5c70(this: u32, label: u32) -> u32 {
    unsafe { search_hash_index(this, label, false) }
});

/// Deliberately wrong version: return a one-based index on a match.
lf_checker_rt::export!(thiscall, mut_008c5c70(this: u32, label: u32) -> u32 {
    unsafe { search_hash_index(this, label, true) }
});
