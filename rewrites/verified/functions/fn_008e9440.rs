// original: 0x008e9440 scan_slots_best_scored_pair (proposed)

/// Scan all 64 slots for the best-scoring record/entry pair under a weighted
/// Manhattan distance, then report the pair and the direction between the two
/// winning points as a compass angle.
///
/// `handle` owns three tables indexed by `slot * 4`: record-array bases at
/// `+0x804`, id-array bases at `+0x904`, and SIGNED record counts at `+0xb04`.
/// Each record is 32 bytes (first one at base `+0x14`): three signed words of
/// packed coordinates at `+0x0/+0x2/+0x4` scaled by 0.125, 0.125 and 0.015625,
/// a flag byte at `+0xa` (bit 0x80 filtered when `use_flag` is set, low
/// nibble is the id count, SIGNED), a selector byte at `+0xb` (bit 1 must
/// equal `want_bit`), a signed word at `-0x2` seeding the id-array cursor,
/// and a dword at `-0xc` passed to the grade callee. Each id-array entry is
/// 8 bytes holding one packed id; its low 16 bits name a slot whose record
/// array holds the sub-point, its high 16 bits the sub-point index (32-byte
/// stride, coordinates at `+0x14/+0x16/+0x18`, same scales and flag bytes at
/// `+0x1e/+0x1f`).
///
/// A record scores `|y-py| + |x-px| + |z-pz| * 3.0` against `point` (in that
/// operation order) and is considered only while strictly below the running
/// best (seeded at 10000.0; NaN never wins). Each surviving id calls the
/// grade callee (callee 1, stdcall of the record dword then the id) whose
/// answer indexes a grade row from the global table: the grade byte's two
/// 3-bit fields must sum to at least `grade_bound` (SIGNED compare), and the
/// sub-point must lie strictly farther than `dist_thresh` from the record
/// point. The winner's pair of packed ids goes to `*out_id0`/`*out_id1`,
/// id order swapped when the sub-point sorts after the record point
/// (lexicographic on (y, x) with ordered float compares), the grade fields
/// to `*out_q0`/`*out_q1` when non-null (same swap), and a grade nibble as a
/// float to `*out_aux` when non-null. The reported angle is the grade-1
/// callee-2 (f64 atan2 over the normalized, x-negated 2-D delta) converted
/// to degrees; a zero-length delta yields angle 0 through a 1/sqrt factor of
/// 0, NaN propagates. The whole report is written only when `win_thresh` is
/// strictly above the final best (ordered); otherwise all three outputs are
/// -1/-1/0.0. Returns the last output pointer read (`out_aux` on a win,
/// `out_angle` on a loss), which is what the original leaves in eax.
///
/// Original: 0x008e9440 (thiscall, twelve stack words; all float arithmetic
/// in the original's operand order, pinned against commuting).
lf_checker_rt::export!(thiscall, rw_008e9440(
    handle: u32,
    point: u32,
    out_id0: u32,
    out_id1: u32,
    out_angle: u32,
    dist_thresh: u32,
    win_thresh: u32,
    use_flag: u32,
    want_bit: u32,
    grade_bound: u32,
    out_q0: u32,
    out_q1: u32,
    out_aux: u32,
) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x804;
        const N_SLOTS: i32 = 0x40;
        const REC_STRIDE: u32 = 0x20;
        const REC_FIRST: u32 = 0x14;
        const SUB_STRIDE: u32 = 32;
        const ABS_MASK: u32 = 0x7fff_ffff;
        const CALLEE_GRADE: u32 = 1;
        const CALLEE_ATAN2: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16i(a: u32) -> i32 {
            unsafe { (a as *const i16).read_unaligned() as i32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn imgf(va: u32) -> f32 {
            unsafe { lf_checker_rt::global::<f32>(va).read() }
        }
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn fabsf(v: f32) -> f32 {
            f32::from_bits(v.to_bits() & ABS_MASK)
        }

        let xy_scale = imgf(0x00FE87A4);
        let z_scale = imgf(0x00FE8720);
        let zw = imgf(0x00FE8A94);
        let mut best = imgf(0x00FE8CB0);
        let deg = imgf(0x00E7C2A8);
        let gbase = lf_checker_rt::relocated(0x01177B80);
        let px = f32::from_bits(rd32(point));
        let py = f32::from_bits(rd32(point.wrapping_add(4)));
        let pz = f32::from_bits(rd32(point.wrapping_add(8)));
        let thresh = f32::from_bits(dist_thresh);
        let win = f32::from_bits(win_thresh);
        let bound = grade_bound as i32;
        let want = (want_bit as u8) & 1;
        let filtered = (use_flag as u8) != 0;

        let mut p0: u32 = 0xffff_ffff;
        let mut p1: u32 = 0xffff_ffff;
        let mut q0: u32 = 0;
        let mut q1: u32 = 0;
        let mut aux: f32 = 0.0;

        let mut outer: i32 = 0;
        while outer < N_SLOTS {
            let slot = handle.wrapping_add(SLOTS).wrapping_add((outer as u32).wrapping_mul(4));
            let arec = rd32(slot);
            if arec != 0 {
                let count = rd32(slot.wrapping_add(0x300)) as i32;
                if count > 0 {
                    let mut i: i32 = 0;
                    while i < count {
                        let rec = arec.wrapping_add(REC_FIRST).wrapping_add((i as u32).wrapping_mul(REC_STRIDE));
                        let mut accept = true;
                        if filtered && rd8(rec.wrapping_add(0xa)) & 0x80 != 0 {
                            accept = false;
                        }
                        if accept && ((rd8(rec.wrapping_add(0xb)) >> 1) & 1) != want {
                            accept = false;
                        }
                        if accept {
                            let x = mul(rd16i(rec) as f32, xy_scale);
                            let y = mul(rd16i(rec.wrapping_add(2)) as f32, xy_scale);
                            let z = mul(rd16i(rec.wrapping_add(4)) as f32, z_scale);
                            let s = add(fabsf(sub(y, py)), fabsf(sub(x, px)));
                            let score = add(s, mul(fabsf(sub(z, pz)), zw));
                            if best > score {
                                let n = (rd8(rec.wrapping_add(0xa)) & 0xf) as i32;
                                if n > 0 {
                                    let cx = rd16i(rec.wrapping_sub(2));
                                    let mut p = rd32(slot.wrapping_add(0x100))
                                        .wrapping_add((cx.wrapping_mul(8)) as u32);
                                    let mut k = n;
                                    while k != 0 {
                                        k -= 1;
                                        let entry = rd32(p);
                                        p = p.wrapping_add(8);
                                        let eobj = rd32(
                                            handle.wrapping_add(SLOTS).wrapping_add(
                                                (entry as u16 as u32).wrapping_mul(4),
                                            ),
                                        );
                                        if eobj == 0 {
                                            continue;
                                        }
                                        let sub = eobj.wrapping_add((entry >> 16).wrapping_mul(SUB_STRIDE));
                                        if filtered && rd8(sub.wrapping_add(0x1e)) & 0x80 != 0 {
                                            continue;
                                        }
                                        if ((rd8(sub.wrapping_add(0x1f)) >> 1) & 1) != want {
                                            continue;
                                        }
                                        let rec_field = rd32(rec.wrapping_sub(0xc));
                                        let ans: u32 = lf_checker_rt::callee_stdcall!(CALLEE_GRADE, u32, rec_field, entry);
                                        let gptr = rd32(gbase.wrapping_add(0x804).wrapping_add((outer as u32).wrapping_mul(4)));
                                        let gb = rd8(gptr.wrapping_add(ans.wrapping_mul(8)).wrapping_add(5));
                                        let low3 = (gb & 7) as u32;
                                        let mid3 = ((gb >> 3) & 7) as u32;
                                        if ((low3.wrapping_add(mid3)) as i32) < bound {
                                            continue;
                                        }
                                        let dx = mul(rd16i(sub.wrapping_add(0x14)) as f32, xy_scale);
                                        let dy = mul(rd16i(sub.wrapping_add(0x16)) as f32, xy_scale);
                                        let dz = mul(rd16i(sub.wrapping_add(0x18)) as f32, z_scale);
                                        let ry = sub(y, dy);
                                        let rx = sub(x, dx);
                                        let rz = sub(z, dz);
                                        let q = add(add(mul(ry, ry), mul(rx, rx)), mul(rz, rz));
                                        let d = core::hint::black_box(q).sqrt();
                                        if !(d > thresh) {
                                            continue;
                                        }
                                        best = score;
                                        let first = ((i as u32) << 16) | (outer as u32);
                                        p0 = first;
                                        p1 = entry;
                                        q0 = low3;
                                        q1 = mid3;
                                        let nib = rd8(gptr.wrapping_add(ans.wrapping_mul(8)).wrapping_add(6)) & 0xf;
                                        aux = (nib as u32) as f32;
                                        if dy > y || (y == dy && x > dx) {
                                            p0 = entry;
                                            p1 = first;
                                            q0 = mid3;
                                            q1 = low3;
                                        }
                                    }
                                }
                            }
                        }
                        i += 1;
                    }
                }
            }
            outer += 1;
        }

        if !(win > best) {
            wr32(out_id0, 0xffff_ffff);
            wr32(out_id1, 0xffff_ffff);
            wr32(out_angle, 0);
            return out_angle;
        }
        wr32(out_id0, p0);
        wr32(out_id1, p1);
        let r0 = rd32(handle.wrapping_add(SLOTS).wrapping_add((p0 as u16 as u32).wrapping_mul(4)))
            .wrapping_add((p0 >> 16).wrapping_mul(SUB_STRIDE));
        let r1 = rd32(handle.wrapping_add(SLOTS).wrapping_add((p1 as u16 as u32).wrapping_mul(4)))
            .wrapping_add((p1 >> 16).wrapping_mul(SUB_STRIDE));
        let ax = mul(rd16i(r0.wrapping_add(0x14)) as f32, xy_scale);
        let ay = mul(rd16i(r0.wrapping_add(0x16)) as f32, xy_scale);
        let bx = mul(rd16i(r1.wrapping_add(0x14)) as f32, xy_scale);
        let by = mul(rd16i(r1.wrapping_add(0x16)) as f32, xy_scale);
        let ex = sub(ax, bx);
        let ey = sub(ay, by);
        let len2 = add(mul(ex, ex), mul(ey, ey));
        // Zero-length delta takes a factor of 0 (the original's ucomiss
        // against +0.0 with a lahf/test/jp shape: equal means zero, above,
        // below or unordered takes the reciprocal-root path, so NaN flows
        // through the division).
        let factor = if len2 == 0.0 {
            0.0f32
        } else {
            div(1.0, core::hint::black_box(len2).sqrt())
        };
        let fx = mul(factor, ex);
        let fy = mul(factor, ey);
        let nx = -fx;
        let dx_bits = (nx as f64).to_bits();
        let dy_bits = (fy as f64).to_bits();
        let ans_bits: u64 = lf_checker_rt::callee_cdecl!(
            CALLEE_ATAN2,
            u64,
            (dx_bits & 0xffff_ffff) as u32,
            (dx_bits >> 32) as u32,
            (dy_bits & 0xffff_ffff) as u32,
            (dy_bits >> 32) as u32
        );
        let ang = mul(f64::from_bits(ans_bits) as f32, deg);
        wr32(out_angle, ang.to_bits());
        if out_q0 != 0 {
            wr32(out_q0, q0);
        }
        if out_q1 != 0 {
            wr32(out_q1, q1);
        }
        if out_aux != 0 {
            wr32(out_aux, aux.to_bits());
        }
        out_aux
    }
});
