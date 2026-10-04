// original: 0x00e6d560 cache_lookup_opponentCheckMeleeActionFlag0

/// Resolve the name "opponentCheckMeleeActionFlag0" to its id and cache it.
///
/// Calls the shared name-lookup callee with the address of the name string
/// in read-only data and a zero second argument, stores the returned id in
/// this member's dedicated cache slot in writable data, and returns the id.
/// Takes no arguments and keeps no state besides the cache slot (cdecl, no
/// stack words; result in EAX). One of a family of identical single-purpose
/// resolvers; only the name string and the cache slot differ between members.
lf_checker_rt::export!(cdecl, rw_00e6d560() -> u32 {
    unsafe {
        /// File VA of the name string in read-only data.
        const NAME: u32 = 0x00EEEA18;
        /// File VA of this member's cache slot in writable data.
        const CACHE_SLOT: u32 = 0x017A4C88;
        /// Intercepted name-lookup callee id (see the contract).
        const LOOKUP: u32 = 1;
        /// Second lookup argument; always zero.
        const FLAGS: u32 = 0;
        let id: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, lf_checker_rt::relocated(NAME), FLAGS);
        lf_checker_rt::global::<u32>(CACHE_SLOT).write_unaligned(id);
        id
    }
});
