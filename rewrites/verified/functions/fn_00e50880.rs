// original: 0x00E50880 clip_path_resolve_and_bind (proposed)
#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn rd16(a: u32) -> u32 {
    unsafe { (a as *const u16).read_unaligned() as u32 }
}
#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}
#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}
#[inline(always)]
unsafe fn rdf(a: u32) -> f32 {
    unsafe { f32::from_bits(rd32(a)) }
}
#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

const CLIP: u32 = 0x338;
const E0: u32 = 0x1e0;
const E4: u32 = 0x1e4;
const EC_: u32 = 0x1ec;
const E8: u32 = 0x1e8;
const W0: u32 = 0x1f4;
const W1: u32 = 0x1f8;
const W2: u32 = 0x1f0;
const W3: u32 = 0x1fc;
const METER: u32 = 0x200;
const MATCH_ID: u32 = 0x328;
const RATE: u32 = 0x330;
const OUT350: u32 = 0x350;
const LIST: u32 = 0x1e0;
const SUB: u32 = 0x1ec;
const IFC: u32 = 0x1fc;
const OUT20C: u32 = 0x20c;
const DONE: u32 = 0x218;

const V_COUNT: u32 = 0x1d4;
const V_ITEMS: u32 = 0x1d0;
const V_PROBE: u32 = 0x124;
const V_STR: u32 = 0x23c;
const V_MK: u32 = 0x54;
const V_REL: u32 = 0x8;
const V_FIN: u32 = 0x238;
const V_CLR: u32 = 0x218;
const V_OPEN: u32 = 0x1e0;
const V_Q0: u32 = 0x4c;
const V_USE: u32 = 0x1d8;
const V_SETU: u32 = 0x58;
const V_CFG2: u32 = 0x22c;
const V_GAUGE: u32 = 0x98;
const V_SETF: u32 = 0xa0;
const V_NEXT: u32 = 0x224;
const V_GET: u32 = 0x220;
const V_LINK: u32 = 0x1e4;
const V_RUN: u32 = 0x1ac;
const V_GO: u32 = 0x18;
const V_ACT: u32 = 0x120;

const S8C: u32 = 1;
const E40: u32 = 2;
const E30: u32 = 3;
const EDF: u32 = 4;
const M56: u32 = 5;
const B10: u32 = 6;
const B40: u32 = 7;
const D66: u32 = 8;
const F2D: u32 = 9;
const B35: u32 = 10;
const CK: u32 = 11;
const AB: u32 = 12;
const GFN: u32 = 13;
const V23C: u32 = 20;
const V124A: u32 = 21;
const V124B: u32 = 22;
const V124C: u32 = 23;
const V124D: u32 = 24;
const VP: u32 = 25;
const VEDI: u32 = 26;
const VGEN: u32 = 27;
const VADV: u32 = 28;
const V1D0: u32 = 29;
const V8: u32 = 30;
const V238: u32 = 31;
const V218: u32 = 32;
const V1E0: u32 = 33;
const V4C0: u32 = 34;
const V1D8: u32 = 35;
const V58: u32 = 36;
const V22C: u32 = 37;
const V98: u32 = 38;
const VA0: u32 = 39;
const V224: u32 = 40;
const V220: u32 = 41;
const V1E4: u32 = 42;
const V1AC: u32 = 43;
const V18: u32 = 44;
const V120: u32 = 45;
const V4C1: u32 = 46;
const V54: u32 = 47;

const STR_VIDS: u32 = 0x00f1a4c8;
const GSTR: u32 = 0x01168dd8;
const TAGD: u32 = 0x00f1a3c4;
const DTAB: u32 = 0x00fe8f50;
const TWO: u32 = 0x00fe8a24;
const MGR: u32 = 0x01981a4c;

#[inline(always)]
unsafe fn vt0(obj: u32, slot: u32, _id: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(obj)
    }
}
// NOTE: vt0's planted stub is chosen by the object's fabricated table; the
// id argument only documents which callee the contract planted there.
#[inline(always)]
unsafe fn vt1(obj: u32, slot: u32, a: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(obj, a)
    }
}
#[inline(always)]
unsafe fn vt1f(obj: u32, slot: u32, a: f32) -> u32 {
    unsafe { vt1(obj, slot, a.to_bits()) }
}
#[inline(always)]
unsafe fn vt2(obj: u32, slot: u32, a: u32, b: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(obj, a, b)
    }
}
#[inline(always)]
unsafe fn vtgauge(obj: u32) -> f32 {
    unsafe {
        let f: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(rd32(rd32(obj) + V_GAUGE) as usize);
        f(obj)
    }
}
#[inline(always)]
unsafe fn gfcall(buf: u32) -> u32 {
    unsafe {
        let f: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(rd32(lf_checker_rt::relocated(0x00e73268)) as usize);
        f(buf)
    }
}
#[inline(always)]
unsafe fn u32_to_f32(c: u32) -> f32 {
    unsafe {
        let tab = lf_checker_rt::global::<u64>(DTAB);
        let magic = f64::from_bits(
            ((tab as *const u64).add((c >> 31) as usize)).read_unaligned(),
        );
        let d = core::hint::black_box((c as i32) as f64) + core::hint::black_box(magic);
        d as f32
    }
}

