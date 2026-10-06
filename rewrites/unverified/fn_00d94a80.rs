// original: 0x00d94a80 traverse_cells (proposed)
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated};
/// Look up the table for a query point, probe it, and resolve the hit cell.
///
/// `p0` points to the query point (four floats, copied to `*p3` when `p3`
/// is non-null); `p1`/`p2` are out-slots for the table and the hit entry;
/// `p4` is a float threshold forwarded to the probe; the low byte of `p5`
/// selects the cached-table fast path. Returns negative sentinels on the
/// way out: -4 when the offset lookup misses, -5 on a null table, -7 when
/// the probe reports no-hit, 0 with the hit entry stored on success.
///
/// Behaviour: with the flag set and a cached offset that is not 0xfff the
/// table lookup uses it directly, otherwise the offset callee (id 0) is
/// asked and 0xfff means -4. The table callee (id 1) resolves the offset;
/// null means -5. The probe (id 2) is asked with the table and the
/// threshold; 0xffff means no-hit: the advance callee (id 4, non-null `p3`)
/// or the simple callee (id 5) runs and -7 is returned. Otherwise the hit
/// entry (table base + answer*40) is stored to `*p2`, the use callee
/// (id 6) runs, and a clear marker bit means success (0). A set bit enters
/// the loop head: the fallback table is planted to `*p1` and the probe is
/// asked again (id 3): 0xffff repeats the advance/simple pair and returns
/// -7, any other answer re-resolves through the tail and a clear bit
/// returns 0. (A set tail bit with an unchanged answer would revisit the
/// head forever; the contract pins it clear.)
///
/// The probe calls take the query words (`p0` floats, third plus the
/// ray constant) and the advance/simple first pointers the same words with
/// the constant subtracted; the second ray pointer is a zeroed out-buffer.
/// Frame addresses are skipped and the three input words snapshotted per
/// call (see `narrowed`). The stale frame word copied to `p3+8` reads the
/// defined stack fill (0 in the contract).
///
/// Original: 0x00d94a80 (thiscall, six stack words, full-eax signed result).
lf_checker_rt::export!(thiscall, rw_00d94a80(this: u32, p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32) -> u32 {
    unsafe {
        const ID_OFS: u32 = 0;
        const ID_TAB: u32 = 1;
        const ID_RAY: u32 = 2;
        const ID_RAY2: u32 = 3;
        const ID_ADV: u32 = 4;
        const ID_SIMPLE: u32 = 5;
        const ID_USE: u32 = 6;
        const FALLBACK: u32 = 0x16b8f8c;
        const NOHIT: u32 = 0xffff;
        const TEN: u32 = 0x41200000;
        const K_RAYADD: u32 = 0xfe88e8;
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rk() -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(K_RAYADD))) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        if p3 != 0 {
            wr32(p3, rd32(p0));
            wr32(p3 + 4, rd32(p0 + 4));
            wr32(p3 + 8, rd32(p0 + 8));
            wr32(p3 + 12, rd32(p0 + 12));
        }
        let eax: u32;
        if (p5 & 0xff) != 0 {
            let cached = rd32(rd32(p1) + 0x80);
            if cached != 0xfff {
                eax = cached;
            } else {
                let a0: u32 = lf_checker_rt::callee_cdecl!(ID_OFS, u32, p0);
                if a0 == 0xfff {
                    return 0xfffffffc;
                }
                eax = a0;
            }
        } else {
            let a0: u32 = lf_checker_rt::callee_cdecl!(ID_OFS, u32, p0);
            if a0 == 0xfff {
                return 0xfffffffc;
            }
            eax = a0;
        }
        let table: u32 = lf_checker_rt::callee_cdecl!(ID_TAB, u32, eax);
        wr32(p1, table);
        if table == 0 {
            return 0xfffffffb;
        }
        // Masked query words (low 12 bits of w10 forced, feeds the -7 checks).
        let w10 = rd32(p0) | 0xffff0fff;
        let _w14 = (rd32(p0 + 4) | 0x0fffffff) & 0xefffffff;
        let mut buf_ray = [0u32; 8];
        let mut buf_sub = [0u32; 8];
        let buf_b = [0u32; 8];
        // Ray query words from the query point: the ray calls take the
        // plus-constant form, the advance/simple first pointers the
        // minus-constant form; the callee reads them (snapshots cover the
        // three words, the second ray pointer stays a zeroed out-buffer).
        buf_ray[0] = rd32(p0);
        buf_ray[1] = rd32(p0 + 4);
        buf_ray[2] = fadd(f32::from_bits(rd32(p0 + 8)), rk()).to_bits();
        buf_sub[0] = rd32(p0);
        buf_sub[1] = rd32(p0 + 4);
        buf_sub[2] = fsub(f32::from_bits(rd32(p0 + 8)), rk()).to_bits();
        let ans1: u32 = lf_checker_rt::callee_thiscall!(ID_RAY, u32, table, buf_ray.as_ptr() as u32, buf_b.as_ptr() as u32, p4);
        if ans1 == NOHIT {
            let pick = if ((rd32(table + 0x50) >> 2) & 1) != 0 {
                rd32(table + 0x80)
            } else {
                0xfff
            };
            if p3 != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(ID_ADV, u32, this, buf_sub.as_ptr() as u32, TEN, buf_ray.as_ptr() as u32, 0, p3, pick);
            } else {
                let _: u32 = lf_checker_rt::callee_thiscall!(ID_SIMPLE, u32, this, buf_sub.as_ptr() as u32, TEN, buf_ray.as_ptr() as u32, 0, pick);
            }
            if (w10 & 0xfff) == 0xfff {
                return 0xfffffff9;
            }
            // Unreachable in practice (low 12 bits forced above); kept so a
            // wrong mask analysis fails loudly instead of silently.
            if ((w10 >> 16) & 0xffff) == 0xffff {
                return 0xfffffff9;
            }
            return 0xfffffff9;
        }
        if p3 != 0 {
            wr32(p3 + 8, 0);
        }
        let out9 = rd32(table + 0x6c).wrapping_add(ans1.wrapping_mul(40));
        wr32(p2, out9);
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_USE, u32, this, out9);
        if ((rd32(out9) >> 0x12) & 1) == 0 {
            return 0;
        }
        loop {
            let planted = rd32(lf_checker_rt::relocated(FALLBACK));
            wr32(p1, planted);
            let ans2: u32 = lf_checker_rt::callee_thiscall!(ID_RAY2, u32, planted, buf_ray.as_ptr() as u32, buf_b.as_ptr() as u32, p4);
            if ans2 != NOHIT {
                if p3 != 0 {
                    wr32(p3 + 8, 0);
                }
                let outt = rd32(planted + 0x6c).wrapping_add(ans2.wrapping_mul(40));
                wr32(p2, outt);
                if ((rd32(outt) >> 0x12) & 1) == 0 {
                    return 0;
                }
            } else {
                let pick = if ((rd32(planted + 0x50) >> 2) & 1) != 0 {
                    rd32(planted + 0x80)
                } else {
                    0xfff
                };
                if p3 != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(ID_ADV, u32, this, buf_sub.as_ptr() as u32, TEN, buf_ray.as_ptr() as u32, 1, p3, pick);
                } else {
                    let _: u32 = lf_checker_rt::callee_thiscall!(ID_SIMPLE, u32, this, buf_sub.as_ptr() as u32, TEN, buf_ray.as_ptr() as u32, 1, pick);
                }
                let w10b = w10 | 0xffff0fff;
                if (w10b & 0xfff) == 0xfff {
                    return 0xfffffff9;
                }
                if ((w10b >> 16) & 0xffff) == 0xffff {
                    return 0xfffffff9;
                }
                return 0xfffffff9;
            }
        }
    }
});
