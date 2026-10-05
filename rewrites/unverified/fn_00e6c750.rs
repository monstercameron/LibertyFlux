// original: 0x00e6c750 veh_float_copy_50 (proposed)

/// Copy one live float global to another: `DST = SRC`.
///
/// Two SSE moves with no arguments and no return value. The value is carried
/// bit-exact (a NaN keeps its payload) because it is never computed on, only
/// moved; the rewrite moves the same 32 bits.
///
/// Original: 0x00e6c750 (cdecl, no arguments, no return channel).
lf_checker_rt::export!(cdecl, rw_00e6c750() -> u32 {
    unsafe {
        const SRC: u32 = 0x01050b4c;
        const DST: u32 = 0x0171de54;
        let v = (lf_checker_rt::global::<u32>(SRC)).read();
        (lf_checker_rt::global::<u32>(DST)).write(v);
        0
    }
});
