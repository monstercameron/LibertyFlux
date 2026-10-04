// original: 0x00da9c60 project_point_onto_segment
/// Project point `c` onto the segment `a`-`b` (using x/y) and write the clamped
/// interpolation of all three coordinates to `out`.
///
/// The interpolation factor is the least-squares projection clamped to
/// [0, 1]; a NaN factor (zero-length segment over a coincident point) is kept
/// as NaN and propagates into the output, matching the original's comiss/jbe
/// clamp exactly.
export!(cdecl, rw_00da9c60(a: u32, b: u32, c: u32, out: u32) -> () { unsafe {
    let ax = f32::from_bits(((a) as *const u32).read_unaligned());
    let ay = f32::from_bits(((a.wrapping_add(4)) as *const u32).read_unaligned());
    let az = f32::from_bits(((a.wrapping_add(8)) as *const u32).read_unaligned());
    let bx = f32::from_bits(((b) as *const u32).read_unaligned());
    let by = f32::from_bits(((b.wrapping_add(4)) as *const u32).read_unaligned());
    let bz = f32::from_bits(((b.wrapping_add(8)) as *const u32).read_unaligned());
    let cx = f32::from_bits(((c) as *const u32).read_unaligned());
    let cy = f32::from_bits(((c.wrapping_add(4)) as *const u32).read_unaligned());
    let dy = by - ay;
    let ey = cy - ay;
    let dx = cx - ax;
    let fx = bx - ax;
    // Ordered adds/muls: the original's scalar sequence is (ey*dy)+(dx*fx),
    // (dy*dy)+(fx*fx), (fx*t)+ax, ((by-ay)*t)+ay; a plain `+` would be
    // commuted by the compiler and change NaN payloads.
    let num = (core::hint::black_box((core::hint::black_box(ey) * core::hint::black_box(dy))) + core::hint::black_box((core::hint::black_box(dx) * core::hint::black_box(fx))));
    let den = (core::hint::black_box((core::hint::black_box(dy) * core::hint::black_box(dy))) + core::hint::black_box((core::hint::black_box(fx) * core::hint::black_box(fx))));
    let mut t = num / den;
    // Ordered comparisons only: NaN keeps its value on both checks.
    if t < 0.0 {
        t = 0.0;
    } else if t > 1.0 {
        t = 1.0;
    }
    ((out) as *mut u32).write_unaligned(((core::hint::black_box((core::hint::black_box(fx) * core::hint::black_box(t))) + core::hint::black_box(ax))).to_bits());
    ((out.wrapping_add(4)) as *mut u32).write_unaligned(((core::hint::black_box((core::hint::black_box(dy) * core::hint::black_box(t))) + core::hint::black_box(ay))).to_bits());
    ((out.wrapping_add(8)) as *mut u32).write_unaligned(((core::hint::black_box((core::hint::black_box(bz - az) * core::hint::black_box(t))) + core::hint::black_box(az))).to_bits());
    } });
