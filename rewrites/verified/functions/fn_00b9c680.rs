// original: 0x00b9c680 NativeImpl_GET_PARKING_NODE_IN_AREA

/// Gets a parking node in an area given by two corners, into three out-slots.
///
/// Normalizes each of the three coordinate pairs (`x0`/`x1`, `y0`/`y1`,
/// `z0`/`z1`) with `comiss`/`jbe` semantics (swap only when strictly
/// greater, so NaN and equal pairs stay), packs the six bounds as
/// [minx, maxx, miny, maxy, minz, maxz] in a frame row, and passes a
/// pointer one past its end through the single `GET` call on the global
/// object `OBJ`. The callee's return is a pointer to a float triple,
/// copied through `o0`, `o1`, `o2` in order; any null out-pointer faults
/// on its store, like the original. Returns `o2` unchanged.
///
/// The end-pointer is a skipped call argument with a 6-word snapshot at
/// offset -24 (see `narrowed`); the pointed-to word itself is not
/// observed.
///
/// Original: 0x00B9C680 (cdecl, nine stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9c680(
    x0: u32, y0: u32, z0: u32, x1: u32, y1: u32, z1: u32, o0: u32, o1: u32, o2: u32,
) -> u32 {
    const OBJ: u32 = 0x01177A80;
    const GET: u32 = 1;
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
        let mut cells = [minx, maxx, miny, maxy, minz, maxz];
        let p: u32 = lf_checker_rt::callee_thiscall!(
            GET,
            u32,
            lf_checker_rt::relocated(OBJ),
            cells.as_mut_ptr().add(6) as u32
        );
        let f0 = (p as *const u32).read_unaligned();
        let f1 = (p.wrapping_add(4) as *const u32).read_unaligned();
        let f2 = (p.wrapping_add(8) as *const u32).read_unaligned();
        (o0 as *mut u32).write_unaligned(f0);
        (o1 as *mut u32).write_unaligned(f1);
        (o2 as *mut u32).write_unaligned(f2);
        o2
    }
});
