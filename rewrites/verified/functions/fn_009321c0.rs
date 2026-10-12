// original: 0x009321C0 imposter_cache_slot_value

/// Read the value word from one of sixteen fixed cache records. `index` is
/// scaled by the 64-byte record stride, and the returned word is loaded from
/// the relocated cache table. The calling convention is cdecl and the result
/// is returned in EAX.
lf_checker_rt::export!(cdecl, rw_009321C0(index: u32) -> u32 {
    const CACHE_VALUES_VA: u32 = 0x011A_2AA0;
    const RECORD_STRIDE: u32 = 64;

    let address = lf_checker_rt::relocated(CACHE_VALUES_VA)
        .wrapping_add(index.wrapping_mul(RECORD_STRIDE));
    unsafe { (address as *const u32).read_unaligned() }
});
