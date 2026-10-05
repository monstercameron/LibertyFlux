// original: 0x00B3EDC0 ped_task_search_best_position (proposed)

/// Search the eight compass directions around a position for the best task slot.
///
/// `center` points at three floats (x, y, z). `out` receives the winning
/// triple plus a fourth word. `radius` scales the search; `flags` selects the
/// mode; `arg4`/`arg5` are opaque values forwarded to the callees.
///
/// Behaviour: the flags word is stored to a global slot. When bit 8 of
/// `flags` is set, all six arguments are forwarded unchanged to a sibling
/// routine and its result is returned. Otherwise the rewrite builds eight
/// 2-D offsets from `radius` (the four axis directions plus the four
/// diagonals at 0.7071, z always zero), adds each to `center`, and resolves
/// every candidate (plus `center` itself) to an id through callee 3,
/// de-duplicating into at most nine ids. Each id is resolved through callee
/// 4 to an object pointer; nulls are skipped. For the rest, callee 5 (bit 3
/// of `flags` set) or callee 6 (clear) is invoked thiscall-style with the
/// center, the radius bits, a four-word scratch buffer, a code-pointer
/// constant, the flags and the opaque arguments (callee 6 takes one extra
/// leading `1`); the callee fills the scratch buffer. When it returns
/// non-null, the squared distance from the buffer's triple to `center` is
/// computed; if it beats the best so far (seeded with FLT_MAX), the buffer
/// is copied to `out` and the return becomes 1 plus bit 6 of the byte at
/// object+0x1c (so the result is 0, 1 or 2).
///
/// Float order is the original's: candidate = offset + center, distance =
/// (dy^2 + dx^2) + dz^2 with each difference = found - center. The sign flip
/// for the negative directions is a bit xor, exact for signed zero and NaN.
///
/// Original: 0x00B3EDC0 (cdecl, six stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00B3EDC0(center: u32, out: u32, radius_bits: u32, flags: u32, arg4: u32, arg5: u32) -> u32 {
    unsafe {
        const FWD_CALLEE: u32 = 1;
        const COOKIE_CALLEE: u32 = 2;
        const RESOLVE_CALLEE: u32 = 3;
        const OBJECT_CALLEE: u32 = 4;
        const PROBE_A_CALLEE: u32 = 5;
        const PROBE_B_CALLEE: u32 = 6;
        const FLAGS_GLOBAL: u32 = 0x1664680;
        const CODE_CONST: u32 = 0x00B3EC90;
        const DIAG: f32 = f32::from_bits(0x3F34_FDF4); // 0.70700002
        const NEG_DIAG: f32 = f32::from_bits(0xBF34_FDF4); // -0.70700002
        const INIT_BEST: f32 = f32::from_bits(0x7F7F_FFFF); // FLT_MAX
        const SIGN_BIT: u32 = 0x8000_0000;

        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }

        lf_checker_rt::global::<u32>(FLAGS_GLOBAL).write(flags);
        if flags & 0x100 != 0 {
            let r = lf_checker_rt::callee_cdecl!(FWD_CALLEE, u32, center, out, radius_bits, flags, arg4, arg5);
            lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32, );
            return r;
        }

        let radius = f32::from_bits(radius_bits);
        let diag_hi = fmul(radius, DIAG);
        let diag_lo = fmul(radius, NEG_DIAG);
        let neg = f32::from_bits(radius_bits ^ SIGN_BIT);
        let zero = 0.0f32;
        // Eight compass offsets: E, NE, N, NW, W, SW, S, SE in (x, y), z = 0.
        // (Order as built by the original: -x, -x/+y diag, +y, +x/+y diag,
        // +x, +x/-y diag, -y, -x/-y diag.)
        let table: [[f32; 3]; 8] = [
            [neg, zero, zero],
            [diag_lo, diag_hi, zero],
            [zero, radius, zero],
            [diag_hi, diag_hi, zero],
            [radius, zero, zero],
            [diag_hi, diag_lo, zero],
            [zero, neg, zero],
            [diag_lo, diag_lo, zero],
        ];
        let cx = rdf(center);
        let cy = rdf(center.wrapping_add(4));
        let cz = rdf(center.wrapping_add(8));

        let mut ids = [0u32; 9];
        ids[0] = lf_checker_rt::callee_cdecl!(RESOLVE_CALLEE, u32, center);
        let mut count: u32 = 1;
        let mut triple = [0u32; 3];
        for entry in table.iter() {
            let t0 = fadd(entry[0], cx);
            let t1 = fadd(entry[1], cy);
            let t2 = fadd(entry[2], cz);
            triple[0] = t0.to_bits();
            triple[1] = t1.to_bits();
            triple[2] = t2.to_bits();
            let id = lf_checker_rt::callee_cdecl!(RESOLVE_CALLEE, u32, triple.as_ptr() as u32);
            let mut k: u32 = 0;
            while k < count {
                if ids[k as usize] == id {
                    break;
                }
                k += 1;
            }
            if k == count {
                ids[count as usize] = id;
                count += 1;
            }
        }

        let mut best = 0u32;
        let mut best_dist = INIT_BEST;
        let code_ptr = lf_checker_rt::relocated(CODE_CONST);
        let mut buf = [0u32; 4];
        let mut j: u32 = 0;
        while j < count {
            let obj = lf_checker_rt::callee_cdecl!(OBJECT_CALLEE, u32, ids[j as usize]);
            j += 1;
            if obj == 0 {
                continue;
            }
            let found = if flags & 8 != 0 {
                lf_checker_rt::callee_thiscall!(PROBE_A_CALLEE, u32, obj, center, radius_bits,
                    buf.as_mut_ptr() as u32, code_ptr, flags, arg4, arg5)
            } else {
                lf_checker_rt::callee_thiscall!(PROBE_B_CALLEE, u32, obj, center, radius_bits,
                    buf.as_mut_ptr() as u32, code_ptr, 1u32, flags, arg4, arg5)
            };
            if found == 0 {
                continue;
            }
            let fx = f32::from_bits(buf[0]);
            let fy = f32::from_bits(buf[1]);
            let fz = f32::from_bits(buf[2]);
            let dx = fsub(fx, cx);
            let dy = fsub(fy, cy);
            let dz = fsub(fz, cz);
            let dist = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
            if best_dist > dist {
                wrf(out, fx);
                wrf(out.wrapping_add(4), fy);
                wrf(out.wrapping_add(8), fz);
                (out.wrapping_add(12) as *mut u32).write_unaligned(buf[3]);
                let flag_bit = (((found.wrapping_add(0x1c)) as *const u8).read() >> 6) & 1;
                best = 1 + flag_bit as u32;
                best_dist = dist;
            }
        }
        lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32, );
        best
    }
});
