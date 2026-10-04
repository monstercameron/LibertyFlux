// original: 0x009F5680 net_obj_build (proposed)

/// Build a network object for a slot, fit its geometry, and report it.
///
/// `arg` is a slot index; a negative slot returns zero at once. Otherwise
/// the element table at `STRUCT` (base at `+0`, bit base at `+4`, count at
/// `+8`, stride at `+0xC`) is scanned from the top: an element whose bit
/// byte has 0x80 set, whose address is null, whose flag at `+0xF19` lacks
/// bit 2, whose kind at `+0x10B8` is not 1, or whose state at `+0xF50` is
/// not zero is skipped; a kept element is probed through callee 2 and, on
/// a zero answer, marked through callee 3 and reported through the shared
/// table's slot 8 (callee 4). Callee 5 must answer at least 5, then the
/// slot is registered (callees 6, 7, 8) and gated through callee 9 (a zero
/// low byte returns zero); bit 1 of callee 6's answer skips a second
/// registration (callee 10).
///
/// Then geometry is fitted: callee 11 initialises the scratch block,
/// callee 12 supplies a vector source scaled by 10, callee 13 stages
/// sixteen bytes and supplies an offset triple, and the shared table's
/// slot 4 (callee 14) builds the object, returning null on failure. Four
/// rounds follow (round `i` runs switch case `i`: scale, stage, combine
/// with adds on rounds 0 and 1 and subtracts on rounds 2 and 3), each
/// round running the object's slot-4 setup (callee 15), an optional mode
/// call picked by the word at `+0x1300` (callee 16 for 1, callee 17 for
/// 0, with a flag for the low three bits of `+0x10B8` and two 5.0
/// words), and an eight-block matrix stage over the base at `+0x20` and
/// the table row picked by the signed word at `+0x2E`. Rounds 0 to 2 end
/// with five evaluator calls (callee 18): a zero low byte starts the
/// next round, five nonzero answers finish early. The end calls are
/// callees 19 and 20, the word at `+0x12D0` is set, and the object is
/// returned.
///
/// Float order is the original's, pinned through black-boxed helpers.
/// Original: 0x009F5680 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_009f5680(arg: u32) -> u32 {
    unsafe {
        const STRUCT: u32 = 0x012E_22A4;
        const VTABLE_SRC: u32 = 0x0166_D9FC;
        const MGR: u32 = 0x012B_4138;
        const ROW_TABLE: u32 = 0x0129_5CD8;
        const K10: u32 = 0x00FE_8B08;
        const K2: u32 = 0x00FE_8A24;
        const K1: u32 = 0x00FE_88E8;
        const ELEM_FLAG: u32 = 0xF19;
        const ELEM_KIND: u32 = 0x10B8;
        const ELEM_STATE: u32 = 0xF50;
        const OBJ_MATRIX: u32 = 0x20;
        const OBJ_ROW: u32 = 0x2E;
        const OBJ_MODE: u32 = 0x41;
        const OBJ_WHICH: u32 = 0x1300;
        const OBJ_DONE: u32 = 0x12D0;
        const VT_SLOT_A: u32 = 8;
        const VT_SLOT_B: u32 = 4;
        const FIVE: f32 = 5.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn frd(f: &[u32], off: usize) -> f32 {
            unsafe { f32::from_bits(*f.get_unchecked(off >> 2)) }
        }
        #[inline(always)]
        unsafe fn fwr(f: &mut [u32], off: usize, v: f32) {
            unsafe { *f.get_unchecked_mut(off >> 2) = v.to_bits() }
        }

        if (arg as i32) < 0 {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(1, u32,);
        let gstruct = lf_checker_rt::global::<u32>(STRUCT);
        let gmgr = lf_checker_rt::global::<u32>(MGR);
        let gvt = lf_checker_rt::global::<u32>(VTABLE_SRC);
        let mut frame = [0u32; 0x60];
        let mut esi = 0u32;
        let st = rd32(gstruct as u32);
        let count = rd32(st.wrapping_add(8));
        frame[0x10 >> 2] = st;
        if count != 0 {
            let mut ecx = count;
            loop {
                let bits = rd32(st.wrapping_add(4));
                ecx = ecx.wrapping_sub(1);
                frame[0x28 >> 2] = ecx;
                let mut elem = 0u32;
                if rd8(ecx.wrapping_add(bits)) & 0x80 == 0 {
                    let stride = rd32(st.wrapping_add(0x0C));
                    elem = stride.wrapping_mul(ecx).wrapping_add(rd32(st));
                    if elem != 0
                        && rd8(elem.wrapping_add(ELEM_FLAG)) & 4 != 0
                        && rd8(elem.wrapping_add(ELEM_KIND)) == 1
                        && rd32(elem.wrapping_add(ELEM_STATE)) == esi
                    {
                        let probe = lf_checker_rt::callee_thiscall!(2, u32, elem);
                        if probe == 0 {
                            lf_checker_rt::callee_cdecl!(3, u32, elem, 0);
                            let obj = rd32(gvt as u32);
                            let slot = rd32(rd32(obj).wrapping_add(VT_SLOT_A));
                            let f: extern "thiscall" fn(u32, u32) -> u32 =
                                core::mem::transmute(slot as usize);
                            f(obj, elem);
                        }
                    }
                }
                let _ = elem;
                if ecx == 0 {
                    break;
                }
            }
        }
        let check = lf_checker_rt::callee_thiscall!(5, u32, st);
        if (check as i32) < 5 {
            return 0;
        }
        let mgr = rd32(gmgr as u32);
        let r6 = lf_checker_rt::callee_cdecl!(6, u32, arg, mgr);
        frame[0x10 >> 2] = r6;
        lf_checker_rt::callee_cdecl!(7, u32, arg, mgr, 2);
        lf_checker_rt::callee_cdecl!(8, u32, 0);
        let gate = lf_checker_rt::callee_cdecl!(9, u32, arg, mgr);
        if gate & 0xFF == 0 {
            return esi;
        }
        if r6 & 2 == 0 {
            lf_checker_rt::callee_cdecl!(10, u32, arg, mgr);
        }
        let k10 = f32::from_bits(rd32(lf_checker_rt::relocated(K10)));
        let k2 = f32::from_bits(rd32(lf_checker_rt::relocated(K2)));
        let k1 = f32::from_bits(rd32(lf_checker_rt::relocated(K1)));
        let fptr = frame.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(11, u32, fptr.wrapping_add(0x60));
        let p0 = lf_checker_rt::callee_thiscall!(12, u32, 0);
        let p1 = rd32(p0.wrapping_add(0x20));
        fwr(&mut frame, 0x10, fmul(f32::from_bits(rd32(p1.wrapping_add(0x10))), k10));
        fwr(&mut frame, 0x28, fmul(f32::from_bits(rd32(p1.wrapping_add(0x14))), k10));
        fwr(&mut frame, 0x30, fmul(f32::from_bits(rd32(p1.wrapping_add(0x18))), k10));
        let q = lf_checker_rt::callee_cdecl!(13, u32, fptr.wrapping_add(0x50));
        let q0 = f32::from_bits(rd32(q));
        let q1 = f32::from_bits(rd32(q.wrapping_add(4)));
        let q2 = f32::from_bits(rd32(q.wrapping_add(8)));
        let c2 = fadd(q2, frd(&frame, 0x30));
        let c0 = fadd(q1, frd(&frame, 0x28));
        let c1 = fadd(q0, frd(&frame, 0x10));
        let c2 = fadd(c2, k2);
        fwr(&mut frame, 0x94, c0);
        let tail0 = frd(&frame, 0x5C);
        fwr(&mut frame, 0x90, c1);
        fwr(&mut frame, 0x9C, tail0);
        fwr(&mut frame, 0x98, c2);
        let gvt_obj = rd32(gvt as u32);
        let slot_b = rd32(rd32(gvt_obj).wrapping_add(VT_SLOT_B));
        let build: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot_b as usize);
        esi = build(gvt_obj, arg, 1, fptr.wrapping_add(0x60), 1);
        if esi == 0 {
            return esi;
        }
        wr8(esi.wrapping_add(ELEM_FLAG), rd8(esi.wrapping_add(ELEM_FLAG)) | 4);
        wr8(esi.wrapping_add(0xF16), rd8(esi.wrapping_add(0xF16)) | 2);
        let mut edi = 0u32;
        loop {
            lf_checker_rt::callee_thiscall!(11, u32, fptr.wrapping_add(0x60));
            let p0 = lf_checker_rt::callee_thiscall!(12, u32, 0);
            let p1 = rd32(p0.wrapping_add(0x20));
            let (o0, o1, o2, stage, tslot, sub): (u32, u32, u32, u32, usize, bool) = match edi {
                0 => (0x10, 0x14, 0x18, 0x130, 0x13C, false),
                1 => (0x00, 0x04, 0x08, 0x140, 0x14C, false),
                2 => (0x00, 0x04, 0x08, 0x150, 0x15C, true),
                3 => (0x10, 0x14, 0x18, 0x040, 0x04C, true),
                _ => core::hint::unreachable_unchecked(),
            };
            fwr(&mut frame, 0x30, fmul(f32::from_bits(rd32(p1.wrapping_add(o0))), k10));
            fwr(&mut frame, 0x10, fmul(f32::from_bits(rd32(p1.wrapping_add(o1))), k10));
            fwr(&mut frame, 0x28, fmul(f32::from_bits(rd32(p1.wrapping_add(o2))), k10));
            let q = lf_checker_rt::callee_cdecl!(13, u32, fptr.wrapping_add(stage));
            let q0 = f32::from_bits(rd32(q));
            let q1 = f32::from_bits(rd32(q.wrapping_add(4)));
            let q2 = f32::from_bits(rd32(q.wrapping_add(8)));
            let (c0, c1, c2) = if sub {
                (
                    fsub(q1, frd(&frame, 0x10)),
                    fsub(q0, frd(&frame, 0x30)),
                    fsub(q2, frd(&frame, 0x28)),
                )
            } else {
                (
                    fadd(q1, frd(&frame, 0x10)),
                    fadd(frd(&frame, 0x30), q0),
                    fadd(q2, frd(&frame, 0x28)),
                )
            };
            fwr(&mut frame, 0x94, c0);
            let tail = frd(&frame, tslot);
            fwr(&mut frame, 0x90, c1);
            fwr(&mut frame, 0x9C, tail);
            let c2 = fadd(c2, k2);
            fwr(&mut frame, 0x98, c2);
            let setup = rd32(rd32(esi).wrapping_add(VT_SLOT_B));
            let setup_fn: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(setup as usize);
            setup_fn(esi, fptr.wrapping_add(0x60), 0, 0);
            let which = rd32(esi.wrapping_add(OBJ_WHICH));
            wr8(esi.wrapping_add(OBJ_MODE), 2);
            if which == 1 || which == 0 {
                let flag = if rd8(esi.wrapping_add(ELEM_KIND)) & 7 != 0 { 1 } else { 0 };
                if which == 1 {
                    lf_checker_rt::callee_thiscall!(16, u32, esi, flag, FIVE.to_bits(), FIVE.to_bits());
                } else {
                    lf_checker_rt::callee_thiscall!(17, u32, esi, flag, FIVE.to_bits(), FIVE.to_bits());
                }
            }
            let mb = rd32(esi.wrapping_add(OBJ_MATRIX));
            let roww = rd16(esi.wrapping_add(OBJ_ROW)) as u16 as i16 as i32;
            let trow = rd32((lf_checker_rt::relocated(ROW_TABLE) as u32)
                .wrapping_add((roww as u32).wrapping_mul(4)));
            let m = |o: u32| f32::from_bits(rd32(mb.wrapping_add(o)));
            let t34 = f32::from_bits(rd32(trow.wrapping_add(0x34)));
            let t24 = f32::from_bits(rd32(trow.wrapping_add(0x24)));
            let slot4c = frd(&frame, 0x4C);
            let zero = 0.0f32;
            // Block 1: rows scaled by t34, zero terms folded in.
            let b7 = fadd(fadd(fmul(m(0x10), t34), fmul(m(0), zero)), fmul(m(0x20), zero));
            let b7 = fadd(b7, m(0x30));
            let b6 = fadd(fadd(fmul(m(0x14), t34), fmul(m(4), zero)), fmul(m(0x24), zero));
            let b6 = fadd(b6, m(0x34));
            let b5 = fadd(fadd(fmul(m(0x18), t34), fmul(m(8), zero)), fmul(m(0x28), zero));
            let b5 = fadd(b5, m(0x38));
            fwr(&mut frame, 0xBC, slot4c);
            fwr(&mut frame, 0xB0, b7);
            fwr(&mut frame, 0xB4, b6);
            fwr(&mut frame, 0xB8, b5);
            // Block 2: rows scaled by t24, b5 carried forward plus one.
            let c4 = fadd(fadd(fmul(m(0x10), t24), fmul(m(0), zero)), fmul(m(0x20), zero));
            let c4 = fadd(c4, m(0x30));
            let c3 = fadd(fadd(fmul(m(0x14), t24), fmul(m(4), zero)), fmul(m(0x24), zero));
            let c3 = fadd(c3, m(0x34));
            let c2 = fadd(fadd(fmul(m(0x18), t24), fmul(m(8), zero)), fmul(m(0x28), zero));
            let c2 = fadd(c2, m(0x38));
            let b5p = fadd(b5, k1);
            fwr(&mut frame, 0x58, b5p);
            fwr(&mut frame, 0xF0, c4);
            fwr(&mut frame, 0xF4, c3);
            fwr(&mut frame, 0xF8, c2);
            let c2p = fadd(c2, k1);
            fwr(&mut frame, 0xFC, slot4c);
            fwr(&mut frame, 0x50, b7);
            fwr(&mut frame, 0x54, b6);
            fwr(&mut frame, 0x120, c4);
            fwr(&mut frame, 0x124, c3);
            fwr(&mut frame, 0x128, c2p);
            // Block 3: rows scaled by t34 plus two.
            let s3 = fadd(t34, k2);
            let d4 = fadd(fadd(fmul(m(0x10), s3), fmul(m(0), zero)), fmul(m(0x20), zero));
            let d4 = fadd(d4, m(0x30));
            let d2 = fadd(fadd(fmul(m(0x14), s3), fmul(m(4), zero)), fmul(m(0x24), zero));
            let d2 = fadd(d2, m(0x34));
            let d1 = fadd(fadd(fmul(m(0x18), s3), fmul(m(8), zero)), fmul(m(0x28), zero));
            let d1 = fadd(d1, m(0x38));
            fwr(&mut frame, 0xDC, slot4c);
            fwr(&mut frame, 0xD4, d2);
            fwr(&mut frame, 0xD0, d4);
            fwr(&mut frame, 0xD8, d1);
            // Block 4: rows scaled by t24 minus two.
            let s4 = fsub(t24, k2);
            let e4 = fadd(fadd(fmul(m(0x10), s4), fmul(m(0), zero)), fmul(m(0x20), zero));
            let e4 = fadd(e4, m(0x30));
            let e2 = fadd(fadd(fmul(m(0x14), s4), fmul(m(4), zero)), fmul(m(0x24), zero));
            let e2 = fadd(e2, m(0x34));
            let e1 = fadd(fadd(fmul(m(0x18), s4), fmul(m(8), zero)), fmul(m(0x28), zero));
            let e1 = fadd(e1, m(0x38));
            fwr(&mut frame, 0xEC, slot4c);
            fwr(&mut frame, 0xE4, e2);
            fwr(&mut frame, 0xE0, e4);
            fwr(&mut frame, 0xE8, e1);
            // Block 5: low row scaled by two, high rows zeroed.
            let f3 = fadd(fadd(fmul(m(0x10), zero), fmul(m(0), k2)), fmul(m(0x20), zero));
            let f3 = fadd(f3, m(0x30));
            let f2 = fadd(fadd(fmul(m(0x14), zero), fmul(m(4), k2)), fmul(m(0x24), zero));
            let f2 = fadd(f2, m(0x34));
            let f1 = fadd(fadd(fmul(m(0x18), zero), fmul(m(8), k2)), fmul(m(0x28), zero));
            let f1 = fadd(f1, m(0x38));
            fwr(&mut frame, 0xAC, slot4c);
            fwr(&mut frame, 0xA0, f3);
            fwr(&mut frame, 0xA4, f2);
            fwr(&mut frame, 0xA8, f1);
            // Block 6: block 5 with the low terms subtracted.
            let g3 = fadd(fsub(fmul(m(0x10), zero), fmul(m(0), k2)), fmul(m(0x20), zero));
            let g3 = fadd(g3, m(0x30));
            let g2 = fadd(fsub(fmul(m(0x14), zero), fmul(m(4), k2)), fmul(m(0x24), zero));
            let g2 = fadd(g2, m(0x34));
            let g1 = fadd(fsub(fmul(m(0x18), zero), fmul(m(8), k2)), fmul(m(0x28), zero));
            let g1 = fadd(g1, m(0x38));
            fwr(&mut frame, 0xCC, slot4c);
            fwr(&mut frame, 0xC0, g3);
            fwr(&mut frame, 0xC4, g2);
            fwr(&mut frame, 0xC8, g1);
            // Block 7: summed rows, low terms scaled by two.
            let h3 = fadd(fmul(fadd(m(0x10), m(0)), zero), fmul(m(0x20), k2));
            let h3 = fadd(h3, m(0x30));
            let h2 = fadd(fmul(fadd(m(0x14), m(4)), zero), fmul(m(0x24), k2));
            let h2 = fadd(h2, m(0x34));
            let h1 = fadd(fmul(fadd(m(0x18), m(8)), zero), fmul(m(0x28), k2));
            let h1 = fadd(h1, m(0x38));
            fwr(&mut frame, 0x100, h3);
            fwr(&mut frame, 0x104, h2);
            fwr(&mut frame, 0x10C, slot4c);
            fwr(&mut frame, 0x108, h1);
            // Block 8: block 7 with the low terms subtracted.
            let i3 = fsub(fmul(fadd(m(0x10), m(0)), zero), fmul(m(0x20), k2));
            let i3 = fadd(i3, m(0x30));
            let i2 = fsub(fmul(fadd(m(0x14), m(4)), zero), fmul(m(0x24), k2));
            let i2 = fadd(i2, m(0x34));
            let i1 = fsub(fmul(fadd(m(0x18), m(8)), zero), fmul(m(0x28), k2));
            let i1 = fadd(i1, m(0x38));
            fwr(&mut frame, 0x110, i3);
            fwr(&mut frame, 0x114, i2);
            fwr(&mut frame, 0x11C, slot4c);
            fwr(&mut frame, 0x118, i1);
            if edi >= 3 {
                break;
            }
            let eval = |a: u32, b: u32, c: u32, d: u32| {
                lf_checker_rt::callee_cdecl!(18, u32, a, b, c, d, 0, 4)
            };
            if eval(fptr.wrapping_add(0xB0), fptr.wrapping_add(0xF0), esi, 6) & 0xFF == 0 {
                edi = edi.wrapping_add(1);
                if edi < 4 {
                    continue;
                }
                break;
            }
            if eval(fptr.wrapping_add(0x50), fptr.wrapping_add(0x120), 0, 6) & 0xFF == 0 {
                edi = edi.wrapping_add(1);
                if edi < 4 {
                    continue;
                }
                break;
            }
            if eval(fptr.wrapping_add(0xD0), fptr.wrapping_add(0xE0), esi, 0x18) & 0xFF == 0 {
                edi = edi.wrapping_add(1);
                if edi < 4 {
                    continue;
                }
                break;
            }
            if eval(fptr.wrapping_add(0xA0), fptr.wrapping_add(0xC0), esi, 0x18) & 0xFF == 0 {
                edi = edi.wrapping_add(1);
                if edi < 4 {
                    continue;
                }
                break;
            }
            if eval(fptr.wrapping_add(0x100), fptr.wrapping_add(0x110), esi, 0x18) & 0xFF != 0 {
                break;
            }
            edi = edi.wrapping_add(1);
            if edi >= 4 {
                break;
            }
        }
        lf_checker_rt::callee_cdecl!(19, u32, esi);
        lf_checker_rt::callee_cdecl!(20, u32, esi, 0);
        wr32(esi.wrapping_add(OBJ_DONE), 1);
        esi
    }
});
