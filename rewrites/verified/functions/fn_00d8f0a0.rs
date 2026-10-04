// original: 0x00d8f0a0 audio_record_query_closest
//! Scan the record array and keep the closest approach to the target point.
//!
//! For each record whose signed 16-bit bounds contain the query box and that
//! passes the optional filter callback, probes its sample vectors, walks the
//! closing edge then every vertex, interpolates each by the curve parameter
//! and keeps the closest squared distance (with the optional vertical
//! stretch), subject to the gate callee. Returns nothing meaningful (void).
//!
//! Two quirks are replicated exactly: the segment loop visits the closing
//! edge (last vertex to first) before vertices 1.., and the stored vertex id
//! trails the visited vertex by one after the first iteration. The walk guard
//! tests the flags of `and count,0xf` (lea sets no flags), so it runs for
//! any nonzero count, including 1.
export!(thiscall, rw_00d8f0a0(this: u32, query: u32, cb: u32, flags: u32, out: u32) -> u32 {
    unsafe {
        let n = *(this as *const u32).add(0x7C / 4);
        if n == 0 {
            // Falls through with entry EAX still in place; return channel unused.
            return 0;
        }
        let base = *(this as *const u32).add(0x6C / 4);
        let table = *(this as *const u32).add(0x60 / 4);
        let stretch = (flags & 0x80) != 0;
        let two = *global::<f32>(0xFE8A24);
        let mut i = 0u32;
        while i < n {
            let rec = base.wrapping_add(i.wrapping_mul(0x28));
            // Signed 16-bit compares (the original uses jg/jl on ax).
            let r = rec as *const i16;
            let qs = query as *const i16;
            let hit = *r.add(0x10 / 2) <= *qs.add(0x02 / 2)
                && *r.add(0x14 / 2) <= *qs.add(0x06 / 2)
                && *r.add(0x18 / 2) <= *qs.add(0x0A / 2)
                && *r.add(0x12 / 2) >= *qs.add(0x00 / 2)
                && *r.add(0x16 / 2) >= *qs.add(0x04 / 2)
                && *r.add(0x1A / 2) >= *qs.add(0x08 / 2);
            if hit {
                let mut go = true;
                if cb != 0 {
                    let filter: extern "cdecl" fn(u32, u32) -> u32 =
                        core::mem::transmute(cb as usize);
                    if filter(this, rec) & 0xFF == 0 {
                        go = false;
                    }
                }
                if go {
                    let head = *(rec as *const u32);
                    let count = ((head >> 21) & 0xF) as usize;
                    if count != 0 {
                        let rbase = (*(rec as *const u32).add(1) & 0x1FFFF) as usize;
                        let mut j = 0usize;
                        while j < count {
                            let sample = *(((table as u32)
                                .wrapping_add(((rbase + j) * 2) as u32))
                                as *const u16) as u32;
                            let slot = (out as u32).wrapping_add(0x40 + (j * 0x10) as u32);
                            let _: u32 = callee_thiscall!(2, u32, this, sample, slot);
                            j += 1;
                        }
                    }
                    // The loop guard tests the flags of `and count,0xf` (lea sets
                    // no flags), so the walk runs for any nonzero count.
                    if count != 0 {
                        let o = out as *const f32;
                        let mostly = *o.add(0x20 / 4);
                        let mostly1 = *o.add(0x24 / 4);
                        let mostly2 = *o.add(0x28 / 4);
                        let mut m = 0usize;
                        while m < count {
                            let p = (out as u32).wrapping_add(0x48 + (m * 0x10) as u32);
                            let off = if m == 0 { (count - 1) * 0x10 } else { (m - 1) * 0x10 };
                            let bptr = (out as u32).wrapping_add(0x40 + off as u32);
                            let b = bptr as *const f32;
                            let a = p.wrapping_sub(8) as *const f32;
                            let mut d = [0f32; 3];
                            d[0] = *a.add(0) - *b.add(0);
                            d[1] = *a.add(1) - *b.add(1);
                            d[2] = *a.add(2) - *b.add(2);
                            let t: f32 = callee_cdecl!(
                                3, f32, bptr, d.as_mut_ptr() as u32,
                                (out as u32).wrapping_add(0x20)
                            );
                            let p0 = *b.add(0) + d[0] * t;
                            let p1 = *b.add(1) + d[1] * t;
                            let p2 = *b.add(2) + d[2] * t;
                            let r0 = p0 - mostly;
                            let r1 = p1 - mostly1;
                            let mut r2 = p2 - mostly2;
                            if stretch {
                                r2 *= two;
                            }
                            // Squared distance in the original accumulation order.
                            let d2 = (r1 * r1 + r0 * r0) + r2 * r2;
                            let ob = *o.add(0x554 / 4);
                            let ib = *o.add(0x550 / 4);
                            if ob > d2 && ib > d2 {
                                let mut store = true;
                                if *(out as *const u32).add(0x560 / 4) != 0 {
                                    let pt = [p0, p1, p2];
                                    let g: u32 = callee_cdecl!(
                                        4, u32, pt.as_ptr() as u32, out as u32
                                    );
                                    if g & 0xFF == 0 {
                                        store = false;
                                    }
                                }
                                if store {
                                    *(out as *mut u32).add(0x558 / 4) = i;
                                    *(out as *mut f32).add(0x550 / 4) = d2;
                                    *(out as *mut u32).add(0x55C / 4) =
                                        if m == 0 { (count - 1) as u32 } else { (m - 1) as u32 };
                                }
                            }
                            m += 1;
                        }
                    }
                }
            }
            i += 1;
        }
        0
    }
});
