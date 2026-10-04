// original: 0x0096C6A0 pose_goal_refresh (proposed)

/// Refresh the goal pose tables of a pose object from its track records.
///
/// `this` points to the object. A predicate callee is asked first; when it
/// answers zero the function returns that answer unchanged, otherwise `edi`
/// (the count at `+0xdb0`) drives three loops. Each loop walks fixed-size
/// records and dispatches on flag bits: records without bit 0 are skipped,
/// records with bit 0 but not bit 2 take a constant path (a -1.0 marker plus
/// a unit vector, with two neighbouring track values copied or shifted by
/// 500.0 depending on unsigned ranges of the count), and records with both
/// bits take a vector path (the length of a delta triple, then the stored
/// triple normalised with a zero length mapping to a zero vector, using the
/// original's operand order and its exact zero-or-NaN branch). The middle
/// loop indexes by the count divided by three; the last loop runs four
/// records and, for its second record only, refreshes one more marker and
/// vector. On the main path the function tail-jumps to a second callee with
/// the count address, returning its answer. Original: 0x0096C6A0 (thiscall,
/// no stack words).
lf_checker_rt::export!(thiscall, rw_0096c6a0(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0xdb0;
        const REC1: u32 = 0xde8;
        const REC2: u32 = 0xf30;
        const REC3: u32 = 0x1000;
        const HALF_K: f32 = 500.0;
        const ONE: f32 = 1.0;
        const NEG_ONE_BITS: u32 = 0xbf800000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn fsqrt(x: f32) -> f32 {
            unsafe {
                use core::arch::x86::{_mm_cvtss_f32, _mm_set_ss, _mm_sqrt_ss};
                _mm_cvtss_f32(_mm_sqrt_ss(_mm_set_ss(core::hint::black_box(x))))
            }
        }
        #[inline(always)]
        unsafe fn inv_or_zero(len2: f32) -> f32 {
            unsafe {
                if len2 != 0.0 { div(ONE, fsqrt(len2)) } else { 0.0 }
            }
        }

        let p: u32 = lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(COUNT));
        if (p & 0xFF) == 0 {
            return p;
        }
        let edi = rd32(this.wrapping_add(COUNT));
        // Loop 1: three records of 0x60 at +0xde8.
        for k in 0..3u32 {
            let rec = this.wrapping_add(REC1).wrapping_add(k.wrapping_mul(0x60));
            let fl = rd8(rec.wrapping_add(0x28));
            if fl & 1 == 0 {
                continue;
            }
            if fl & 4 == 0 {
                wr32(this.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x17e0), NEG_ONE_BITS);
                let a = edi.wrapping_add(0x139).wrapping_mul(2);
                let base = this.wrapping_add(a.wrapping_mul(8));
                wrf(base.wrapping_add(8), ONE);
                wrf(base.wrapping_add(4), ONE);
                wrf(base, ONE);
                let c24 = rdf(rec.wrapping_sub(0x24));
                if edi.wrapping_sub(6) > 12 {
                    wrf(this.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x1840), add(c24, HALF_K));
                } else if edi.wrapping_sub(7) > 10 {
                    wrf(this.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x1840), c24);
                } else {
                    wrf(this.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x1840), sub(c24, HALF_K));
                }
                let c28 = rdf(rec.wrapping_sub(0x28));
                if edi.wrapping_sub(1) > 10 {
                    if (edi as i32) > 12 {
                        wrf(this.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x18a0), sub(c28, HALF_K));
                    } else {
                        wrf(this.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x18a0), c28);
                    }
                } else {
                    wrf(this.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x18a0), add(c28, HALF_K));
                }
            } else {
                let rm4 = rdf(rec.wrapping_sub(4));
                let rm8 = rdf(rec.wrapping_sub(8));
                let d0 = sub(rdf(rec), rdf(rec.wrapping_sub(0x20)));
                let d1 = sub(rm4, rdf(rec.wrapping_sub(0x24)));
                let d2 = sub(rm8, rdf(rec.wrapping_sub(0x28)));
                let len2 = add(add(mul(d2, d2), mul(d1, d1)), mul(d0, d0));
                wrf(this.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x17e0), fsqrt(len2));
                let e0 = rdf(rec.wrapping_add(8));
                let e1 = rdf(rec.wrapping_add(0xc));
                let e2 = rdf(rec.wrapping_add(0x10));
                let d = edi.wrapping_add(0x139).wrapping_mul(2);
                let base = this.wrapping_add(d.wrapping_mul(8));
                wrf(base, e0);
                wrf(base.wrapping_add(4), e1);
                wrf(base.wrapping_add(8), e2);
                wr32(base.wrapping_add(0xc), rd32(rec.wrapping_add(0x14)));
                let nlen2 = add(add(mul(e0, e0), mul(e1, e1)), mul(e2, e2));
                let inv = inv_or_zero(nlen2);
                wrf(base, mul(e0, inv));
                wrf(base.wrapping_add(4), mul(inv, e1));
                wrf(base.wrapping_add(8), mul(inv, e2));
                wrf(this.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x1840), rm4);
                wrf(this.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x18a0), rm8);
            }
        }
        // Loop 2: three records of 0x60 at +0xf30, indexed by count/3.
        let idx = ((edi as i32) / 3) as u32;
        for k in 0..3u32 {
            let rec = this.wrapping_add(REC2).wrapping_add(k.wrapping_mul(0x60));
            let fl = rd8(rec) as u32;
            if fl & 1 == 0 {
                continue;
            }
            if fl & 4 == 0 {
                wr32(this.wrapping_add(idx.wrapping_mul(4)).wrapping_add(0x1690), NEG_ONE_BITS);
            } else {
                let d0 = sub(rdf(rec.wrapping_sub(0x30)), rdf(rec.wrapping_sub(0x50)));
                let d1 = sub(rdf(rec.wrapping_sub(0x2c)), rdf(rec.wrapping_sub(0x4c)));
                let d2 = sub(rdf(rec.wrapping_sub(0x28)), rdf(rec.wrapping_sub(0x48)));
                let len2 = add(add(mul(d1, d1), mul(d0, d0)), mul(d2, d2));
                wrf(this.wrapping_add(idx.wrapping_mul(4)).wrapping_add(0x1690), fsqrt(len2));
            }
        }
        // Loop 3: four records of 0x60 at +0x1000.
        for edx in 0..4u32 {
            let rec = this.wrapping_add(REC3).wrapping_add(edx.wrapping_mul(0x60));
            let fl = rd8(rec.wrapping_add(0x50));
            if fl & 1 == 0 {
                continue;
            }
            let slot = edi.wrapping_add(edx.wrapping_mul(8));
            if fl & 4 == 0 {
                wr32(this.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x1960), NEG_ONE_BITS);
                let c04 = rdf(rec.wrapping_add(4));
                if edi.wrapping_sub(2) > 4 {
                    wrf(this.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x19e0), add(c04, HALF_K));
                } else if edi.wrapping_sub(3) > 2 {
                    wrf(this.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x19e0), c04);
                } else {
                    wrf(this.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x19e0), sub(c04, HALF_K));
                }
                let c00 = rdf(rec);
                if edi.wrapping_sub(1) > 2 {
                    if (edi as i32) > 4 {
                        wrf(this.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x1a60), sub(c00, HALF_K));
                    } else {
                        wrf(this.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x1a60), c00);
                    }
                } else {
                    wrf(this.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x1a60), add(c00, HALF_K));
                }
                let c08 = rdf(rec.wrapping_add(8));
                if (edx as i32) >= 2 {
                    wrf(this.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x1ae0), sub(c08, HALF_K));
                } else {
                    wrf(this.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x1ae0), add(c08, HALF_K));
                }
            } else {
                let v0 = rdf(rec.wrapping_add(0x20));
                let v1 = rdf(rec.wrapping_add(0x24));
                let v2 = rdf(rec.wrapping_add(0x28));
                let d0 = sub(v0, rdf(rec));
                let d1 = sub(v1, rdf(rec.wrapping_add(4)));
                let d2 = sub(v2, rdf(rec.wrapping_add(8)));
                let len2 = add(add(mul(d1, d1), mul(d0, d0)), mul(d2, d2));
                wrf(this.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x1960), fsqrt(len2));
                wrf(this.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x19e0), v1);
                wrf(this.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x1a60), v0);
                wrf(this.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x1ae0), v2);
            }
            if edx != 1 {
                continue;
            }
            if fl & 4 == 0 {
                wr32(this.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x1358), NEG_ONE_BITS);
                let a = edi.wrapping_add(0x151).wrapping_mul(2);
                let base = this.wrapping_add(a.wrapping_mul(8));
                wrf(base.wrapping_add(8), ONE);
                wrf(base.wrapping_add(4), ONE);
                wrf(base, ONE);
            } else {
                let v0 = rdf(rec.wrapping_add(0x20));
                let v1 = rdf(rec.wrapping_add(0x24));
                let v2 = rdf(rec.wrapping_add(0x28));
                let d0 = sub(v0, rdf(rec));
                let d1 = sub(v1, rdf(rec.wrapping_add(4)));
                let d2 = sub(v2, rdf(rec.wrapping_add(8)));
                let len2 = add(add(mul(d1, d1), mul(d0, d0)), mul(d2, d2));
                wrf(this.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x1358), fsqrt(len2));
                let e0 = rdf(rec.wrapping_add(0x30));
                let e1 = rdf(rec.wrapping_add(0x34));
                let e2 = rdf(rec.wrapping_add(0x38));
                let c = edi.wrapping_add(0x151).wrapping_mul(2);
                let base = this.wrapping_add(c.wrapping_mul(8));
                wrf(base, e0);
                wrf(base.wrapping_add(4), e1);
                wrf(base.wrapping_add(8), e2);
                wr32(base.wrapping_add(0xc), rd32(rec.wrapping_add(0x3c)));
                let nlen2 = add(add(mul(e1, e1), mul(e0, e0)), mul(e2, e2));
                let inv = inv_or_zero(nlen2);
                wrf(base, mul(e0, inv));
                wrf(base.wrapping_add(4), mul(inv, e1));
                wrf(base.wrapping_add(8), mul(inv, e2));
            }
        }
        lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(COUNT))
    }
});
