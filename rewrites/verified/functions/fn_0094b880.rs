// original: 0x0094B880 lazy_singleton_get (proposed)

/// Return the cached singleton, creating and initialising it on first use.
///
/// Reads the cache word `CACHE`. A non-zero value is returned as is. When
/// zero, allocates through callee 1 (one stack word, the constant tag
/// `ALLOC_TAG`): a null answer clears the cache and returns null, otherwise
/// the fresh block is initialised through callee 2 (thiscall, the block in
/// ECX), its answer is stored into the cache and returned.
///
/// Original: 0x0094B880 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_0094B880() -> u32 {
    unsafe {
        const CACHE: u32 = 0x167E3B4;
        const ALLOC_TAG: u32 = 0x20020;
        const ALLOC: u32 = 1;
        const INIT: u32 = 2;
        let cached = (lf_checker_rt::global::<u32>(CACHE) as *const u32).read();
        if cached != 0 {
            return cached;
        }
        let fresh = lf_checker_rt::callee_cdecl!(ALLOC, u32, ALLOC_TAG);
        if fresh == 0 {
            (lf_checker_rt::global::<u32>(CACHE)).write(0);
            0
        } else {
            let init = lf_checker_rt::callee_thiscall!(INIT, u32, fresh);
            (lf_checker_rt::global::<u32>(CACHE)).write(init);
            init
        }
    }
});
