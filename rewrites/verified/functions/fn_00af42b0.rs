// original: 0x00AF42B0 stream_slot_lookup_b (proposed)

/// Hash a name and publish the matching stream slot index.
///
/// Same shape as the sibling slot-A lookup: the argument is hashed with a
/// zero seed word, the index table is searched for the hash, and the
/// resulting index is stored to global slot B and returned.
///
/// Original: 0x00AF42B0 (cdecl, one stack word, two direct callees).
lf_checker_rt::export!(cdecl, rw_00af42b0(arg: u32) -> u32 {
    unsafe {
        const HASH_CALLEE: u32 = 1;
        const LOOKUP_CALLEE: u32 = 2;
        const SLOT_B: u32 = 0x0103F9AC;
        let h = lf_checker_rt::callee_cdecl!(HASH_CALLEE, u32, arg, 0);
        let idx = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, h);
        (lf_checker_rt::global::<u32>(SLOT_B)).write_unaligned(idx);
        idx
    }
});
