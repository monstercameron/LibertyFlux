// original: 0x008ED030 table_range_scan (proposed)

/// Scan the 64 resident tables for a row whose scaled value matches the
/// query point, returning 1 on the first match and 0 otherwise.
///
/// `this` is the owner object: 64 table pointers at `+0x804` (a null entry
/// is skipped). `vec` points to three floats (x, y, z). For each resident
/// table `i` two level values are fetched from the level helper (callee 1
/// with `i mod 8`, callee 3 with `i / 8`; both quotients are SIGNED, though
/// `i` itself is never negative), and the table is skipped unless both
/// levels overlap the query window: `f0 < x + PAD` with `f0 + SPAN > x -
/// PAD`, and the same for `f1` against y (`PAD` is 50, `SPAN` is 750).
/// Every gate is a strict ordered `comiss` greater-than: an unordered
/// (NaN) comparison skips the table.
///
/// A surviving table is range-queried through the range helper (callee 2,
/// thiscall: index, y-window low/high bits, two out-pointers), which
/// reports a half-open row range `[lo, hi)`. The bounds are compared
/// SIGNED (`cmp`+`jge`/`jl`): a negative `lo` scans rows before the table.
/// Each row is 32 bytes; a row matches when its tag byte at `+0x1e` is
/// non-negative with a non-zero low nibble, its flag byte at `+0x1c` has a
/// zero high nibble, and `|raw * SCALE - z| < TOL` where `raw` is the
/// signed 16-bit value at `+0x18` (`SCALE` is 0.015625, `TOL` is 10).
///
/// Original: 0x008ED030 (thiscall, one stack word, low byte of the return
/// compared; all float arithmetic in the original's operand order).
lf_checker_rt::export!(thiscall, rw_008ED030(this: u32, vec: u32) -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x804;
        const TABLE_LEN: u32 = 0x40;
        const ENTRY_STRIDE: u32 = 0x20;
        const ENTRY_VALUE: u32 = 0x18;
        const ENTRY_FLAGS: u32 = 0x1c;
        const ENTRY_TAG: u32 = 0x1e;
        const CALLEE_LEVEL: u32 = 1;
        const CALLEE_LEVEL2: u32 = 3;
        const CALLEE_RANGE: u32 = 2;

        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let pad = *lf_checker_rt::global::<f32>(0xfe8b68);
        let span = *lf_checker_rt::global::<f32>(0xe83174);
        let tol = *lf_checker_rt::global::<f32>(0xfe8b08);
        let scale = *lf_checker_rt::global::<f32>(0xfe8720);
        let x = (vec as *const f32).read_unaligned();
        let y = ((vec + 4) as *const f32).read_unaligned();
        let z = ((vec + 8) as *const f32).read_unaligned();
        let xlo = sub(x, pad);
        let xhi = add(x, pad);
        let ylo = sub(y, pad);
        let yhi = add(y, pad);

        let mut i = 0u32;
        while i < TABLE_LEN {
            let table = rd32(this.wrapping_add(TABLE_BASE).wrapping_add(i * 4));
            if table != 0 {
                let f0: f32 = lf_checker_rt::callee_stdcall!(
                    CALLEE_LEVEL,
                    f32,
                    (i as i32 % 8) as u32
                );
                let f1: f32 = lf_checker_rt::callee_stdcall!(
                    CALLEE_LEVEL2,
                    f32,
                    (i as i32 / 8) as u32
                );
                // Each gate is `!(a > b)` so an unordered (NaN) compare
                // skips, exactly like the original's `comiss`+`jbe`.
                if !(xhi > f0) {
                    i += 1;
                    continue;
                }
                if !(add(f0, span) > xlo) {
                    i += 1;
                    continue;
                }
                if !(yhi > f1) {
                    i += 1;
                    continue;
                }
                if !(add(f1, span) > ylo) {
                    i += 1;
                    continue;
                }
                let (mut lo, mut hi) = (0u32, 0u32);
                lf_checker_rt::callee_thiscall!(
                    CALLEE_RANGE,
                    u32,
                    this,
                    i,
                    ylo.to_bits(),
                    yhi.to_bits(),
                    &mut lo as *mut u32 as u32,
                    &mut hi as *mut u32 as u32
                );
                // SIGNED bounds: a negative `lo` scans before the table.
                let (lo, hi) = (lo as i32, hi as i32);
                if lo < hi {
                    let mut k = lo;
                    while k < hi {
                        let addr = table
                            .wrapping_add((k as u32).wrapping_shl(5))
                            .wrapping_add(ENTRY_TAG);
                        let tag = (addr as *const u8).read();
                        if (tag as i8) >= 0
                            && ((addr.wrapping_sub(
                                ENTRY_TAG.wrapping_sub(ENTRY_FLAGS),
                            ))
                                as *const u8)
                                .read()
                                & 0xf0
                                == 0
                            && tag & 0x0f != 0
                        {
                            let raw = (((addr.wrapping_sub(
                                ENTRY_TAG.wrapping_sub(ENTRY_VALUE),
                            ))
                                as *const u16)
                                .read_unaligned()
                                as i16) as f32;
                            let dist = f32::from_bits(
                                sub(mul(raw, scale), z).to_bits() & 0x7fff_ffff,
                            );
                            if tol > dist {
                                return 1;
                            }
                        }
                        k = k.wrapping_add(1);
                    }
                }
            }
            i += 1;
        }
        0
    }
});
