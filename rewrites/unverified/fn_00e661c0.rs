// original: 0x00E661C0 initialize_random_hash (proposed)

/// Hash one constant audio name and store the hash in its global slot.
///
/// Calls the game's string-hash callee (cdecl, two stack words: a pointer to
/// the read-only name `0x00e91460` and a zero seed) and stores the returned
/// 32-bit hash at the global word `0x01284670`. The stored value is
/// also the value left in EAX. No other memory is read or written.
///
/// Original: 0x00E661C0 (cdecl, no arguments; callee cleans nothing).
lf_checker_rt::export!(cdecl, rw_00E661C0() -> u32 {
    unsafe {
        const NAME: u32 = 0x00e91460;
        const SLOT: u32 = 0x01284670;
        const HASH_CALLEE: u32 = 1;
        let h: u32 =
            lf_checker_rt::callee_cdecl!(HASH_CALLEE, u32, lf_checker_rt::relocated(NAME), 0);
        lf_checker_rt::global::<u32>(SLOT).write_unaligned(h);
        h
    }
});
