// original: 0x00b9d260 NativeImpl_SWITCH_PED_PATHS_ON

/// Switches ped paths on in an area given by two corners.
///
/// Identical to its twin `0x00B9D1C0` except for the final callee:
/// normalizes each of the three coordinate pairs with `comiss`/`jbe`
/// semantics (swap only when strictly greater), packs an 8-word frame
/// row as [maxx, maxy, maxz, scratch, minx, miny, minz, scratch] (scratch
/// words are zero under the contract's `stack_fill`), sets bit 0 of the
/// flag word at offset `FLAG_OFF` of the singleton from `GET`, reads a
/// word at offset `W_OFF` from it, and passes the min half (row[4..8])
/// then the full row (row[0..8]) plus that word through the 3-argument
/// `APPLY` call. Returns whatever `APPLY` returns.
///
/// Both view pointers are skipped call arguments with snapshot-verified
/// contents (see `narrowed`).
///
/// Original: 0x00B9D260 (cdecl, six stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9d260(x0: u32, y0: u32, z0: u32, x1: u32, y1: u32, z1: u32) -> u32 {
    const GET: u32 = 1;
    const APPLY: u32 = 2;
    const FLAG_OFF: u32 = 0xAC;
    const W_OFF: u32 = 4;
    #[inline(always)]
    fn norm(mut lo: f32, mut hi: f32) -> (u32, u32) {
        if lo > hi {
            core::mem::swap(&mut lo, &mut hi);
        }
        (lo.to_bits(), hi.to_bits())
    }
    unsafe {
        let (minx, maxx) = norm(f32::from_bits(x0), f32::from_bits(x1));
        let (miny, maxy) = norm(f32::from_bits(y0), f32::from_bits(y1));
        let (minz, maxz) = norm(f32::from_bits(z0), f32::from_bits(z1));
        let mut row = [maxx, maxy, maxz, 0, minx, miny, minz, 0];
        let p1: u32 = lf_checker_rt::callee_cdecl!(GET, u32,);
        let flag = (p1.wrapping_add(FLAG_OFF) as *mut u32);
        flag.write_unaligned(flag.read_unaligned() | 1);
        let p2: u32 = lf_checker_rt::callee_cdecl!(GET, u32,);
        let w = (p2.wrapping_add(W_OFF) as *const u32).read_unaligned();
        let base = row.as_mut_ptr();
        lf_checker_rt::callee_cdecl!(APPLY, u32, base.add(4) as u32, base as u32, w)
    }
});
