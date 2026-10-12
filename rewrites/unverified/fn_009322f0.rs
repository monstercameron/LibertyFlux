// original: 0x009322F0 tree_imposter_cache_size

/// Return the cache-size class selected by the global mode. Modes zero and
/// one select 128 bytes, modes two and three select 256 bytes, and every
/// larger value selects 512 bytes. The dispatch table is local to this
/// routine; the mode word is read from relocated writable data.
lf_checker_rt::export!(cdecl, rw_009322F0() -> u32 {
    const CACHE_MODE_VA: u32 = 0x0116_0EAC;
    const SIZE_SMALL: u32 = 0x80;
    const SIZE_MEDIUM: u32 = 0x100;
    const SIZE_LARGE: u32 = 0x200;

    let mode = unsafe { lf_checker_rt::global::<u32>(CACHE_MODE_VA).read_unaligned() };
    match mode {
        0 | 1 => SIZE_SMALL,
        2 | 3 => SIZE_MEDIUM,
        _ => SIZE_LARGE,
    }
});
