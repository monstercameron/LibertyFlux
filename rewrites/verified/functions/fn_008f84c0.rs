// original: 0x008f84c0 input_lookup_slot (proposed)

/// Map a key through the hash callee to its slot in the lookup table.
///
/// `key` is hashed by the hash callee (cdecl, one word, same answer every
/// call in a trial) and the answer is compared, as an UNSIGNED equality,
/// against each of the 73 dwords of the global table in order. The first
/// match wins and its index is returned; with no match the function returns
/// 1. The loop bound is the small constant 73, so the signed loop-end
/// compare behaves identically to an unsigned one.
///
/// Cdecl: one stack word, caller cleans.
lf_checker_rt::export!(cdecl, rw_008f84c0(key: u32) -> u32 {
    unsafe {
        const C_HASH: u32 = 1;
        const G_TABLE: u32 = 0x118dc70;
        const SLOTS: u32 = 0x49;
        const MISS: u32 = 1;
        let base = lf_checker_rt::global::<u32>(G_TABLE);
        let mut i = 0u32;
        while i < SLOTS {
            let h: u32 = lf_checker_rt::callee_cdecl!(C_HASH, u32, key);
            if h == base.add(i as usize).read_unaligned() {
                return i;
            }
            i += 1;
        }
        MISS
    }
});
