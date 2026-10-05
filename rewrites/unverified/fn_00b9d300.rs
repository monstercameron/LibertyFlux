// original: 0x00b9d300 NativeImpl_SWITCH_PED_ROADS_BACK_TO_ORIGINAL

/// Switches ped roads back to original in an area given by two corners.
///
/// Normalizes each of the three coordinate pairs with `comiss`/`jbe`
/// semantics (swap only when strictly greater), packs two frame rows of
/// [maxx, maxy, maxz, scratch] and [minx, miny, minz, scratch] (scratch
/// words are zero under the contract's `stack_fill`), and passes the min
/// row then the max row through the 2-argument `DO` call. Returns
/// whatever the callee returns.
///
/// Both row pointers are skipped call arguments with snapshot-verified
/// contents (see `narrowed`).
///
/// Original: 0x00B9D300 (cdecl, six stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9d300(x0: u32, y0: u32, z0: u32, x1: u32, y1: u32, z1: u32) -> u32 {
    const DO: u32 = 1;
    #[inline(always)]
    fn norm(mut lo: f32, mut hi: f32) -> (u32, u32) {
        if lo > hi {
            core::mem::swap(&mut lo, &mut hi);
        }
        (lo.to_bits(), hi.to_bits())
    }
    let (minx, maxx) = norm(f32::from_bits(x0), f32::from_bits(x1));
    let (miny, maxy) = norm(f32::from_bits(y0), f32::from_bits(y1));
    let (minz, maxz) = norm(f32::from_bits(z0), f32::from_bits(z1));
    let mut mins = [minx, miny, minz, 0];
    let mut maxs = [maxx, maxy, maxz, 0];
    lf_checker_rt::callee_cdecl!(
        DO,
        u32,
        mins.as_mut_ptr() as u32,
        maxs.as_mut_ptr() as u32
    )
});
