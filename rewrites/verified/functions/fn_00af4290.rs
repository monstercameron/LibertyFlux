// original: 0x00AF4290 stream_slot_lookup_a (proposed)

/// Hash a name and publish the matching stream slot index.
///
/// The argument is hashed with a zero seed word, the index table is
/// searched for the hash, and the resulting index is stored to global
/// slot A and returned. Both callees are scripted by the checker; this
/// rewrite covers the argument plumbing and the publication.
///
/// Original: 0x00AF4290 (cdecl, one stack word, two direct callees).
lf_checker_rt::export!(cdecl, rw_00af4290(arg: u32) -> u32 {
    unsafe {
        const HASH_CALLEE: u32 = 1;
        const LOOKUP_CALLEE: u32 = 2;
        const SLOT_A: u32 = 0x0103F980;
        let h = lf_checker_rt::callee_cdecl!(HASH_CALLEE, u32, arg, 0);
        let idx = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, h);
        (lf_checker_rt::global::<u32>(SLOT_A)).write_unaligned(idx);
        idx
    }
});
