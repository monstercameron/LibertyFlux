// original: 0x008A7BA0 audio_nearest_slot_distance (proposed)

/// Minimum scaled range over up to five audio slots, square-rooted.
///
/// `this` points to the audio manager object, `src` to three floats (x, y,
/// z). The thread's audio state index `edx` is read from the TLS slot whose
/// number the global `TLS_INDEX` holds, field `TLS_THREAD_FIELD` of the
/// thread block. Each gate `i` (0..5) is enabled by a nonzero dword at
/// `this + edx*4 + GATE_BASE + i*GATE_STRIDE`.
///
/// An enabled gate transforms (x, y, z) through a 3x4 row at
/// `this + ((edx + ROW_BASE + i*4) << ROW_SHIFT)` (three scaled rows plus a
/// translation at +0x30/+0x34/+0x38), scales the vector by `this[SCALE_OFF]`
/// and forms the squared length `d`. Gate 0 sets the best to `d`
/// unconditionally; later gates take `d` when the running best is negative
/// or NaN, then again when `d - best` is negative or NaN (both comparisons
/// are ordered, matching the original's comiss/jae pair). With no gate
/// enabled the best stays at the constant from `BEST_INIT` (-1.0). The
/// result is `sqrt(best)`, returned in ST0 (thiscall, one stack word).
///
/// Float operation order is the original's, pinned with black_box; the
/// square root is the scalar instruction, matching the low lane of the
/// original's packed one.
lf_checker_rt::export!(thiscall, rw_008A7BA0(this: u32, src: u32) -> f32 {
    unsafe {
        const TLS_INDEX_GLOBAL: u32 = 0x017ABA14;
        const TLS_THREAD_FIELD: u32 = 0x70;
        const GATE_BASE: u32 = 0x1580;
        const GATE_STRIDE: u32 = 0x10;
        const ROW_BASE: u32 = 0x28;
        const ROW_SHIFT: u32 = 6;
        const SCALE_OFF: u32 = 0x1714;
        const BEST_INIT_GLOBAL: u32 = 0x00FE8D94;
        const N_GATES: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Squared scaled length for one gate's row, in the original's order:
        /// vx/vy/vz accumulate (row-pair products, then translation), the
        /// scale multiplies (vx*s, s*vy, s*vz), and d sums the squares as
        /// ((s*vy)^2 + (vx*s)^2) + (s*vz)^2.
        unsafe fn candidate(row: u32, x: f32, y: f32, z: f32, s: f32) -> f32 {
            unsafe {
                let vx = add(
                    add(add(mul(rdf(row + 0x10), y), mul(rdf(row), x)), mul(rdf(row + 0x20), z)),
                    rdf(row + 0x30),
                );
                let vy = add(
                    add(add(mul(rdf(row + 0x14), y), mul(rdf(row + 0x04), x)), mul(rdf(row + 0x24), z)),
                    rdf(row + 0x34),
                );
                let vz = add(
                    add(add(mul(rdf(row + 0x18), y), mul(rdf(row + 0x08), x)), mul(rdf(row + 0x28), z)),
                    rdf(row + 0x38),
                );
                let ax = mul(vx, s);
                let ay = mul(s, vy);
                let az = mul(s, vz);
                add(add(mul(ay, ay), mul(ax, ax)), mul(az, az))
            }
        }

        let slot = rd32(lf_checker_rt::relocated(TLS_INDEX_GLOBAL));
        let thread = lf_checker_rt::tls_slot(slot as usize);
        let edx = rd32(thread.wrapping_add(TLS_THREAD_FIELD));
        let x = rdf(src);
        let y = rdf(src.wrapping_add(4));
        let z = rdf(src.wrapping_add(8));
        let s = rdf(this.wrapping_add(SCALE_OFF));
        let mut best = f32::from_bits(rd32(lf_checker_rt::relocated(BEST_INIT_GLOBAL)));
        for i in 0..N_GATES {
            let gate = this
                .wrapping_add(edx.wrapping_mul(4))
                .wrapping_add(GATE_BASE + i.wrapping_mul(GATE_STRIDE));
            if rd32(gate) == 0 {
                continue;
            }
            let row = this.wrapping_add(edx.wrapping_add(ROW_BASE + i.wrapping_mul(4)).wrapping_shl(ROW_SHIFT));
            let d = candidate(row, x, y, z, s);
            if i == 0 {
                best = d;
                continue;
            }
            if !(best >= 0.0) {
                best = d;
            }
            let diff = sub(d, best);
            if !(diff >= 0.0) {
                best = d;
            }
        }
        best.sqrt()
    }
});
