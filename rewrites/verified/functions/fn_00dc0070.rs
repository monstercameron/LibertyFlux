// original: 0x00dc0070 match_and_project (proposed)

/// Match-then-project query: find `p3` through a handle scan, sample two
/// vectors, and write one of them (optionally shifted) to `out`.
///
/// `p0`/`p1` point at three floats each, `p2`/`p3` at small headers,
/// `out` at four floats, and `flag`'s low byte selects a final shift.
/// Returns 1 with `out` written, else 0 with `out` untouched.
///
/// Phase 1 resolves two handles: 12-bit ids taken from bits 17-28 of
/// `[p2+4]`/`[p3+4]` are looked up; either answer null ends the call.
/// Phase 2 scans `count` entries (`[p2]` bits 21-24, zero ends the call),
/// starting at `[h1+0x64] + ([p2+4] & 0x1ffff) * 8` in 8-byte steps. Each
/// entry is tested through a match call that rewrites two words of a
/// four-word sample slot (the slot starts zeroed; before each call word 0
/// is ORed with `0xFFFF0FFF` and ANDed back, so bits 12-15 clear, and
/// word 1 is ORed with `0x0FFFFFFF` and ANDed with `0xEFFFFFFF`). An
/// entry matches when the returned status has bits 13-14 clear, its low
/// 12 equal the second id, and `[h2+0x6c] + (status >> 16) * 40` is
/// exactly `p3`. No match in `count` entries ends the call.
///
/// Phase 3 samples two four-float vectors `B` (reusing the slot) and `A`
/// (a fresh zeroed slot) through word-table lookups at `[h1+0x60]`,
/// indexed by `base + k` and `base + ahead`, where `k` is the matching
/// iteration and `ahead` is `k + 1` unless it reaches `count`.
///
/// Phase 4 projects: with `D = A - B` and `E = p1 - p0`, both scales
/// `1/|D|` and `1/|E|` stay 0 for any ordered length and divide only
/// when the squared length is NaN (the original tests this with
/// `ucomiss` plus the parity flag). A chain of products against `p0`,
/// `A` and `B` forms two accumulators; when `|S3 - P| > |S1 - P|` the
/// call writes `B` to `out`, else `A`, then shifts `out.xyz` by a
/// quarter of the scaled `D` (added for `B`, subtracted for `A`) when
/// the flag is set. The full vector is always written before the shift,
/// so a shifted result writes `xyz` twice.
///
/// Original: 0x00DC0070 (stdcall, six stack words; no register inputs).
#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd16(a: u32) -> u32 {
    unsafe { (a as *const u16).read_unaligned() as u32 }
}

#[inline(always)]
unsafe fn rdf(a: u32) -> f32 {
    unsafe { f32::from_bits(rd32(a)) }
}

