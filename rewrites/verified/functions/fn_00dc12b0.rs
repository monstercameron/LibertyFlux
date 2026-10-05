// original: 0x00DC12B0 ped_task_fit_candidates_norm (proposed)

/// Fit live task candidates against per-slot anchors, keeping the best.
///
/// `this` is only forwarded to callees. `obj` points at the work block: a
/// flag word at `+0x14` (bit 9 forwarded to the confirm callee), a slot count
/// at `+0x1c` (two more than the outer iterations), a float anchor area at
/// `+0x94` (stepped by 16 bytes per outer slot) and a slot-pointer area at
/// `+0x1E4` (stepped by 4). Shared state is a candidate table (16-byte
/// records), a touch table passed by address, and an init-once flag word.
///
/// Behaviour: after the init flag is set and the count is range-checked
/// (unsigned above 2, then signed above 1 after decrementing), each outer
/// slot loads its entry object (a null entry or a null classify answer skips
/// the slot). Two anchor distances are formed as sums of three squared
/// coordinate differences each; each distance maps to a factor (one over the
/// square-root callee's answer when above a hundredth, else a tenth). When
/// the entry's middle flag bits are set, a touch callee sees each of up to
/// fifteen word codes from the classify answer's table. The fetch callee
/// (cdecl, six arguments, including the entry, a computed anchor and the
/// low byte of a frame temp holding entry bit 1) returns how many table
/// records are live; the frame temp's upper bytes are never written, so the
/// contract defines the stack fill and the rewrite passes the bit alone.
/// Each live record forms the same two distances against the slot anchors;
/// a record worse than the slot on the first distance while worse on the
/// second is dropped, otherwise its factors join the slot's and the combined
/// score must beat the running best (starting at zero). Two gate words from
/// the slot area each guard a get-plus-check call pair, then the confirm
/// callee accepts the record twice (against both anchor pointers). An
/// accepted record becomes the best: its words are copied over the second
/// anchor, the slot distances, factors and best are replaced, and the
/// record index is stamped into the entry's top bits.
///
/// Original: 0x00DC12B0 (thiscall, one stack word; no return value).
lf_checker_rt::export!(thiscall, rw_00dc12b0(this: u32, obj: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x17A1ED0;
        const TOUCH_BASE: u32 = 0x17A6A10;
        const INIT_FLAG: u32 = 0x17A6B10;
        const EPS: u32 = 0xFE8710;
        const FALLBACK: u32 = 0xFE879C;
        const ONE: u32 = 0xFE88E8;
        const CLASSIFY_CALLEE: u32 = 0;
        const SQRT_CALLEE: u32 = 1;
        const TOUCH_CALLEE: u32 = 2;
        const FETCH_CALLEE: u32 = 3;
        const GET_CALLEE: u32 = 4;
        const CHECK_CALLEE: u32 = 5;
        const CONFIRM_CALLEE: u32 = 6;

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
        unsafe fn factor(x: f32, eps: f32, one: f32, fallback: f32) -> f32 {
            unsafe {
                if x > eps {
                    let r: f32 = lf_checker_rt::callee_cdecl!(SQRT_CALLEE, f32, x.to_bits());
                    div(one, r)
                } else {
                    fallback
                }
            }
        }

        let eps = rdf(lf_checker_rt::relocated(EPS));
        let fallback = rdf(lf_checker_rt::relocated(FALLBACK));
        let one = rdf(lf_checker_rt::relocated(ONE));
        let table = lf_checker_rt::relocated(TABLE);
        let touch_base = lf_checker_rt::relocated(TOUCH_BASE);
        let init_flag = lf_checker_rt::relocated(INIT_FLAG);
        let mut n = rd32(obj.wrapping_add(0x1C));
        if n <= 2 {
            return 0;
        }
        let fl = rd32(init_flag);
        if (fl & 1) == 0 {
            wr32(init_flag, fl | 1);
        }
        n = n.wrapping_sub(1);
        if (n as i32) <= 1 {
            return 0;
        }
        n = n.wrapping_sub(1);
        let mut anchor = obj.wrapping_add(0x94);
        let mut slot = obj.wrapping_add(0x1E4);
        let mut outer = n;
        loop {
            let entry_slot = slot.wrapping_add(4);
            let entry = rd32(entry_slot);
            if entry != 0 {
                let cls = (rd32(entry.wrapping_add(4)) >> 0x11) & 0xFFF;
                let handle: u32 = lf_checker_rt::callee_cdecl!(CLASSIFY_CALLEE, u32, cls);
                if handle != 0 {
                    let gate1 = rd32(slot);
                    let gate2 = rd32(slot.wrapping_add(8));
                    let a = anchor;
                    let x1 = sub(rdf(a.wrapping_add(0xC)), rdf(a.wrapping_sub(4)));
                    let mut x5 = sub(rdf(a.wrapping_add(0x10)), rdf(a));
                    let mut x0 = sub(rdf(a.wrapping_add(0x14)), rdf(a.wrapping_add(4)));
                    let mut x4 = sub(rdf(a.wrapping_add(0x20)), rdf(a.wrapping_add(0x10)));
                    let mut x1s = mul(x1, x1);
                    x5 = mul(x5, x5);
                    x5 = add(x5, x1s);
                    x1s = sub(rdf(a.wrapping_add(0x1C)), rdf(a.wrapping_add(0xC)));
                    x0 = mul(x0, x0);
                    x4 = mul(x4, x4);
                    x5 = add(x5, x0);
                    x0 = sub(rdf(a.wrapping_add(0x24)), rdf(a.wrapping_add(0x14)));
                    x1s = mul(x1s, x1s);
                    x4 = add(x4, x1s);
                    x0 = mul(x0, x0);
                    let ptr_c = a.wrapping_add(0x1C);
                    let ptr_b = a.wrapping_sub(4);
                    x4 = add(x4, x0);
                    let mut f5 = factor(x5, eps, one, fallback);
                    let mut f4 = factor(x4, eps, one, fallback);
                    let eflags = rd32(entry);
                    if (eflags & 0x01E0_0000) != 0 {
                        let words = rd32(handle.wrapping_add(0x60));
                        let base_id = rd32(entry.wrapping_add(4)) & 0x1_FFFF;
                        let count = ((eflags >> 0x15) & 0xF) as u32;
                        let mut k: u32 = 0;
                        loop {
                            let code = ((words.wrapping_add(base_id.wrapping_add(k).wrapping_mul(2))) as *const u16).read_unaligned() as u32;
                            let _: u32 = lf_checker_rt::callee_thiscall!(TOUCH_CALLEE, u32, handle, code, touch_base.wrapping_add(k.wrapping_mul(16)));
                            k = k.wrapping_add(1);
                            if !(k < count) {
                                break;
                            }
                        }
                    }
                    let idx = rd32(entry.wrapping_add(4)) & 0x1_FFFF;
                    let abase = rd32(handle.wrapping_add(0x64));
                    let bit = ((rd32(entry) >> 1) & 1) as u32;
                    let fetch_count = lf_checker_rt::callee_cdecl!(FETCH_CALLEE, u32, entry, touch_base, abase.wrapping_add(idx.wrapping_mul(8)), 0, bit, table) as u32;
                    let mut best = 0.0f32;
                    if fetch_count != 0 {
                        let mut j: u32 = 0;
                        loop {
                            let rec = table.wrapping_add(j.wrapping_mul(16));
                            let skip_j = rd32(entry) >> 0x19;
                            if j != skip_j {
                                let mut y1 = sub(rdf(rec), rdf(ptr_b));
                                let mut y5 = sub(rdf(rec.wrapping_add(4)), rdf(ptr_b.wrapping_add(4)));
                                let mut y0 = sub(rdf(rec.wrapping_add(8)), rdf(ptr_b.wrapping_add(8)));
                                y1 = mul(y1, y1);
                                let mut y4 = sub(rdf(ptr_c.wrapping_add(4)), rdf(rec.wrapping_add(4)));
                                y5 = mul(y5, y5);
                                y0 = mul(y0, y0);
                                y5 = add(y5, y1);
                                y1 = sub(rdf(ptr_c), rdf(rec));
                                y4 = mul(y4, y4);
                                y5 = add(y5, y0);
                                y0 = sub(rdf(ptr_c.wrapping_add(8)), rdf(rec.wrapping_add(8)));
                                y1 = mul(y1, y1);
                                y4 = add(y4, y1);
                                y0 = mul(y0, y0);
                                y4 = add(y4, y0);
                                let mut cont = false;
                                if y5 > x5 && y4 > x4 {
                                    cont = true;
                                }
                                if !cont {
                                    let f5i = factor(y5, eps, one, fallback);
                                    let f4i = factor(y4, eps, one, fallback);
                                    let score = sub(add(f4, f5), add(f4i, f5i));
                                    if score > best {
                                        if gate1 != 0 {
                                            let _: u32 = lf_checker_rt::callee_thiscall!(GET_CALLEE, u32, this);
                                            let ok: u32 = lf_checker_rt::callee_thiscall!(CHECK_CALLEE, u32, this, rec, ptr_b, gate1, entry, 0);
                                            if (ok & 0xFF) == 0 {
                                                cont = true;
                                            }
                                        }
                                        if !cont && gate2 != 0 {
                                            let _: u32 = lf_checker_rt::callee_thiscall!(GET_CALLEE, u32, this);
                                            let ok: u32 = lf_checker_rt::callee_thiscall!(CHECK_CALLEE, u32, this, rec, ptr_c, gate2, entry, 0);
                                            if (ok & 0xFF) == 0 {
                                                cont = true;
                                            }
                                        }
                                        if !cont {
                                            let abit = ((rd32(obj.wrapping_add(0x14)) >> 9) & 1) as u32;
                                            let ok1: u32 = lf_checker_rt::callee_thiscall!(CONFIRM_CALLEE, u32, this, rec, ptr_b, abit);
                                            if (ok1 & 0xFF) == 0 {
                                                cont = true;
                                            } else {
                                                let ok2: u32 = lf_checker_rt::callee_thiscall!(CONFIRM_CALLEE, u32, this, rec, ptr_c, abit);
                                                if (ok2 & 0xFF) == 0 {
                                                    cont = true;
                                                }
                                            }
                                        }
                                        if !cont {
                                            best = score;
                                            wr32(a.wrapping_add(0xC), rd32(rec));
                                            wrf(a.wrapping_add(0x10), rdf(rec.wrapping_add(4)));
                                            wrf(a.wrapping_add(0x14), rdf(rec.wrapping_add(8)));
                                            wr32(a.wrapping_add(0x18), rd32(rec.wrapping_add(0xC)));
                                            x5 = y5;
                                            x4 = y4;
                                            f5 = f5i;
                                            f4 = f4i;
                                            wr32(entry, (rd32(entry) & 0x1FF_FFFF) | (j.wrapping_shl(0x19)));
                                        }
                                    }
                                }
                            }
                            j = j.wrapping_add(1);
                            if !(j < fetch_count) {
                                break;
                            }
                        }
                    }
                }
            }
            anchor = anchor.wrapping_add(0x10);
            slot = entry_slot;
            outer = outer.wrapping_sub(1);
            if outer == 0 {
                break;
            }
        }
        0
    }
});