/// Resolve a clip path, pick the active source, and bind the clip.
///
/// `this` points to the view object with the current clip at `+0x338`, four
/// candidate sources at `+0x1e0`/`+0x1e4`/`+0x1ec`/`+0x1e8` with weights at
/// `+0x1f4`/`+0x1f8`/`+0x1f0`/`+0x1fc`, a meter at `+0x200`, a match id at
/// `+0x328`, a rate at `+0x330` and an output word at `+0x350`. `flag`'s low
/// byte skips the path block when nonzero. Returns nothing meaningful.
///
/// Algorithm: exit early when there is no clip. Otherwise, unless skipped,
/// build the clip path on the stack (fixed prefix, clip name, retagged
/// suffix), hash it twice through the global hash function, and register the
/// clip when accepted. Probe the four sources in order for the first live
/// one; resolve the clip's peer object and, when the peer count is exactly 1
/// (equality) and the source count exceeds 1 (UNSIGNED `jbe`), drop a
/// duplicate peer. Release the clip, and when the peer count is zero while
/// the source count is exactly 1, run the tail calls and finish. Otherwise
/// walk the source list (count read as an UNSIGNED 16-bit word): entries
/// with a zero id take the defer path (scale rates, run tails, adopt the
/// previous entry's peer, done); entries at or above the match id
/// (UNSIGNED `jae`) are skipped; the rest are opened, linked and configured.
/// A trailing zero id rescales the rates. Finally mark the source done,
/// unless its count exceeds 2 (UNSIGNED `ja`) activate the sub object, adopt
/// the next peer when one resolves, and record completion.
///
/// Edge cases: a null peer anywhere faults on the next virtual call, exactly
/// like the original. The overlong-path abort is never taken here (inputs
/// keep paths short); its stub stays exempt. Float minima take the new value
/// on unordered comparison, matching `comiss`/`ja`.
///
/// Original: 0x00E50880 (thiscall, one stack word, no meaningful return).
/// `tail_signed` selects the wrong-version hook: false is the original's
/// UNSIGNED `ja`, true is a signed comparison.
unsafe fn run(this: u32, flag: u32, tail_signed: bool) -> u32 {
    unsafe {
        let clip0 = rd32(this + CLIP);
        if clip0 == 0 {
            lf_checker_rt::callee_cdecl!(CK, u32,);
            return 0;
        }
        let mut clip = clip0;
        if (flag & 0xff) == 0 {
            lf_checker_rt::callee_cdecl!(
                S8C,
                u32,
                lf_checker_rt::relocated(STR_VIDS),
                0
            );
            clip = rd32(this + CLIP);
            let ebp = vt0(clip, V_STR, V23C);
            let mut buf = [0u8; 0x220];
            let mut i = 0usize;
            loop {
                let b = rd8(lf_checker_rt::relocated(GSTR).wrapping_add(i as u32));
                buf[i] = b;
                i += 1;
                if b == 0 {
                    break;
                }
            }
            let len1 = i - 1;
            let mut len2 = 0usize;
            while rd8(ebp.wrapping_add(len2 as u32)) != 0 {
                len2 += 1;
            }
            let total = len2 + 1;
            let mut k = 0usize;
            while k < total {
                buf[len1 + k] = rd8(ebp.wrapping_add(k as u32));
                k += 1;
            }
            let bufptr = buf.as_mut_ptr() as u32;
            gfcall(bufptr);
            let mut len = 0usize;
            while buf[len] != 0 {
                len += 1;
            }
            let chk = (len as u32).wrapping_sub(3);
            if chk >= 0x200 {
                lf_checker_rt::callee_cdecl!(AB, u32,);
                let _ = rd32(0);
                return 0;
            }
            buf[len.wrapping_sub(3)] = 0;
            let mut lenb = 0usize;
            while buf[lenb] != 0 {
                lenb += 1;
            }
            wr32(bufptr.wrapping_add(lenb as u32), rd32(lf_checker_rt::relocated(TAGD)));
            gfcall(bufptr);
            let ok = lf_checker_rt::callee_thiscall!(E40, u32, this, ebp);
            if (ok & 0xff) != 0 {
                lf_checker_rt::callee_thiscall!(E30, u32, this, ebp);
                lf_checker_rt::callee_thiscall!(EDF, u32, this, ebp);
            }
        }
        // Probe the four sources for the first live one.
        let mut edi: u32 = 0;
        let mut wval: u32 = 0;
        let c0 = rd32(this + E0);
        if (vt0(c0, V_PROBE, V124A) & 0xff) != 0 {
            wval = rd32(this + W0);
            edi = c0;
        } else {
            let c1 = rd32(this + E4);
            if (vt0(c1, V_PROBE, V124B) & 0xff) != 0 {
                wval = rd32(this + W1);
                edi = c1;
            } else {
                let c2 = rd32(this + EC_);
                if (vt0(c2, V_PROBE, V124C) & 0xff) != 0 {
                    wval = rd32(this + W2);
                    edi = c2;
                } else {
                    let c3 = rd32(this + E8);
                    if (vt0(c3, V_PROBE, V124D) & 0xff) != 0 {
                        wval = rd32(this + W3);
                        edi = c3;
                    }
                }
            }
        }
        clip = rd32(this + CLIP);
        let r = vt0(clip, V_MK, V54);
        let p = lf_checker_rt::callee_thiscall!(M56, u32, lf_checker_rt::relocated(MGR), r);
        let vp = vt0(p, V_COUNT, VP);
        if vp == 1 {
            let ve = vt0(edi, V_COUNT, VEDI);
            if ve > 1 {
                lf_checker_rt::callee_thiscall!(B10, u32, edi);
                let list = rd32(edi + LIST);
                let p1 = vt0(list, V_ITEMS, V1D0);
                let mut esi = rd32(p1);
                let p2 = vt0(list, V_ITEMS, V1D0);
                let end = rd32(p2).wrapping_add(rd16(p2 + 4).wrapping_mul(4));
                if esi != end {
                    let mut idx: i32 = -1;
                    loop {
                        if rd32(esi) == p {
                            lf_checker_rt::callee_thiscall!(F2D, u32, edi);
                            lf_checker_rt::callee_thiscall!(B40, u32, edi, 0, 1);
                            lf_checker_rt::callee_thiscall!(B40, u32, edi, idx as u32, 1);
                        }
                        let p3 = vt0(list, V_ITEMS, V1D0);
                        let e3 = rd32(p3).wrapping_add(rd16(p3 + 4).wrapping_mul(4));
                        idx = idx.wrapping_add(1);
                        esi = esi.wrapping_add(4);
                        if esi == e3 {
                            break;
                        }
                    }
                }
            }
        }
        clip = rd32(this + CLIP);
        if clip != 0 {
            vt1(clip, V_REL, 1);
        }
        wr32(this + CLIP, 0);
        vt0(p, V_FIN, V238);
        vt1(p, V_CLR, 0);
        let vp2 = vt0(p, V_COUNT, VP);
        if vp2 != 0 {
            // continue to main walk
        } else {
            let ve2 = vt0(edi, V_COUNT, VEDI);
            if ve2 == 1 {
                vt1(p, V_REL, 1);
                lf_checker_rt::callee_thiscall!(F2D, u32, edi);
                lf_checker_rt::callee_thiscall!(B35, u32, wval);
                lf_checker_rt::callee_cdecl!(CK, u32,);
                return 0;
            }
        }
        // Main walk over the source list.
        let mut counter: u32 = 0;
        let list = rd32(edi + LIST);
        let q1 = vt0(list, V_ITEMS, V1D0);
        let mut cursor = rd32(q1);
        let q2 = vt0(list, V_ITEMS, V1D0);
        let mut end = rd32(q2).wrapping_add(rd16(q2 + 4).wrapping_mul(4));
        let mut next = cursor.wrapping_add(4);
        if cursor != end {
            loop {
                let item = rd32(cursor);
                let v1 = vt0(item, V_COUNT, VGEN);
                if v1 == 0 {
                    // Defer path: scale, run tails, adopt previous peer, done.
                    vt0(edi, V_COUNT, VEDI);
                    let prev = rd32(cursor.wrapping_sub(4));
                    vt1(item, V_REL, 1);
                    let c = vt0(edi, V_COUNT, VEDI);
                    scale_block(this, wval, c);
                    let c2 = rd32(edi + IFC);
                    wr32(edi + OUT20C, fmul(u32_to_f32(c2), rdf(this + RATE)).to_bits());
                    lf_checker_rt::callee_thiscall!(F2D, u32, edi);
                    lf_checker_rt::callee_thiscall!(B40, u32, edi, 0, 1);
                    lf_checker_rt::callee_thiscall!(
                        B40,
                        u32,
                        edi,
                        counter.wrapping_sub(1),
                        1
                    );
                    if prev != 0 {
                        let g = vt0(prev, V_GET, V220);
                        wr32(this + CLIP, g);
                    }
                    break;
                }
                let v2 = vt0(item, V_COUNT, VGEN);
                if v2 >= rd32(this + MATCH_ID) {
                    counter = counter.wrapping_add(1);
                } else {
                    // Open, link and configure the next entry.
                    let q3 = vt0(list, V_ITEMS, V1D0);
                    let end2 = rd32(q3).wrapping_add(rd16(q3 + 4).wrapping_mul(4));
                    if next != end2 {
                        let nxt = rd32(next);
                        let v3 = vt0(nxt, V_COUNT, VGEN);
                        if v3 != 0 {
                            let ebp3 = nxt;
                            let a1 = vt1(ebp3, V_OPEN, 0);
                            wr32(this + CLIP, a1);
                            let a2 = vt0(a1, V_Q0, V4C0);
                            vt1(item, V_USE, a2);
                            let a3 = vt0(item, V_Q0, V4C0);
                            vt1(a1, V_SETU, a3);
                            if ebp3 == p {
                                vt2(ebp3, V_CFG2, 0, 1);
                            }
                            let t = vt0(ebp3, V_ITEMS, V1D0);
                            lf_checker_rt::callee_thiscall!(D66, u32, t, 0);
                            vt0(ebp3, V_FIN, V238);
                            vt0(item, V_FIN, V238);
                        }
                    }
                    let q4 = vt0(list, V_ITEMS, V1D0);
                    let end3 = rd32(q4).wrapping_add(rd16(q4 + 4).wrapping_mul(4));
                    if next != end3 {
                        let ebp3b = rd32(next);
                        let v4 = vt0(ebp3b, V_COUNT, VADV);
                        if v4 == 0 {
                            vt1(ebp3b, V_REL, 1);
                            let c = vt0(edi, V_COUNT, VEDI);
                            scale_block(this, wval, c);
                            let c2 = rd32(edi + IFC);
                            wr32(edi + OUT20C, fmul(u32_to_f32(c2), rdf(this + RATE)).to_bits());
                            lf_checker_rt::callee_thiscall!(F2D, u32, edi);
                            lf_checker_rt::callee_thiscall!(B40, u32, edi, 0, 1);
                            lf_checker_rt::callee_thiscall!(B40, u32, edi, counter, 1);
                        }
                    }
                }
                cursor = cursor.wrapping_add(4);
                next = next.wrapping_add(4);
                let q5 = vt0(list, V_ITEMS, V1D0);
                end = rd32(q5).wrapping_add(rd16(q5 + 4).wrapping_mul(4));
                if cursor == end {
                    break;
                }
            }
        }
        // Tail.
        wr8(edi + DONE, 1);
        let v5 = vt0(edi, V_COUNT, VEDI);
        let skip: bool = if tail_signed {
            (v5 as i32) <= 2
        } else {
            v5 <= 2
        };
        if skip {
            let sub = rd32(edi + SUB);
            vt1(sub, V_ACT, 0);
        }
        let f = vt0(list, V_NEXT, V224);
        if f != 0 {
            let g = vt0(f, V_GET, V220);
            wr32(this + CLIP, g);
            if g != 0 {
                let h = vt1(g, V_Q0, 1);
                let h2 = vt1(f, V_LINK, h);
                vt1(f, V_CFG2, h2);
                let gb = rd32(this + CLIP);
                vt0(gb, V_RUN, V1AC);
                vt1(gb, V_GO, 1);
            }
        }
        wr32(this + OUT350, 8);
        lf_checker_rt::callee_cdecl!(CK, u32,);
        0
    }
}

/// Scale the meter rates from an integer count: convert through the double
/// table, scale by the rate, add two gauge readings in order, keep the
/// minimum, and store it to the weight object.
unsafe fn scale_block(this: u32, wval: u32, c: u32) {
    unsafe {
        let meter = rd32(this + METER);
        let rate = rdf(this + RATE);
        let mut sum = fmul(u32_to_f32(c), rate);
        let g1 = vtgauge(meter);
        sum = fadd(sum, g1);
        let g2 = vtgauge(meter);
        let t = fmul(rate, f32::from_bits(rd32(lf_checker_rt::relocated(TWO))));
        let u = fadd(g2, t);
        if !(u > sum) {
            sum = u;
        }
        vt1f(wval, V_SETF, sum);
    }
}

lf_checker_rt::export!(thiscall, rw_00e50880(this: u32, flag: u32) -> u32 {
    unsafe { run(this, flag, false) }
});
