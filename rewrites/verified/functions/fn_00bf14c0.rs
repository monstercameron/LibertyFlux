// original: 0x00bf14c0 files_memory_record_build
// Derive the compact record `this_ptr` from the source object `src`.
//
// Flag bits of two header bytes come from source words and bytes, from one
// pointer-identity test against a manager answer, and from two vtable
// queries; two bytes come from float scaling (`src+0xAA0` by a constant,
// `src+0xE0C` clamped to unit range then scaled); an 11-byte table is filled
// from a second vtable query with per-index bit merging gated by a startup
// global; a 6-byte interleave comes from a linked object or a constant
// pattern; the first counted loop runs table lookups and 3x3 float
// transforms per index; the second counted loop (while the index is below
// the count minus a flags-derived base) runs a short call on a zero word
// and otherwise one of two 3x3 float-transform blocks selected by a table
// answer. Covers the whole function.
export!(thiscall, rw_00BF14C0(this_ptr: u32, src: u32, flags: u32) -> () {
    unsafe {
        let thisp = this_ptr as *mut u8;
        let srcp = src as *const u8;
        let r32 = |b: *const u8, off: usize| -> u32 { *b.add(off).cast::<u32>() };
        let r16 = |b: *const u8, off: usize| -> u16 { *b.add(off).cast::<u16>() };
        let r8 = |b: *const u8, off: usize| -> u8 { *b.add(off) };
        let rf = |b: *const u8, off: usize| -> f32 { *b.add(off).cast::<f32>() };
        let w32 = |b: *mut u8, off: usize, v: u32| { *b.add(off).cast::<u32>() = v; };
        let w16 = |b: *mut u8, off: usize, v: u16| { *b.add(off).cast::<u16>() = v; };
        let w8 = |b: *mut u8, off: usize, v: u8| { *b.add(off) = v; };

        callee_thiscall!(1, u32, this_ptr, 0x1b);

        let linked = r32(srcp, 0x21c);
        w8(thisp, 8, r8(linked as *const u8, 0x12c));

        let src24 = r32(srcp, 0x24);
        w8(thisp, 9, b137_set_bits(r8(thisp, 9), 0x01, (src24 >> 5) as u8));

        // If src+0x210 is set with src+0xA74 equal to 1 or 2 the answer is
        // true, otherwise it falls through to testing src+0x224 for zero.
        let sel: bool = (r8(srcp, 0x210) != 0 && {
            let e = r32(srcp, 0xa74);
            e == 1 || e == 2
        }) || r32(srcp, 0x224) == 0;
        w8(thisp, 9, b137_set_bits(r8(thisp, 9), 0x04, (sel as u8) << 2));

        let mgr = callee_cdecl!(2, u32, 0);
        w8(thisp, 9, b137_set_bits(r8(thisp, 9), 0x10, ((src == mgr) as u8) << 4));

        let bit1: bool = (src24 >> 27) & 1 != 0
            && r32(srcp, 0x48) != 0xFFFF_FFFF
            && r8(srcp, 0x40) != 0x3f;
        w8(thisp, 9, b137_set_bits(r8(thisp, 9), 0x02, (bit1 as u8) << 1));

        w32(thisp, 0x0c, r32(srcp, 0x64));
        w16(thisp, 0x14, r16(srcp, 0x2e));
        w8(thisp, 0x0a, b137_cvtt_lo(rf(srcp, 0xaa0) * f32::from_bits(0x4223_0EAC)));
        w8(thisp, 0x0b, r8(srcp, 0x63));
        w32(thisp, 0x10, 0xFFFF_FFFF);
        w8(thisp, 9, b137_set_bits(r8(thisp, 9), 0x08, r8(srcp, 0x26c) << 3));
        w8(thisp, 0x16, b137_set_bits(r8(thisp, 0x16), 0x07, r8(srcp, 0xb80)));

        // First vtable group: slot A0 tested, then re-called; its answer's
        // slot E0 selects X, else the linked object at src+0x100.
        let probe = b137_vcall(src, 0xa0);
        let x: u32 = if probe == 0 {
            r32(srcp, 0x100)
        } else {
            let obj = b137_vcall(src, 0xa0);
            let _ = r32(obj as *const u8, 0);
            b137_vcall(obj, 0xe0)
        };
        let sel_obj = r32(x as *const u8, 4);
        let count = r8(sel_obj as *const u8, 0x14);

        w8(thisp, 0x16, r8(thisp, 0x16) & 7);
        let head16 = r8(thisp, 0x16);
        w8(thisp, 0x17, count);

        if r8(srcp, 0x26c) & 4 != 0 {
            let r = r32(srcp, 0xb30);
            if r != 0 {
                w32(thisp, 0x10, r32(r as *const u8, 0x64));
                let r2 = r32(srcp, 0xb30);
                if r32(r2 as *const u8, 0x1304) == 1 {
                    let x = rf(srcp, 0xe0c);
                    // The original skips exactly when x == 0.0 (either sign;
                    // NaN proceeds): `x != 0.0` matches, then a 0..1 clamp.
                    if x != 0.0 {
                        let c = if x < 0.0 {
                            0.0
                        } else if x > 1.0 {
                            1.0
                        } else {
                            x
                        };
                        w8(thisp, 0x16, (b137_cvtt_lo(c * 31.0) << 3) | (head16 & 7));
                    }
                }
            }
        }

        if r8(thisp, 9) & 2 != 0 {
            w32(thisp, 0x18, r32(srcp, 0xb8));
            w32(thisp, 0x1c, r32(srcp, 0x48));
        } else {
            w32(thisp, 0x18, 0xFFFF_FFFF);
            w32(thisp, 0x1c, 0xFFFF_FFFF);
        }

        callee_thiscall!(3, u32, this_ptr, r32(srcp, 0x20));

        for k in 0..11u32 {
            let d = b137_vcall(src, 0xd0);
            let b = r8((d.wrapping_add(k)) as *const u8, 0x5c);
            let slot = this_ptr.wrapping_add(0x30).wrapping_add(k) as *mut u8;
            *slot = b137_set_bits(*slot, 0x7f, b);
            let d2 = b137_vcall(src, 0xd0);
            w8(thisp, (0x3b + k) as usize, r8((d2.wrapping_add(k)) as *const u8, 0x67));
            let g = *global::<u32>(0x011D_6FD4);
            if g == 1 || g == 2 {
                if r8(thisp, 9) & 0x10 != 0 {
                    let dl: u8 = if k < 16 {
                        *global::<u8>(0x0150_E0DCu32.wrapping_add(k))
                    } else {
                        1
                    };
                    let cur = r8(thisp, (0x30 + k) as usize);
                    w8(thisp, (0x30 + k) as usize, (cur & 0x7f) | ((dl & 1) << 7));
                }
            }
        }

        w8(thisp, 9, r8(thisp, 9) & 0x9f);

        let df = b137_vcall(src, 0xd0);
        if df != 0 {
            let ab = callee_thiscall!(4, u32, linked.wrapping_add(0x80));
            w8(thisp, 9, b137_set_bits(r8(thisp, 9), 0x60, (ab as u8) << 5));
        }

        if r8(thisp, 9) & 0x60 != 0 {
            for i in 0..2u32 {
                let base = linked.wrapping_add(i.wrapping_mul(0x40));
                let o = this_ptr.wrapping_add(0x48).wrapping_add(i);
                if r32(base as *const u8, 0x80) == 0xFFFF_FFFF {
                    *((o.wrapping_sub(2)) as *mut u8) = 0xff;
                    *(o as *mut u8) = 0;
                    *((o.wrapping_add(2)) as *mut u8) = 0;
                } else {
                    *((o.wrapping_sub(2)) as *mut u8) = r8(base as *const u8, 0x80);
                    *(o as *mut u8) = r8(base as *const u8, 0x84);
                    *((o.wrapping_add(2)) as *mut u8) = r8(base as *const u8, 0x88);
                }
            }
        } else {
            for i in 0..2u32 {
                let o = this_ptr.wrapping_add(0x48).wrapping_add(i);
                *((o.wrapping_sub(2)) as *mut u8) = 0xff;
                *(o as *mut u8) = 0;
                *((o.wrapping_add(2)) as *mut u8) = 0;
            }
        }

        let l4c: u32 = if flags & 0xFF != 0 { 10 } else { 0 };

        callee_thiscall!(5, u32, src);

        // Second and third vtable groups: same shape; their results feed the
        // counted loops below.
        let v2 = b137_vcall(src, 0xa0);
        let l14: u32 = if v2 == 0 {
            r32(srcp, 0x100)
        } else {
            let obj = b137_vcall(src, 0xa0);
            let _ = r32(obj as *const u8, 0);
            b137_vcall(obj, 0xe0)
        };
        let v3 = b137_vcall(src, 0xa0);
        let x3: u32 = if v3 == 0 {
            r32(srcp, 0x100)
        } else {
            let obj = b137_vcall(src, 0xa0);
            let _ = r32(obj as *const u8, 0);
            b137_vcall(obj, 0xe0)
        };
        // Loaded before the count test even on the skip path.
        let l1c: u32 = r32(x3 as *const u8, 4);

        // The word slot shared by both loops: the entry saves `this` here,
        // loop 1 stores each index's word then (on the float path) its last
        // delta, and loop 2 reads the upper bytes back under its own index.
        let mut e20: u32 = this_ptr;
        // First counted loop: per index, two table lookups gate the body;
        // a zero word takes the short call pair, otherwise a block of four
        // 3x3 float transforms feeds the same tail call.
        let mut l18: u32 = 0;
        let mut idx: u8 = 0;
        while idx < count {
            let w = callee_cdecl!(9, u32, idx as u32, flags) as u16;
            e20 = (w as u32) & 0xFFFF;
            let wb = callee_cdecl!(10, u32, w as u32) as u8;
            let run: bool = if wb != 0 {
                true
            } else {
                (callee_cdecl!(11, u32, w as u32) as u8) != 0
            };
            if run {
                if w == 0 {
                    let p = callee_thiscall!(12, u32, src, idx as u32);
                    callee_thiscall!(13, u32, this_ptr, p, l18);
                } else {
                    let t1 = r32(l14 as *const u8, 0x14);
                    let s3 = l1c;
                    let t2 = r32(s3 as *const u8, 0);
                    let row = r32(
                        (t2.wrapping_add((idx as u32).wrapping_mul(0xe0))) as *const u8,
                        0x10,
                    );
                    let word = r16(row as *const u8, 0x14) as u32;
                    let m = t1.wrapping_add(word.wrapping_mul(0x40));
                    let vb = t1.wrapping_add((idx as u32).wrapping_mul(0x40));
                    let mp = m as *const u8;
                    let vp = vb as *const u8;
                    let m0 = rf(mp, 0x00);
                    let m1 = rf(mp, 0x04);
                    let m2 = rf(mp, 0x08);
                    let m3 = rf(mp, 0x10);
                    let m4 = rf(mp, 0x14);
                    let m5 = rf(mp, 0x18);
                    let m6 = rf(mp, 0x20);
                    let m7 = rf(mp, 0x24);
                    let m8 = rf(mp, 0x28);
                    // Four row-triples; each output is (a+b)+c in this order,
                    // with pinned operand order (see b137_fmul/b137_fadd).
                    // The block mirrors the original frame layout exactly:
                    // gaps are zero (the contract fills uninit stack with 0).
                    let a0 = rf(vp, 0x00);
                    let a1 = rf(vp, 0x04);
                    let a2 = rf(vp, 0x08);
                    let ax = b137_fadd(b137_fadd(b137_fmul(a0, m0), b137_fmul(a1, m1)), b137_fmul(a2, m2));
                    let ay = b137_fadd(b137_fadd(b137_fmul(a0, m3), b137_fmul(a1, m4)), b137_fmul(a2, m5));
                    let az = b137_fadd(b137_fadd(b137_fmul(a0, m6), b137_fmul(a1, m7)), b137_fmul(a2, m8));
                    let b0 = rf(vp, 0x10);
                    let b1 = rf(vp, 0x14);
                    let b2 = rf(vp, 0x18);
                    let bx = b137_fadd(b137_fadd(b137_fmul(b0, m0), b137_fmul(b1, m1)), b137_fmul(b2, m2));
                    let by = b137_fadd(b137_fadd(b137_fmul(b0, m3), b137_fmul(b1, m4)), b137_fmul(b2, m5));
                    let bz = b137_fadd(b137_fadd(b137_fmul(b0, m6), b137_fmul(b1, m7)), b137_fmul(b2, m8));
                    let c0 = rf(vp, 0x20);
                    let c1 = rf(vp, 0x24);
                    let c2 = rf(vp, 0x28);
                    let cx = b137_fadd(b137_fadd(b137_fmul(c0, m0), b137_fmul(c1, m1)), b137_fmul(c2, m2));
                    let cy = b137_fadd(b137_fadd(b137_fmul(c0, m3), b137_fmul(c1, m4)), b137_fmul(c2, m5));
                    let cz = b137_fadd(b137_fadd(b137_fmul(c0, m6), b137_fmul(c1, m7)), b137_fmul(c2, m8));
                    let dx = rf(vp, 0x30) - rf(mp, 0x30);
                    let dy = rf(vp, 0x34) - rf(mp, 0x34);
                    let dz = rf(vp, 0x38) - rf(mp, 0x38);
                    e20 = dx.to_bits();
                    let ox = b137_fadd(b137_fadd(b137_fmul(dx, m0), b137_fmul(dy, m1)), b137_fmul(dz, m2));
                    let oy = b137_fadd(b137_fadd(b137_fmul(dx, m3), b137_fmul(dy, m4)), b137_fmul(dz, m5));
                    let oz = b137_fadd(b137_fadd(b137_fmul(dx, m6), b137_fmul(dy, m7)), b137_fmul(dz, m8));
                    let blk = [
                        ax.to_bits(),
                        ay.to_bits(),
                        az.to_bits(),
                        0,
                        bx.to_bits(),
                        by.to_bits(),
                        bz.to_bits(),
                        0,
                        cx.to_bits(),
                        cy.to_bits(),
                        cz.to_bits(),
                        0,
                        ox.to_bits(),
                        oy.to_bits(),
                        oz.to_bits(),
                        0,
                    ];
                    callee_thiscall!(14, u32, this_ptr, blk.as_ptr() as u32, l18);
                }
                l18 = l18.wrapping_add(1);
            }
            idx = idx.wrapping_add(1);
        }
        // Second counted loop: runs while the index is below count minus
        // the flags-derived base (so only when the low flag byte is clear
        // and the count is positive, given the pinned count range). Each
        // index looks up a word; zero takes a short call with a counter
        // argument, otherwise a table answer selects path B (zero) or path A
        // (nonzero), each a block of four 3x3 float transforms. The word
        // argument carries loop 1's residue in its upper bytes.
        let trips: i32 = (count as i32).wrapping_sub(l4c as i32);
        if trips > 0 {
            let t1 = r32(l14 as *const u8, 0x14);
            let t2 = r32(l1c as *const u8, 0);
            let mut l10: u32 = 0;
            // Only the low byte is cleared; the rest is loop 1's counter,
            // which never exceeds a byte (masked, not assumed).
            let mut l18b: u32 = l18 & 0xFFFF_FF00;
            let mut l20: u8 = 0;
            loop {
                let l20dword: u32 = (e20 & 0xFFFF_FF00) | (l20 as u32);
                let w2 = callee_cdecl!(15, u32, l20dword, flags) as u16;
                if w2 == 0 {
                    let hybrid = (l20 as u32)
                        .wrapping_mul(0x40)
                        .wrapping_add(t1);
                    callee_thiscall!(17, u32, this_ptr, hybrid, l10);
                    l10 = l10.wrapping_add(1);
                } else {
                    let gate = callee_cdecl!(16, u32, w2 as u32) as u8;
                    let vb = (l20 as u32).wrapping_mul(0x40).wrapping_add(t1);
                    let row = r32(
                        (t2.wrapping_add((l20 as u32).wrapping_mul(0xe0))) as *const u8,
                        0x10,
                    );
                    let word = r16(row as *const u8, 0x14) as u32;
                    let m = t1.wrapping_add(word.wrapping_mul(0x40));
                    let mp = m as *const u8;
                    let vp = vb as *const u8;
                    let m0 = rf(mp, 0x00);
                    let m1 = rf(mp, 0x04);
                    let m2 = rf(mp, 0x08);
                    let m3 = rf(mp, 0x10);
                    let m4 = rf(mp, 0x14);
                    let m5 = rf(mp, 0x18);
                    let m6 = rf(mp, 0x20);
                    let m7 = rf(mp, 0x24);
                    let m8 = rf(mp, 0x28);
                    let a0 = rf(vp, 0x00);
                    let a1 = rf(vp, 0x04);
                    let a2 = rf(vp, 0x08);
                    let b0 = rf(vp, 0x10);
                    let b1 = rf(vp, 0x14);
                    let b2 = rf(vp, 0x18);
                    let c0 = rf(vp, 0x20);
                    let c1 = rf(vp, 0x24);
                    let c2 = rf(vp, 0x28);
                    let d0 = rf(vp, 0x30);
                    let d1 = rf(vp, 0x34);
                    let d2 = rf(vp, 0x38);
                    // The branch tests the table answer's low byte only; the
                    // row word feeds both paths as the matrix selector.
                    let take_b: bool = gate == 0;
                    if take_b {
                        let ax = b137_fadd(b137_fadd(b137_fmul(a0, m0), b137_fmul(a1, m1)), b137_fmul(a2, m2));
                        let ay = b137_fadd(b137_fadd(b137_fmul(a1, m4), b137_fmul(a0, m3)), b137_fmul(a2, m5));
                        let az = b137_fadd(b137_fadd(b137_fmul(a1, m7), b137_fmul(m6, a0)), b137_fmul(a2, m8));
                        let bx = b137_fadd(b137_fadd(b137_fmul(b0, m0), b137_fmul(b1, m1)), b137_fmul(b2, m2));
                        let by = b137_fadd(b137_fadd(b137_fmul(b0, m3), b137_fmul(b1, m4)), b137_fmul(b2, m5));
                        let bz = b137_fadd(b137_fadd(b137_fmul(m7, b1), b137_fmul(m6, b0)), b137_fmul(m8, b2));
                        let cx = b137_fadd(b137_fadd(b137_fmul(m0, c0), b137_fmul(c1, m1)), b137_fmul(c2, m2));
                        let cy = b137_fadd(b137_fadd(b137_fmul(m3, c0), b137_fmul(c1, m4)), b137_fmul(c2, m5));
                        let cz = b137_fadd(b137_fadd(b137_fmul(m6, c0), b137_fmul(m7, c1)), b137_fmul(m8, c2));
                        let dx = d0 - rf(mp, 0x30);
                        let dy = d1 - rf(mp, 0x34);
                        let dz = d2 - rf(mp, 0x38);
                        let ox = b137_fadd(b137_fadd(b137_fmul(m0, dx), b137_fmul(dy, m1)), b137_fmul(dz, m2));
                        let oy = b137_fadd(b137_fadd(b137_fmul(m3, dx), b137_fmul(dy, m4)), b137_fmul(dz, m5));
                        let oz = b137_fadd(b137_fadd(b137_fmul(m6, dx), b137_fmul(m7, dy)), b137_fmul(m8, dz));
                        let blk = [
                            ax.to_bits(), ay.to_bits(), az.to_bits(), 0,
                            bx.to_bits(), by.to_bits(), bz.to_bits(), 0,
                            cx.to_bits(), cy.to_bits(), cz.to_bits(), 0,
                            ox.to_bits(), oy.to_bits(), oz.to_bits(), 0,
                        ];
                        callee_thiscall!(19, u32, this_ptr, blk.as_ptr() as u32, l18b);
                        l18b = (l18b & 0xFFFF_FF00)
                            | ((l18b as u8).wrapping_add(1) as u32);
                    } else {
                        let ax = b137_fadd(b137_fadd(b137_fmul(a0, m0), b137_fmul(m1, a1)), b137_fmul(m2, a2));
                        let ay = b137_fadd(b137_fadd(b137_fmul(a0, m3), b137_fmul(m4, a1)), b137_fmul(m5, a2));
                        let az = b137_fadd(b137_fadd(b137_fmul(a1, m7), b137_fmul(m6, a0)), b137_fmul(a2, m8));
                        let bx = b137_fadd(b137_fadd(b137_fmul(b0, m0), b137_fmul(m1, b1)), b137_fmul(m2, b2));
                        let by = b137_fadd(b137_fadd(b137_fmul(b0, m3), b137_fmul(m4, b1)), b137_fmul(m5, b2));
                        let bz = b137_fadd(b137_fadd(b137_fmul(b1, m7), b137_fmul(m6, b0)), b137_fmul(b2, m8));
                        let cx = b137_fadd(b137_fadd(b137_fmul(c0, m0), b137_fmul(m1, c1)), b137_fmul(m2, c2));
                        let cy = b137_fadd(b137_fadd(b137_fmul(c0, m3), b137_fmul(m4, c1)), b137_fmul(m5, c2));
                        let cz = b137_fadd(b137_fadd(b137_fmul(c1, m7), b137_fmul(m6, c0)), b137_fmul(c2, m8));
                        let dx = d0 - rf(mp, 0x30);
                        let dy = d1 - rf(mp, 0x34);
                        let dz = d2 - rf(mp, 0x38);
                        let ox = b137_fadd(b137_fadd(b137_fmul(dx, m0), b137_fmul(m1, dy)), b137_fmul(m2, dz));
                        let oy = b137_fadd(b137_fadd(b137_fmul(dx, m3), b137_fmul(m4, dy)), b137_fmul(m5, dz));
                        let oz = b137_fadd(b137_fadd(b137_fmul(dy, m7), b137_fmul(m6, dx)), b137_fmul(dz, m8));
                        let blk = [
                            ax.to_bits(), ay.to_bits(), az.to_bits(), 0,
                            bx.to_bits(), by.to_bits(), bz.to_bits(), 0,
                            cx.to_bits(), cy.to_bits(), cz.to_bits(), 0,
                            ox.to_bits(), oy.to_bits(), oz.to_bits(), 0,
                        ];
                        callee_thiscall!(18, u32, this_ptr, blk.as_ptr() as u32, l10);
                        l10 = l10.wrapping_add(1);
                    }
                }
                l20 = l20.wrapping_add(1);
                if (l20 as i32) >= trips {
                    break;
                }
            }
        }
    }
});

