// original: 0x00c91b60 task_list_update_c91b60 (proposed)

/// Scan a ped's task list for four task-type keys and run the matching
/// update path: gated member calls, two vtable dispatches, float blending
/// and a fade-timer update.
///
/// `thiscall(this)`, no stack arguments. `this+0x40` points at the ped
/// object, whose `+0xb30` entry leads to the data read throughout and whose
/// `+0x224`/`+0x2e0` chain heads the task list. Returns an `eax` value that
/// depends on the exit path (a pointer, a scripted callee answer, or 0).
///
/// Early gates: callee 1 (`thiscall(this)`) must answer nonzero with a live
/// chain through `+0x6c`, callee 2 (`thiscall(seg_b, seg_a)`) nonzero,
/// dword `+0x1304` equal to 1. When global `G_MODE` equals 1, region A runs:
/// callee 3 (`cdecl`, two relocated constants), callee 4
/// (`thiscall(seg_y, r3)`), two indirect calls through the object found in
/// the global table `G_TABLE` at the sign-extended word `+0x2e` (vtable
/// slot `+0x38`, arguments `0x12`/`0x13`), each followed by callee 6, with
/// two single-precision blend blocks over `[edi+0x58]`, the `+0x20` object
/// and global `G_SCALE`, accumulated into the callee-6 answer's `+0x30`
/// words. Float operation order is the original's, pinned.
///
/// The task list is then scanned four times for keys `0x2c5`, `0x419`,
/// `0x347`, `0x640`, setting `esp16`/`bh`/`bl`; all clear exits. Each scan
/// aborts its walk when the 3-bit field at node `+0x8` increases between
/// consecutive nodes, before comparing the key. Region B
/// runs callee 7 (`cdecl`, `[seg_o+0xc4]`), callee 8 (must answer 0),
/// callee 9 twice, and callee 10. Region C maintains the `G_FADE` float
/// against `G_STEP` (clamped at zero from below, `bl` set while positive)
/// unless `bl` forces the `0.25f` reset. Region D resolves two slots from
/// `seg_o+0xcc` (either `-1` exits), runs callees 11 and 12 around them,
/// and conditionally runs callee 13 twice with 16 bytes copied from
/// `G_VEC1`/`G_VEC2` into stack buffers passed by address.
///
/// Narrowings: frame-pointer arguments are address-skipped (`id12:1`,
/// `id13:1,2`; `id13:1` is content-compared with a 4-word snapshot, whose
/// last word is uninitialised scratch matched to the zero stack fill), the
/// `id11` frame-pointer `ecx` is uncompared, and the `[esp+0x18]` scratch
/// byte the original reads uninitialised is matched to the zero fill.
lf_checker_rt::export!(thiscall, rw_00c91b60(this: u32) -> u32 {
    unsafe {
        const G_MODE: u32 = 0x011D_6FD4;
        const G_TABLE: u32 = 0x0129_5CD8;
        const G_FADE: u32 = 0x0171_BB68;
        const G_STEP: u32 = 0x0117_35BC;
        const G_SCALE: u32 = 0x0105_0C70;
        const G_VEC1: u32 = 0x0105_0BF0;
        const G_VEC2: u32 = 0x0105_0C00;
        const PUSH_A: u32 = 0x00ED_6E8C;
        const PUSH_B: u32 = 0x00ED_6E94;
        const PUSH_C: u32 = 0x0111_0090;
        const VT_SLOT: u32 = 0x38;
        const FADE_RESET: u32 = 0x3E80_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Sign-extended word load (movsx), as a u32.
        #[inline(always)]
        unsafe fn movsx_w(a: u32) -> u32 {
            unsafe { (rd16(a) as i16) as i32 as u32 }
        }
        /// Indirect call through slot +0x38 of the object at `obj`.
        #[inline(always)]
        unsafe fn vt_call(obj: u32, arg: u32) -> u32 {
            unsafe {
                let vtable = rd32(obj);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vtable.wrapping_add(VT_SLOT)) as usize);
                f(obj, arg)
            }
        }

        let table = lf_checker_rt::relocated(G_TABLE);
        let r1: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
        let seg_a = rd32(this.wrapping_add(0x40));
        if (r1 & 0xFF) != 0
            && rd8(seg_a.wrapping_add(0x219)) != 0
        {
            let p = rd32(seg_a.wrapping_add(0x6C));
            if p != 0 && (rd8(p.wrapping_add(0x820)) & 4) != 0 {
                return p;
            }
        }
        let seg_b = rd32(seg_a.wrapping_add(0xB30));
        if seg_b == 0 {
            return seg_a;
        }
        let r2: u32 = lf_checker_rt::callee_thiscall!(2, u32, seg_b, seg_a);
        if (r2 & 0xFF) == 0 {
            return r2;
        }
        let seg_b = rd32(seg_a.wrapping_add(0xB30));
        if rd32(seg_b.wrapping_add(0x1304)) != 1 {
            return seg_b;
        }

        // Region A (optional): gated member calls, vtable dispatches, blends.
        let mut eaxv = seg_b;
        if rd32(lf_checker_rt::relocated(G_MODE)) == 1 {
            let r3: u32 = lf_checker_rt::callee_cdecl!(
                3, u32,
                lf_checker_rt::relocated(PUSH_B),
                lf_checker_rt::relocated(PUSH_A)
            );
            eaxv = r3;
            if r3 != 0 {
                let seg_y = rd32(seg_a.wrapping_add(0x78));
                let r4: u32 = lf_checker_rt::callee_thiscall!(4, u32, seg_y, r3);
                eaxv = r4;
                if r4 != 0 {
                    let edi = r4;
                    let idx = movsx_w(seg_a.wrapping_add(0x2E));
                    let seg_o = rd32(table.wrapping_add(idx.wrapping_mul(4)));
                    let r5a = vt_call(seg_o, 0x12);
                    let seg_w: u32 = lf_checker_rt::callee_thiscall!(6, u32, seg_a, r5a);
                    let seg_v = rd32(rd32(seg_a.wrapping_add(0xB30)).wrapping_add(0x20));
                    // Blend block 1.
                    let f_edi = rdf(edi.wrapping_add(0x58));
                    let g = rdf(lf_checker_rt::relocated(G_SCALE));
                    let mut x1 = fmul(f_edi, rdf(seg_v.wrapping_add(0x20)));
                    let mut x2 = fmul(rdf(seg_v.wrapping_add(0x24)), f_edi);
                    let mut x3 = fmul(rdf(seg_v.wrapping_add(0x28)), f_edi);
                    x2 = fmul(x2, g);
                    x1 = fmul(x1, g);
                    x3 = fmul(x3, g);
                    let mut x0 = rdf(seg_w.wrapping_add(0x34));
                    x1 = fadd(x1, rdf(seg_w.wrapping_add(0x30)));
                    x0 = fadd(x0, x2);
                    wr32(seg_w.wrapping_add(0x30), x1.to_bits());
                    wr32(seg_w.wrapping_add(0x34), x0.to_bits());
                    x0 = rdf(seg_w.wrapping_add(0x38));
                    x0 = fadd(x0, x3);
                    wr32(seg_w.wrapping_add(0x38), x0.to_bits());
                    // Second dispatch and blend block 2 (same table word).
                    let idx2 = movsx_w(seg_a.wrapping_add(0x2E));
                    let seg_o2 = rd32(table.wrapping_add(idx2.wrapping_mul(4)));
                    let r5b = vt_call(seg_o2, 0x13);
                    let seg_w2: u32 = lf_checker_rt::callee_thiscall!(6, u32, seg_a, r5b);
                    let f_edi = rdf(edi.wrapping_add(0x58));
                    let g = rdf(lf_checker_rt::relocated(G_SCALE));
                    let mut y1 = fmul(rdf(seg_v.wrapping_add(0x20)), f_edi);
                    let mut y2 = fmul(rdf(seg_v.wrapping_add(0x24)), f_edi);
                    let mut y3 = fmul(rdf(seg_v.wrapping_add(0x28)), f_edi);
                    y1 = fmul(y1, g);
                    y2 = fmul(y2, g);
                    y1 = fadd(y1, rdf(seg_w2.wrapping_add(0x30)));
                    y3 = fmul(y3, g);
                    y2 = fadd(y2, rdf(seg_w2.wrapping_add(0x34)));
                    y3 = fadd(y3, rdf(seg_w2.wrapping_add(0x38)));
                    wr32(seg_w2.wrapping_add(0x30), y1.to_bits());
                    wr32(seg_w2.wrapping_add(0x34), y2.to_bits());
                    wr32(seg_w2.wrapping_add(0x38), y3.to_bits());
                    eaxv = seg_w2;
                }
            }
        }

        // Task-list scans: one walk per key over the same list. Each scan
        // loads the previous 3-bit value once, then per node loads the
        // current one into eax, ends the scan when the previous value is
        // below the current one (an increasing run aborts the walk before
        // the key compare), and otherwise checks the key; eax ends as the
        // last examined node's 3-bit value either way.
        let head = rd32(rd32(seg_a.wrapping_add(0x224)).wrapping_add(0x2E0));
        let mut esp16: u8 = 0;
        let mut node = head;
        if node != 0 {
            let mut prev = (rd32(node.wrapping_add(8)) >> 1) & 7;
            loop {
                eaxv = (rd32(node.wrapping_add(8)) >> 1) & 7;
                if prev < eaxv {
                    break;
                }
                if rd32(node.wrapping_add(4)) == 0x2C5 {
                    esp16 = 1;
                    break;
                }
                prev = eaxv;
                node = rd32(node.wrapping_add(12));
                if node == 0 {
                    break;
                }
            }
        }
        let mut bh: u8 = 0;
        let mut skip_scan3 = false;
        node = head;
        if node != 0 {
            let mut prev = (rd32(node.wrapping_add(8)) >> 1) & 7;
            loop {
                eaxv = (rd32(node.wrapping_add(8)) >> 1) & 7;
                if prev < eaxv {
                    break;
                }
                if rd32(node.wrapping_add(4)) == 0x419 {
                    bh = 1;
                    skip_scan3 = true;
                    break;
                }
                prev = eaxv;
                node = rd32(node.wrapping_add(12));
                if node == 0 {
                    break;
                }
            }
        }
        if !skip_scan3 {
            node = head;
            if node != 0 {
                let mut prev = (rd32(node.wrapping_add(8)) >> 1) & 7;
                loop {
                    eaxv = (rd32(node.wrapping_add(8)) >> 1) & 7;
                    if prev < eaxv {
                        break;
                    }
                    if rd32(node.wrapping_add(4)) == 0x347 {
                        bh = 1;
                        break;
                    }
                    prev = eaxv;
                    node = rd32(node.wrapping_add(12));
                    if node == 0 {
                        break;
                    }
                }
            }
        }
        let mut bl: u8 = 0;
        node = head;
        if node != 0 {
            let mut prev = (rd32(node.wrapping_add(8)) >> 1) & 7;
            loop {
                eaxv = (rd32(node.wrapping_add(8)) >> 1) & 7;
                if prev < eaxv {
                    break;
                }
                if rd32(node.wrapping_add(4)) == 0x640 {
                    bl = 1;
                    break;
                }
                prev = eaxv;
                node = rd32(node.wrapping_add(12));
                if node == 0 {
                    break;
                }
            }
        }
        if esp16 == 0 && bh == 0 && bl == 0 {
            return eaxv;
        }

        // Region B.
        let seg_b = rd32(seg_a.wrapping_add(0xB30));
        let idx_b = movsx_w(seg_b.wrapping_add(0x2E));
        let seg_o_b = rd32(table.wrapping_add(idx_b.wrapping_mul(4)));
        let r7: u32 = lf_checker_rt::callee_cdecl!(7, u32, rd32(seg_o_b.wrapping_add(0xC4)));
        if r7 == 0 {
            return 0;
        }
        let seg_y = rd32(seg_a.wrapping_add(0x78));
        let r8: u32 = lf_checker_rt::callee_thiscall!(8, u32, seg_y, 0x4000000, 1);
        if r8 != 0 {
            return r8;
        }
        let _r9a: u32 = lf_checker_rt::callee_thiscall!(9, u32, seg_y, 0x80000, 1);
        let r9b: u32 = lf_checker_rt::callee_thiscall!(9, u32, seg_y, 0x800000, 1);
        // [esp+0x18] low byte: uninitialised scratch (0 under the zero fill),
        // forced to 1 when the pointed-to dword differs from 0x187.
        let mut esp18: u8 = 0;
        if r9b != 0 && rd32(r9b.wrapping_add(0xC)) != 0x187 {
            esp18 = 1;
        }
        let edi = seg_a;
        let esp1c: u8 = if esp16 != 0 { 0 } else { 1 };
        let r10: u32 = lf_checker_rt::callee_thiscall!(10, u32, edi);
        if (r10 & 0xFF) != 0 && (rd32(lf_checker_rt::relocated(G_MODE)) as i32) >= 1 {
            // Region C: fade-timer maintenance.
            if bl != 0 {
                wr32(lf_checker_rt::relocated(G_FADE), FADE_RESET);
            } else {
                let d = fsub(
                    rdf(lf_checker_rt::relocated(G_FADE)),
                    rdf(lf_checker_rt::relocated(G_STEP)),
                );
                let m = if d > 0.0 { d } else { 0.0 };
                wr32(lf_checker_rt::relocated(G_FADE), m.to_bits());
                bl = u8::from(m > 0.0);
            }
        }

        // Region D.
        if esp18 != 0 || bh != 0 || bl != 0 {
            bl = 0;
        } else {
            bl = 1;
        }
        let seg_b = rd32(edi.wrapping_add(0xB30));
        let idx_d = movsx_w(seg_b.wrapping_add(0x2E));
        let seg_o_d = rd32(table.wrapping_add(idx_d.wrapping_mul(4)));
        let seg_p = rd32(seg_o_d.wrapping_add(0xCC));
        let mut frame_out = [0u32; 4];
        let edi_v = rd32(seg_p.wrapping_add(0x98));
        if edi_v == 0xFFFF_FFFF {
            return seg_p;
        }
        // Callee 11 really cleans nothing (cdecl): the original's later
        // [esp+X] reads prove the stub must leave esp alone.
        let _: u32 =
            lf_checker_rt::callee_cdecl!(11, u32, lf_checker_rt::relocated(PUSH_C));
        let _: u32 = lf_checker_rt::callee_thiscall!(
            12, u32,
            seg_b,
            edi_v,
            frame_out.as_mut_ptr() as u32
        );
        if esp1c != 0 {
            let g = lf_checker_rt::relocated(G_VEC1);
            let mut buf = [0u32; 5];
            buf[0] = rd32(g);
            buf[1] = rd32(g.wrapping_add(4));
            buf[2] = rd32(g.wrapping_add(8));
            buf[3] = rd32(g.wrapping_add(12));
            // buf[4] stays 0: the original's 4th snapshot word is
            // uninitialised scratch (zero under the fill).
            let buf2 = [0u32; 4];
            let _: u32 = lf_checker_rt::callee_thiscall!(
                13, u32,
                this,
                1,
                (&buf[1] as *const u32) as u32,
                buf2.as_ptr() as u32
            );
        }
        let seg_b = rd32(seg_a.wrapping_add(0xB30));
        let idx_d2 = movsx_w(seg_b.wrapping_add(0x2E));
        let seg_o_d2 = rd32(table.wrapping_add(idx_d2.wrapping_mul(4)));
        let seg_p2 = rd32(seg_o_d2.wrapping_add(0xCC));
        let edi_v2 = rd32(seg_p2.wrapping_add(0x94));
        if edi_v2 == 0xFFFF_FFFF {
            return seg_p2;
        }
        let _: u32 =
            lf_checker_rt::callee_cdecl!(11, u32, lf_checker_rt::relocated(PUSH_C));
        let r12b: u32 = lf_checker_rt::callee_thiscall!(
            12, u32,
            seg_b,
            edi_v2,
            frame_out.as_mut_ptr() as u32
        );
        if bl == 0 {
            return r12b;
        }
        let g = lf_checker_rt::relocated(G_VEC2);
        let mut buf = [0u32; 5];
        buf[0] = rd32(g);
        buf[1] = rd32(g.wrapping_add(4));
        buf[2] = rd32(g.wrapping_add(8));
        buf[3] = rd32(g.wrapping_add(12));
        let buf2 = [0u32; 4];
        let r13b: u32 = lf_checker_rt::callee_thiscall!(
            13, u32,
            this,
            0,
            (&buf[1] as *const u32) as u32,
            buf2.as_ptr() as u32
        );
        r13b
    }
});
