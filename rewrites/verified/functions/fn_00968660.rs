// original: 0x00968660 timing_grid_direction (proposed)

/// Look up one cell of a 120-wide byte grid and write a normalised
/// 3-vector plus the cell's scaled coordinates into an output struct.
///
/// `a1` and `a2` are grid coordinates, `out` points to 0x18 bytes. When
/// either coordinate is 0x78 or more (unsigned), the vector part is the
/// constant (1, 0, 0) and the coordinate part is left untouched; otherwise
/// the cell index `a1 + a2 * 120` addresses a byte table holding a flag bit
/// (bit 7) and a 7-bit magnitude per cell (odd bytes; the even byte beside
/// it is a signed neighbour value used only when the cell byte is exactly
/// 0x80).
///
/// In-bounds output: the coordinates scaled as `(float(a) - 60) * 50` at
/// `+0x10`/`+0x14` (converted through doubles with an unsigned-widening
/// table whose zero entry is the only one ever addressed); the vector
/// `(0, 1 - m, m)` at `+0x00`/`+0x04`/`+0x08` where `m` is the magnitude
/// capped at 13 and divided by 13 — except a cell of exactly 0x80 with a
/// neighbour below -86 takes the constant (1, 0, 0) instead. The vector is
/// then scaled to unit length (`k = 1 / sqrt(y*y + x*x + z*z)`, in that
/// association order). All float operations keep the original's operand
/// order. The return value is dead scratch (one path even returns the
/// incoming accumulator) and is not compared.
///
/// Proof note: the original keeps the cell flag bit in a stack temp that
/// overlaps the last byte of its dead first-argument slot, so the stack
/// check is off; the flag computation itself selects the compared `out[0]`
/// value and is verified.
///
/// Original: stdcall, three stack words.
lf_checker_rt::export!(stdcall, rw_00968660(a1: u32, a2: u32, out: u32) -> u32 {
    unsafe {
        const GRID_WIDE: u32 = 120;
        const GRID_LIM: u32 = 0x78;
        const TABLE_BASE: u32 = 0x0121_8558;
        const WIDE_TAB: u32 = 0x00fe_8f50;
        const CAP: f32 = 13.0;
        const SUB_C: f32 = 60.0;
        const MUL_C: f32 = 50.0;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn normalize(out: u32) {
            unsafe {
                let x = rdf(out.wrapping_add(4));
                let y = rdf(out);
                let z = rdf(out.wrapping_add(8));
                let n2 = add(add(mul(y, y), mul(x, x)), mul(z, z));
                let root = n2.sqrt();
                let one = rdf(lf_checker_rt::relocated(0x00fe_88e8));
                let k = core::hint::black_box(one) / core::hint::black_box(root);
                wrf(out, mul(k, y));
                wrf(out.wrapping_add(4), mul(k, x));
                wrf(out.wrapping_add(8), mul(k, z));
            }
        }

        let table = lf_checker_rt::relocated(TABLE_BASE);
        if a1 < GRID_LIM && a2 < GRID_LIM {
            let cell = a1.wrapping_add(a2.wrapping_mul(GRID_WIDE));
            let wide = lf_checker_rt::relocated(WIDE_TAB);
            let mut c = 0u32;
            while c < 2 {
                let a = if c == 0 { a1 } else { a2 };
                let adj = f64::from_bits(unsafe {
                    ((wide.wrapping_add((a >> 31).wrapping_mul(8))) as *const u64).read_unaligned()
                });
                let d = (a as i32) as f64 + adj;
                wrf(out.wrapping_add(0x10).wrapping_add(c * 4), mul(sub(d as f32, SUB_C), MUL_C));
                c += 1;
            }
            let b = rd8(table.wrapping_add(cell.wrapping_mul(2)).wrapping_add(1));
            let mag = (b & 0x7f) as f32;
            let capped = if mag > CAP { CAP } else { mag };
            let m = mul(capped, rdf(lf_checker_rt::relocated(0x00fe_8788)));
            let one = rdf(lf_checker_rt::relocated(0x00fe_88e8));
            wrf(out.wrapping_add(8), m);
            wrf(out.wrapping_add(4), sub(one, m));
            if (b >> 7) == 0 || (b & 0x7f) != 0 {
                wrf(out, 0.0);
            } else {
                let n = rd8(table.wrapping_add(cell.wrapping_mul(2))) as i8 as i32;
                if n.wrapping_add(0x60) < 10 {
                    wrf(out.wrapping_add(8), 0.0);
                    wrf(out.wrapping_add(4), 0.0);
                    wrf(out, 1.0);
                } else {
                    wrf(out, 0.0);
                }
            }
        } else {
            wrf(out.wrapping_add(8), 0.0);
            wrf(out.wrapping_add(4), 0.0);
            wrf(out, 1.0);
        }
        normalize(out);
        0
    }
});
