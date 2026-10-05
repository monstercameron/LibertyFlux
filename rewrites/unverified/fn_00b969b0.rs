// original: 0x00b969b0 NativeImpl_IS_BULLET_IN_BOX_2

/// Tests whether a bullet is inside a normalized box, optionally primed.
///
/// When the flag byte is nonzero, primes through `PRIME` first. Normalizes
/// each of the three corner pairs with `comiss`/`jbe` semantics (no swap
/// when unordered, so NaN keeps its place): after the swap each first
/// component is the minimum. Packs the minima and maxima into two frame
/// triples and tests them with 0 through `TEST`. No value is returned.
///
/// Both buffer pointers are skipped call arguments with snapshot-verified
/// contents (see `narrowed`).
///
/// Original: 0x00B969B0 (cdecl, seven stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b969b0(x0: u32, y0: u32, z0: u32, x1: u32, y1: u32, z1: u32, flag: u32) -> u32 {
    const PRIME: u32 = 1;
    const TEST: u32 = 2;
    if flag & 0xFF != 0 {
        let _: u32 = lf_checker_rt::callee_cdecl!(PRIME, u32,);
    }
    let (mut x0f, mut x1f) = (f32::from_bits(x0), f32::from_bits(x1));
    if x0f > x1f {
        core::mem::swap(&mut x0f, &mut x1f);
    }
    let (mut y0f, mut y1f) = (f32::from_bits(y0), f32::from_bits(y1));
    if y0f > y1f {
        core::mem::swap(&mut y0f, &mut y1f);
    }
    let (mut z0f, mut z1f) = (f32::from_bits(z0), f32::from_bits(z1));
    if z0f > z1f {
        core::mem::swap(&mut z0f, &mut z1f);
    }
    let mut mins = [x0f.to_bits(), y0f.to_bits(), z0f.to_bits()];
    let mut maxs = [x1f.to_bits(), y1f.to_bits(), z1f.to_bits()];
    let _: u32 = lf_checker_rt::callee_cdecl!(
        TEST, u32, mins.as_mut_ptr() as u32, maxs.as_mut_ptr() as u32, 0
    );
    0
});
