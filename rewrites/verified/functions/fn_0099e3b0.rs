// original: 0x0099E3B0 audio_cached_word_or_resolve (proposed)

/// Return a cached 16-bit word, resolving it through two helpers on a miss.
///
/// `this`+0x9C is a cache-valid flag (any nonzero value hits) and `this`+0xC8
/// holds the cached word. On a hit only the low 16 bits are defined (the
/// original moves `ax`, leaving the entry residue above); the contract fixes
/// entry `eax` to zero so the full return compares. On a miss the id helper
/// (callee 1, thiscall/1 of constant 1) maps the object to a key and the
/// resolver (callee 2, thiscall/1 on the shared audio manager) turns the key
/// into the answer, which is returned in full.
lf_checker_rt::export!(thiscall, rw_0099E3B0(this: u32) -> u32 {
    unsafe {
        const CACHE_VALID: u32 = 0x9C;
        const CACHE_WORD: u32 = 0xC8;
        const MANAGER: u32 = 0x01288780;
        const ID_CALLEE: u32 = 1;
        const RESOLVE_CALLEE: u32 = 2;
        let flag = ((this.wrapping_add(CACHE_VALID)) as *const u32).read_unaligned();
        if flag != 0 {
            return ((this.wrapping_add(CACHE_WORD)) as *const u16).read_unaligned() as u32;
        }
        let key = lf_checker_rt::callee_thiscall!(ID_CALLEE, u32, this, 1);
        lf_checker_rt::callee_thiscall!(RESOLVE_CALLEE, u32, lf_checker_rt::relocated(MANAGER), key)
    }
});