#[inline(always)]
unsafe fn wrf(a: u32, v: f32) {
    unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

#[inline(always)]
fn fdiv(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}

lf_checker_rt::export!(stdcall, rw_00dc0070(p0: u32, p1: u32, p2: u32, p3: u32, out: u32, flag: u32) -> u8 {
    unsafe {
        const LOOKUP_CALLEE: u32 = 1;
        const MATCH_CALLEE: u32 = 2;
        const SAMPLE1_CALLEE: u32 = 3;
        const SAMPLE2_CALLEE: u32 = 4;
        const ONE_AT: u32 = 0xfe88e8;
        const QUARTER_AT: u32 = 0xfe87e4;
        const SIGN_AT: u32 = 0xfe8fa0;
        const ABS_AT: u32 = 0xfe8f80;

        let h1: u32 =
            lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, (rd32(p2 + 4) >> 17) & 0xfff);
        if h1 == 0 {
            return 0;
        }
        let h2: u32 =
            lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, (rd32(p3 + 4) >> 17) & 0xfff);
        if h2 == 0 {
            return 0;
        }
        let count = (rd32(p2) >> 21) & 0xf;
        if count == 0 {
            return 0;
        }
        let base = rd32(p2 + 4) & 0x1ffff;
        let entries = rd32(h1 + 0x64);
        let arr2 = rd32(h2 + 0x6c);
        let id_b = (rd32(p3 + 4) >> 17) & 0xfff;
        let mut buf = [0u32; 4];
        let mut entry = entries.wrapping_add(base.wrapping_mul(8));
        let mut k: u32 = 0;
        let found = loop {
            buf[0] = (buf[0] | 0xffff_0fff) & 0xffff_0fff;
            buf[1] = (buf[1] | 0x0fff_ffff) & 0xefff_ffff;
            lf_checker_rt::callee_thiscall!(MATCH_CALLEE, u32, entry, buf.as_mut_ptr() as u32);
            let status = buf[0];
            let mut hit = false;
            if status & 0x6000 == 0
                && ((id_b ^ status) & 0xfff) == 0
                && arr2.wrapping_add(((status >> 16) & 0xffff).wrapping_mul(5).wrapping_mul(8))
                    == p3
            {
                hit = true;
            }
            if hit {
                break true;
            }
            k += 1;
            entry = entry.wrapping_add(8);
            if k >= count {
                break false;
            }
        };
        if !found {
            return 0;
        }
        let words = rd32(h1 + 0x60);
        let next = k.wrapping_add(1);
        let ahead = if next < count { next } else { 0 };
        let wv1 = rd16(words.wrapping_add(base.wrapping_add(k).wrapping_mul(2)));
        lf_checker_rt::callee_thiscall!(SAMPLE1_CALLEE, u32, h1, wv1, buf.as_mut_ptr() as u32);
        let mut buf2 = [0u32; 4];
        let wv2 = rd16(words.wrapping_add(base.wrapping_add(ahead).wrapping_mul(2)));
        lf_checker_rt::callee_thiscall!(SAMPLE2_CALLEE, u32, h1, wv2, buf2.as_mut_ptr() as u32);
        let bf = |w: u32| f32::from_bits(w);
        let (b1x, b1y, b1z, b1w) = (bf(buf[0]), bf(buf[1]), bf(buf[2]), bf(buf[3]));
        let (b2x, b2y, b2z, b2w) = (bf(buf2[0]), bf(buf2[1]), bf(buf2[2]), bf(buf2[3]));
        let one = rdf(lf_checker_rt::relocated(ONE_AT));
        let d3 = fsub(b2x, b1x);
        let d4 = fsub(b2y, b1y);
        let d5 = fsub(b2z, b1z);
        let q2 = fadd(fadd(fmul(d4, d4), fmul(d3, d3)), fmul(d5, d5));
        let s1 = if q2.is_nan() { fdiv(one, q2.sqrt()) } else { 0.0 };
        let dx = fsub(rdf(p1), rdf(p0));
        let d3s = fmul(d3, s1);
        let d4s = fmul(d4, s1);
        let dy = fsub(rdf(p1 + 4), rdf(p0 + 4));
        let d5s = fmul(d5, s1);
        let dz = fsub(rdf(p1 + 8), rdf(p0 + 8));
        let q3 = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
        let s2 = if q3.is_nan() { fdiv(one, q3.sqrt()) } else { 0.0 };
        let zero = 0.0f32;
        let dzs = fmul(fmul(dz, s2), zero);
        let dxs = fmul(dx, s2);
        let dys = fmul(dy, s2);
        let y2 = fsub(dys, dzs);
        let z2 = fsub(dzs, dxs);
        let x2 = fsub(fmul(dxs, zero), fmul(dys, zero));
        let p = fadd(
            fadd(fmul(rdf(p0 + 4), z2), fmul(rdf(p0), y2)),
            fmul(rdf(p0 + 8), x2),
        );
        let negp = f32::from_bits(p.to_bits() ^ rd32(lf_checker_rt::relocated(SIGN_AT)));
        let s1acc = fadd(fadd(fmul(y2, b1x), fmul(z2, b1y)), fmul(x2, b1z));
        let s3acc = fadd(fadd(fmul(z2, b2y), fmul(y2, b2x)), fmul(x2, b2z));
        let mask = rd32(lf_checker_rt::relocated(ABS_AT));
        let a1 = f32::from_bits(fadd(s1acc, negp).to_bits() & mask);
        let a3 = f32::from_bits(fadd(s3acc, negp).to_bits() & mask);
        let quarter = rdf(lf_checker_rt::relocated(QUARTER_AT));
        if a3 > a1 {
            wrf(out, b1x);
            wrf(out + 4, b1y);
            wrf(out + 8, b1z);
            wrf(out + 12, b1w);
            if (flag as u8) != 0 {
                wrf(out, fadd(fmul(d3s, quarter), b1x));
                wrf(out + 4, fadd(fmul(d4s, quarter), b1y));
                wrf(out + 8, fadd(fmul(d5s, quarter), b1z));
            }
        } else {
            wrf(out, b2x);
            wrf(out + 4, b2y);
            wrf(out + 8, b2z);
            wrf(out + 12, b2w);
            if (flag as u8) != 0 {
                wrf(out, fsub(b2x, fmul(d3s, quarter)));
                wrf(out + 4, fsub(b2y, fmul(d4s, quarter)));
                wrf(out + 8, fsub(b2z, fmul(d5s, quarter)));
            }
        }
        1
    }
});