/// Float multiply with the original's exact NaN propagation. Hardware
/// `mulss` keeps the first NaN operand (quieted), but the compiler is free
/// to swap the operands, which changes two-NaN results (proven twice by
/// r-b137). The NaN cases are decided explicitly on the bits and only
/// NaN-free operands reach hardware, where swapping is value-preserving.
#[inline(always)]
fn b137_fmul(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        f32::from_bits(a.to_bits() | 0x0040_0000)
    } else if b.is_nan() {
        f32::from_bits(b.to_bits() | 0x0040_0000)
    } else {
        core::hint::black_box(a) * b
    }
}

/// Float add with the same explicit NaN rule: first NaN operand wins,
/// quieted. Matches `addss` bit for bit, immune to operand swapping.
#[inline(always)]
fn b137_fadd(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        f32::from_bits(a.to_bits() | 0x0040_0000)
    } else if b.is_nan() {
        f32::from_bits(b.to_bits() | 0x0040_0000)
    } else {
        core::hint::black_box(a) + b
    }
}


/// Set `mask` bits of `old` from `val`, keeping the other bits.
#[inline(always)]
fn b137_set_bits(old: u8, mask: u8, val: u8) -> u8 {
    (old & !mask) | (val & mask)
}

/// Low byte of x86 `cvttss2si`, including its out-of-range rule: NaN,
/// infinities and values outside `i32` convert to `0x80000000`, whose low
/// byte is 0. Rust's saturating `as` cast agrees everywhere except positive
/// overflow (it yields `i32::MAX`, low byte `0xFF`), so only that case needs
/// the explicit rule; negative overflow already yields `0x00` both ways.
#[inline(always)]
fn b137_cvtt_lo(x: f32) -> u8 {
    if x >= 2147483648.0 {
        0
    } else {
        ((x as i32) & 0xFF) as u8
    }
}

/// One indirect `thiscall` with no stack arguments through a fabricated
/// object, exactly like the original: load the table pointer, load the slot,
/// call it. Both sides land on the same planted recorder stub.
#[inline(always)]
unsafe fn b137_vcall(obj: u32, slot: usize) -> u32 {
    let vt = *(obj as *const u32);
    let addr = *((vt.wrapping_add(slot as u32)) as *const u32);
    let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(addr as usize);
    f(obj)
}
