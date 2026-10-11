// original: 0x00DB1520 mainloop_timing_crc32_static_key
/// Hashes the fixed static key through the shared CRC helper. The function
/// passes the relocated key address as one cdecl argument and returns the
/// helper's full 32-bit EAX result unchanged.
lf_checker_rt::export!(cdecl, rw_00DB1520() -> u32 {
    const STATIC_KEY_VA: u32 = 0x00EF_2008;
    let static_key = lf_checker_rt::relocated(STATIC_KEY_VA);
    lf_checker_rt::callee_cdecl!(1, u32, static_key)
});
