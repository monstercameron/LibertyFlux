// original: 0x00e6c890 veh_float_copy_90 (proposed)

/// Copy one live float global to another: `DST = SRC`.
///
/// Two SSE moves with no arguments and no return value. The value is carried
/// bit-exact (a NaN keeps its payload) because it is never computed on, only
/// moved; the rewrite moves the same 32 bits.
///
/// Original: 0x00e6c890 (cdecl, no arguments, no return channel).
lf_checker_rt::export!(cdecl, rw_00e6c890() -> u32 {
    unsafe {
        const SRC: u32 = 0x00ed7cdc;
        const DST: u32 = 0x0171fac4;
        let v = (lf_checker_rt::global::<u32>(SRC)).read();
        (lf_checker_rt::global::<u32>(DST)).write(v);
        0
    }
});
