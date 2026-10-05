// original: 0x00b96950 NativeImpl_IS_BULLET_IN_AREA_2

/// Tests whether a bullet is inside an area box, optionally primed.
///
/// When the flag byte is nonzero, primes through `PRIME` first. Packs
/// (`x`, `y`, `z`) into a frame buffer and tests it with (`w`, 0) through
/// `TEST`. No value is returned.
///
/// The buffer pointer is a skipped call argument with snapshot-verified
/// contents (see `narrowed`).
///
/// Original: 0x00B96950 (cdecl, five stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b96950(x: u32, y: u32, z: u32, w: u32, flag: u32) -> u32 {
    const PRIME: u32 = 1;
    const TEST: u32 = 2;
    if flag & 0xFF != 0 {
        let _: u32 = lf_checker_rt::callee_cdecl!(PRIME, u32,);
    }
    let mut buf = [x, y, z];
    let _: u32 = lf_checker_rt::callee_cdecl!(TEST, u32, buf.as_mut_ptr() as u32, w, 0);
    0
});
