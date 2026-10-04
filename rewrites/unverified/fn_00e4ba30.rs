// original: 0x00e4ba30 export_gallery_low (proposed)

/// Build the gallery screen for a low-detail video export: find the newest
/// rendered clip, assemble the preview, viewer, title and resolution panels
/// for it, and pick the matching export profile.
///
/// `this` is the gallery object. The stem buffer at `+0x209` and the index
/// at `+0x430` are filled by the clip search (callee 1); the object slots
/// at `+0x1e0`..`+0x1f4` receive five heap panels; the word/dword pairs at
/// `+0x43c`..`+0x44c` tune the title truncation.
///
/// Behaviour in order: if the readiness check (slot `+0x140` of this)
/// reports ready, return its answer at once. Otherwise run the clip
/// search, allocate the background panel (slot `+0x1f0`, built by callee 3
/// over the global base directory), and derive three float ratios from the
/// size globals (selected by six tester calls, callee 4). Configure the
/// background (callee 5) and emit three setting triples into it; scale the
/// first ratio and hand it to slot `+0x94`; mark the panel current (slot
/// `+0x28`) and attach this gallery's previewer handle (slot `+0x4c` of
/// this into slot `+0x17c`). Repeat the allocate-configure-emit-attach
/// cycle for the previewer panel (slot `+0x1e4`, four triples, callee 9
/// binds the clip stem) and the viewer panel (slot `+0x1e0`, callee 11
/// picks an aspect ladder entry, callee 12 binds the base directory copy,
/// four triples, slots `+0x18`/`+0x28`/`+0x24`). Build the title string
/// from the stem (callee 13), truncate it against the three word/dword
/// pairs (callee 14), build the title panel (slot `+0x1ec`, callee 16
/// binds the second ratio) and the resolution panel (slot `+0x1f4) with
/// three triples each. Compare the stem against the empty reference: on a
/// mismatch ask the profile matcher (callee 17) and append the LOW, MED or
/// HI export tag (callee 18). Release both title strings (callee 19),
/// rebind the stem slot (callee 20), notify this (slots `+0x18`, `+0x13c`)
/// and return the last notification's answer.
///
/// Floats are bit-exact: every division and multiplication runs in the
/// original's operand order, and each `comiss`+branch pair keeps the
/// original's ordered/unordered sense (a quiet NaN takes every `jbe` edge
/// and no `ja` edge). The aspect ladder matches the ratio against the
/// 16:9, 4:3, 5:3, 16:10 and 5:4 centres within 0.01.
///
/// Original: 0x00e4ba30 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00e4ba30(this: u32) -> u32 {
    unsafe {
        const STEM: u32 = 0x209;
        const CLIP_INDEX: u32 = 0x430;
        const PANEL_BG: u32 = 0x1f0;
        const PANEL_PREVIEW: u32 = 0x1e4;
        const PANEL_VIEWER: u32 = 0x1e0;
        const PANEL_TITLE: u32 = 0x1ec;
        const PANEL_RES: u32 = 0x1f4;
        const STEM_SLOT: u32 = 0x1f8;
        const OBJ_CURRENT: u32 = 0x1d8;
        const OBJ_MODE: u32 = 0x354;
        const BASE_DIR: u32 = 0x0116_8dd8;
        const SIZE_A: u32 = 0x0105_c880;
        const SIZE_A_ALT: u32 = 0x0105_c87c;
        const SIZE_B: u32 = 0x0105_c884;
        const SIZE_B_ALT: u32 = 0x0105_c888;
        const WIDE_FLAG: u32 = 0x0118_dc44;
        const REF_W: u32 = 0x0105_96a8;
        const REF_H: u32 = 0x0105_96a4;
        const ZERO_GAP: u32 = 0x017a_cc7c;
        const RATIO_16_9: u32 = 0x00fe_89b8;
        const K_0_5625: u32 = 0x00fe_8848;
        const K_734_4: u32 = 0x00f1_86ec;
        const SIGN_MASK: u32 = 0x8000_0000;
        const GAL_BG: u32 = 0x00f1_7b08;
        const GAL_PREVIEWER: u32 = 0x00f1_7b5c;
        const GAL_VIEWER: u32 = 0x00f1_7be0;
        const VIDEOS_RENDERED: u32 = 0x00f1_84a4;
        const TAG_SUFFIX: u32 = 0x00f1_7c0c;
        const GAL_TITLE: u32 = 0x00f1_7c9c;
        const GAL_RES: u32 = 0x00f1_7f30;
        const CTX_A: u32 = 0x00f1_783f;
        const CTX_B: u32 = 0x00f1_7847;
        const REF_EMPTY: u32 = 0x00f1_799e;
        const STEM_KEY: u32 = 0x00f1_799f;
        const TAG_LOW: u32 = 0x00f1_7dfc;
        const TAG_MED: u32 = 0x00f1_7e34;
        const TAG_HI: u32 = 0x00f1_7e6c;
        const VT_READY: u32 = 0x140;
        const VT_BACKEND: u32 = 0x48;
        const VT_HANDLE: u32 = 0x4c;
        const VT_NOTIFY: u32 = 0x18;
        const VT_DONE: u32 = 0x13c;
        const VT_EMIT: u32 = 0x100;
        const VT_ATTACH: u32 = 0x17c;
        const VT_BIND4: u32 = 0x1cc;
        const VT_SCALE: u32 = 0x94;
        const VT_CURRENT: u32 = 0x28;
        const VT_EXTRA: u32 = 0x24;
        const VT_FLAG: u32 = 0x1fc;
        const VT_TITLE: u32 = 0x1e0;
        const VT_RES: u32 = 0x1d4;
        const VT_FIN: u32 = 0x1f8;
        const VT_SEAL: u32 = 0x200;
        const WORD_PAIRS: [(u32, u32); 3] =
            [(0x43c, 0x438), (0x444, 0x440), (0x44c, 0x448)];
        const DEFAULT_ID: u32 = 0xfc9c85;

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
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// `comiss a, b; jbe`: taken when a <= b or either is NaN.
        #[inline(always)]
        fn below_eq(a: f32, b: f32) -> bool {
            !(a > b)
        }
        /// Absolute value by the original's comiss/xorps idiom.
        #[inline(always)]
        fn abs_if_below(d: f32) -> f32 {
            if 0.0 > d {
                f32::from_bits(d.to_bits() & !SIGN_MASK)
            } else {
                d
            }
        }
        #[inline(always)]
        unsafe fn strlen(mut p: u32) -> u32 {
            unsafe {
                let mut n = 0u32;
                while rd8(p) != 0 {
                    p += 1;
                    n += 1;
                }
                n
            }
        }

        // Emit one setting triple into a panel: build the 24-byte record
        // (callee 6), pass its words by value with the tag (slot +0x100),
        // release the record (callee 7). `s100` is (first, tag, last) in
        // push order; the call observes (last, tag, first, words...).
        #[inline(always)]
        unsafe fn emit(
            obj: u32, vt: u32, a0: u32, a1: u32, p1: u32, tag: u32, p3: u32,
            zeros: u32,
        ) {
            unsafe {
                let rec = lf_checker_rt::callee_thiscall!(6, u32, zeros, a0, a1);
                let f: extern "thiscall" fn(
                    u32, u32, u32, u32, u32, u32, u32, u32, u32, u32,
                ) -> u32 = core::mem::transmute(
                    rd32(vt.wrapping_add(VT_EMIT)) as usize
                );
                f(
                    obj,
                    p3,
                    lf_checker_rt::relocated(tag),
                    p1,
                    rd32(rec),
                    rd32(rec.wrapping_add(4)),
                    rd32(rec.wrapping_add(8)),
                    rd32(rec.wrapping_add(12)),
                    rd32(rec.wrapping_add(16)),
                    rd32(rec.wrapping_add(20)),
                );
                lf_checker_rt::callee_thiscall!(7, u32, zeros);
            }
        }
        #[inline(always)]
        unsafe fn slot0(vt: u32, slot: u32, obj: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(slot)) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn slot1(vt: u32, slot: u32, obj: u32, a0: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(slot)) as usize);
                f(obj, a0)
            }
        }
        #[inline(always)]
        unsafe fn slot2(vt: u32, slot: u32, obj: u32, a0: u32, a1: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(slot)) as usize);
                f(obj, a0, a1)
            }
        }
        #[inline(always)]
        unsafe fn slot4(
            vt: u32, slot: u32, obj: u32, a0: u32, a1: u32, a2: u32, a3: u32,
        ) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(slot)) as usize);
                f(obj, a0, a1, a2, a3)
            }
        }

        let vt0 = rd32(this);
        // Scratch mirrors of the original's frame slots: the stubs observe
        // these through the pointers we pass (the contents match the
        // original's stack Fill + writes on every trial).
        let zeros = [0u8; 24];
        let zeros_ptr = zeros.as_ptr() as u32;
        // Mirror of frame+0x30..: [0] selected float, [1] first ratio,
        // [2..] base directory copy.
        let mut surf = [0u8; 32];
        let surf_ptr = surf.as_mut_ptr() as u32;
        // Mirror of frame+0x04..: [0] second ratio, [1]/[2] title struct.
        let mut face = [0u8; 32];
        let face_ptr = face.as_mut_ptr() as u32;
        // Mirror of frame+0x28..: first title struct [ptr, len].
        let mut head = [0u8; 8];
        let head_ptr = head.as_mut_ptr() as u32;
        let mut f52: u32 = 0;

        let ready = slot0(vt0, VT_READY, this);
        if ready & 0xff != 0 {
            lf_checker_rt::callee_cdecl!(21, u32,);
            return ready;
        }
        let stem = this.wrapping_add(STEM);
        lf_checker_rt::callee_stdcall!(1, u32, stem, this.wrapping_add(CLIP_INDEX));

        // Background panel.
        let bg = lf_checker_rt::callee_cdecl!(2, u32, 0x25cu32);
        if bg == 0 {
            wr32(this.wrapping_add(PANEL_BG), 0);
        } else {
            let t = slot0(vt0, VT_BACKEND, this);
            let built = lf_checker_rt::callee_thiscall!(
                3, u32, bg, lf_checker_rt::relocated(GAL_BG), t
            );
            wr32(this.wrapping_add(PANEL_BG), built);
        }
        // Float ratios from the size globals.
        let q = || -> bool { lf_checker_rt::callee_cdecl!(4, u32,) & 0xff != 0 };
        let pick = |a: u32, b: u32| -> i32 {
            rd32(lf_checker_rt::relocated(if q() { b } else { a })) as i32
        };
        let ratio1 = fdiv(pick(SIZE_B, SIZE_B_ALT) as f32, pick(SIZE_A, SIZE_A_ALT) as f32);
        let ratio2 = fdiv(
            rdf(lf_checker_rt::relocated(ZERO_GAP)),
            fdiv(pick(SIZE_B, SIZE_B_ALT) as f32, pick(SIZE_A, SIZE_A_ALT) as f32),
        );
        let ratio3 = fdiv(
            fdiv(
                rdf(lf_checker_rt::relocated(REF_W)),
                rdf(lf_checker_rt::relocated(REF_H)),
            ),
            fdiv(pick(SIZE_A, SIZE_A_ALT) as f32, pick(SIZE_B, SIZE_B_ALT) as f32),
        );
        wr32(face_ptr, ratio2.to_bits());
        wr32(surf_ptr.wrapping_add(4), ratio1.to_bits());
        let mut pick_ratio = ratio1;
        if rd8(lf_checker_rt::relocated(WIDE_FLAG)) != 0 {
            pick_ratio = rdf(lf_checker_rt::relocated(RATIO_16_9));
        }
        let bg_obj = rd32(this.wrapping_add(PANEL_BG));
        wr32(surf_ptr, 0xff1a_1a1a);
        lf_checker_rt::callee_thiscall!(
            5, u32, bg_obj, 0u32, 0u32, surf_ptr.wrapping_add(0), 0xffff_ffffu32
        );
        let bg_vt = rd32(bg_obj);
        emit(bg_obj, bg_vt, 0, 0x4120_0000, 0x10, 0x00f1_7b24, 4, zeros_ptr);
        emit(bg_obj, bg_vt, 0, 0xc120_0000, 4, 0x00f1_7b34, 0x10, zeros_ptr);
        emit(bg_obj, bg_vt, 0, 0, 8, 0x00f1_7b48, 2, zeros_ptr);
        let scaled = fmul(
            fmul(pick_ratio, rdf(lf_checker_rt::relocated(K_0_5625))),
            rdf(lf_checker_rt::relocated(K_734_4)),
        );
        slot1(bg_vt, VT_SCALE, bg_obj, scaled.to_bits());
        wr32(bg_obj.wrapping_add(OBJ_CURRENT), 1);
        slot1(bg_vt, VT_CURRENT, bg_obj, 1);
        let handle = slot0(vt0, VT_HANDLE, this);
        slot1(bg_vt, VT_ATTACH, bg_obj, handle);

        // Previewer panel.
        let pv = lf_checker_rt::callee_cdecl!(2, u32, 0x24cu32);
        if pv == 0 {
            wr32(this.wrapping_add(PANEL_PREVIEW), 0);
        } else {
            let t = slot0(vt0, VT_BACKEND, this);
            let built = lf_checker_rt::callee_thiscall!(
                8, u32, pv, lf_checker_rt::relocated(GAL_PREVIEWER), t, this
            );
            wr32(this.wrapping_add(PANEL_PREVIEW), built);
        }
        let pv_obj = rd32(this.wrapping_add(PANEL_PREVIEW));
        lf_checker_rt::callee_thiscall!(
            9, u32, pv_obj, stem, rd32(this.wrapping_add(CLIP_INDEX))
        );
        let pv_vt = rd32(pv_obj);
        emit(pv_obj, pv_vt, 0, 0x41a0_0000, 4, 0x00f1_7b70, 4, zeros_ptr);
        emit(pv_obj, pv_vt, 0, 0, 0x10, 0x00f1_7b8c, 0x10, zeros_ptr);
        emit(pv_obj, pv_vt, 0x40a0_0000, 0, 2, 0x00f1_7ba8, 2, zeros_ptr);
        emit(pv_obj, pv_vt, 0xc0a0_0000, 0, 8, 0x00f1_7bc4, 8, zeros_ptr);
        let handle = slot0(vt0, VT_HANDLE, this);
        slot1(pv_vt, VT_ATTACH, pv_obj, handle);
        slot1(pv_vt, VT_CURRENT, pv_obj, 1);

        // Viewer panel.
        let vw = lf_checker_rt::callee_cdecl!(2, u32, 0x37cu32);
        if vw == 0 {
            wr32(this.wrapping_add(PANEL_VIEWER), 0);
        } else {
            let t = slot0(vt0, VT_BACKEND, this);
            let built = lf_checker_rt::callee_thiscall!(
                22, u32, vw, lf_checker_rt::relocated(GAL_VIEWER), t, this,
                lf_checker_rt::relocated(CTX_A), 0u32
            );
            wr32(this.wrapping_add(PANEL_VIEWER), built);
        }
        let vw_obj = rd32(this.wrapping_add(PANEL_VIEWER));
        wr32(vw_obj.wrapping_add(OBJ_MODE), 2);
        lf_checker_rt::callee_cdecl!(
            10, u32, lf_checker_rt::relocated(VIDEOS_RENDERED), 0u32
        );
        // Copy the base directory into the surface buffer.
        {
            let base = lf_checker_rt::relocated(BASE_DIR);
            let mut i = 0usize;
            loop {
                let b = rd8(base.wrapping_add(i as u32));
                surf[8 + i] = b;
                if b == 0 {
                    break;
                }
                i += 1;
            }
        }
        // Aspect ladder over the first ratio: each band matches when the
        // ratio is within 0.01 of its centre (strictly above on the
        // tolerance, so NaN matches nothing).
        let x = pick_ratio;
        let tol = rdf(lf_checker_rt::relocated(0x00fe_870c));
        let mut ladder0 = rdf(lf_checker_rt::relocated(0x00fe_8a24));
        let mut ladder2 = rdf(lf_checker_rt::relocated(0x00f1_86e0));
        if tol > abs_if_below(fsub(x, rdf(lf_checker_rt::relocated(0x00fe_89b8)))) {
            ladder0 = rdf(lf_checker_rt::relocated(0x00fe_8a60));
            ladder2 = rdf(lf_checker_rt::relocated(0x00f1_86e8));
        } else if tol
            > abs_if_below(fsub(x, rdf(lf_checker_rt::relocated(0x00fe_8934))))
        {
            let pick_hi = lf_checker_rt::callee_cdecl!(11, u32,) & 0xff != 0;
            ladder0 = rdf(lf_checker_rt::relocated(0x00fe_8a94));
            ladder2 = rdf(lf_checker_rt::relocated(if pick_hi {
                0x00eb_229c
            } else {
                0x00f1_86dc
            }));
        } else if tol
            > abs_if_below(fsub(x, rdf(lf_checker_rt::relocated(0x00fe_8994))))
        {
            ladder2 = rdf(lf_checker_rt::relocated(0x00f1_86e4));
        } else if tol
            > abs_if_below(fsub(x, rdf(lf_checker_rt::relocated(0x00fe_897c))))
        {
            ladder0 = rdf(lf_checker_rt::relocated(0x00fe_8a94));
        } else if tol
            > abs_if_below(fsub(x, rdf(lf_checker_rt::relocated(0x00fe_8920))))
        {
            ladder0 = rdf(lf_checker_rt::relocated(0x00fe_8a94));
            ladder2 = rdf(lf_checker_rt::relocated(0x00fe_8bd0));
        }
        wr32(surf_ptr, ladder0.to_bits());
        let f0 = fdiv(
            fmul(
                fmul(ladder2, rdf(lf_checker_rt::relocated(0x00fe_8b28))),
                rdf(lf_checker_rt::relocated(0x00fe_87a0)),
            ),
            rdf(face_ptr),
        );
        let f1 = fmul(ladder2, ratio3);
        lf_checker_rt::callee_thiscall!(
            12, u32, vw_obj, surf_ptr.wrapping_add(8),
            lf_checker_rt::relocated(TAG_SUFFIX), 1u32, f0.to_bits(),
            f1.to_bits(), 0x40a0_0000u32, 0x4170_0000u32
        );
        let vw_vt = rd32(vw_obj);
        emit(vw_obj, vw_vt, 0, 0x4120_0000, 0x10, 0x00f1_7c14, 4, zeros_ptr);
        emit(vw_obj, vw_vt, 0, 0xc120_0000, 4, 0x00f1_7c24, 0x10, zeros_ptr);
        emit(vw_obj, vw_vt, ladder0.to_bits(), 0, 8, 0x00f1_7c38, 2, zeros_ptr);
        emit(vw_obj, vw_vt, 0xc2cc_0000, 0, 8, 0x00f1_7c54, 8, zeros_ptr);
        slot1(vw_vt, VT_NOTIFY, vw_obj, 1);
        slot1(vw_vt, VT_CURRENT, vw_obj, 1);
        slot1(vw_vt, VT_EXTRA, vw_obj, 1);
        let handle = slot0(vt0, VT_HANDLE, this);
        slot1(vw_vt, VT_ATTACH, vw_obj, handle);

        // Title string from the stem, truncated against the word pairs.
        wr32(head_ptr, 0);
        wr32(head_ptr.wrapping_add(4), 0);
        f52 = 0;
        if stem != 0 {
            let len = strlen(stem);
            lf_checker_rt::callee_thiscall!(13, u32, head_ptr, stem, len);
            let data = rd32(head_ptr);
            let mut si = rd16(head_ptr.wrapping_add(4));
            f52 = data;
            let mut cut: Option<u16> = None;
            for round in 0..3 {
                let (wo, dout) = WORD_PAIRS[round];
                let w = rd16(this.wrapping_add(wo));
                let arg = if w == 0 {
                    DEFAULT_ID
                } else {
                    rd32(this.wrapping_add(dout))
                };
                let ok = lf_checker_rt::callee_thiscall!(14, u32, head_ptr, arg);
                if ok & 0xff == 0 {
                    if round == 2 {
                        break;
                    }
                    continue;
                }
                if round == 0 {
                    if si <= w {
                        continue;
                    }
                    cut = Some(si.wrapping_sub(w));
                    break;
                }
                let c = si.wrapping_sub(w);
                if c == 0 {
                    if round == 2 {
                        break;
                    }
                    continue;
                }
                cut = Some(c);
                break;
            }
            if let Some(c) = cut {
                if c != 0 {
                    wr16(head_ptr.wrapping_add(4), c);
                    si = c;
                    (f52.wrapping_add(c as u32) as *mut u8).write(0);
                }
            }
            let _ = si;
        }
        // Title panel.
        let tt = lf_checker_rt::callee_cdecl!(2, u32, 0x610u32);
        if tt == 0 {
            wr32(this.wrapping_add(PANEL_TITLE), 0);
        } else {
            let t = slot0(vt0, VT_BACKEND, this);
            let built = lf_checker_rt::callee_thiscall!(
                15, u32, tt, lf_checker_rt::relocated(GAL_TITLE), t
            );
            wr32(this.wrapping_add(PANEL_TITLE), built);
        }
        let tt_obj = rd32(this.wrapping_add(PANEL_TITLE));
        let tt_ret = lf_checker_rt::callee_cdecl!(
            16, u32, face_ptr, 0x3eu32, 0u32, 2u32
        );
        let tt_vt = rd32(tt_obj);
        slot4(tt_vt, VT_BIND4, tt_obj, 0x4190_0000, tt_ret, 0, 2);
        slot1(tt_vt, VT_FLAG, tt_obj, 1);
        let use_ptr = rd16(head_ptr.wrapping_add(4)) != 0;
        let arg = if use_ptr { rd32(head_ptr) } else { DEFAULT_ID };
        slot2(tt_vt, VT_TITLE, tt_obj, 0, arg);
        slot1(tt_vt, VT_RES, tt_obj, 1);
        slot1(tt_vt, VT_FIN, tt_obj, 1);
        slot1(tt_vt, VT_SEAL, tt_obj, 1);
        emit(tt_obj, tt_vt, 0, 0x4000_0000, 4, 0x00f1_7cd4, 4, zeros_ptr);
        emit(tt_obj, tt_vt, 0, 0xc000_0000, 4, 0x00f1_7d1c, 0x10, zeros_ptr);
        emit(tt_obj, tt_vt, 0x40a0_0000, 0, 2, 0x00f1_7d4c, 2, zeros_ptr);
        let handle = slot0(vt0, VT_HANDLE, this);
        slot1(tt_vt, VT_ATTACH, tt_obj, handle);
        slot1(tt_vt, VT_CURRENT, tt_obj, 1);

        // Export tag from the stem.
        lf_checker_rt::callee_thiscall!(
            13, u32, face_ptr.wrapping_add(4),
            lf_checker_rt::relocated(CTX_B), 0u32
        );
        {
            let r = lf_checker_rt::relocated(REF_EMPTY);
            let mut i = 0u32;
            let mut diff = 0i32;
            loop {
                let a = rd8(stem.wrapping_add(i));
                let b = rd8(r.wrapping_add(i));
                if a != b {
                    diff = if a < b { -1 } else { 1 };
                    break;
                }
                if a == 0 {
                    break;
                }
                i += 1;
            }
            if diff != 0 {
                let which =
                    lf_checker_rt::callee_thiscall!(17, u32, this, stem);
                let tag = if which == 0 {
                    Some(TAG_LOW)
                } else if which == 1 {
                    Some(TAG_MED)
                } else if which == 2 {
                    Some(TAG_HI)
                } else {
                    None
                };
                if let Some(t) = tag {
                    lf_checker_rt::callee_thiscall!(
                        18, u32, face_ptr.wrapping_add(4),
                        lf_checker_rt::relocated(t)
                    );
                }
            }
        }
        // Resolution panel.
        let rs = lf_checker_rt::callee_cdecl!(2, u32, 0x610u32);
        if rs == 0 {
            wr32(this.wrapping_add(PANEL_RES), 0);
        } else {
            let t = slot0(vt0, VT_BACKEND, this);
            let built = lf_checker_rt::callee_thiscall!(
                15, u32, rs, lf_checker_rt::relocated(GAL_RES), t
            );
            wr32(this.wrapping_add(PANEL_RES), built);
        }
        let rs_obj = rd32(this.wrapping_add(PANEL_RES));
        let rs_ret = lf_checker_rt::callee_cdecl!(
            16, u32, face_ptr, 0x3eu32, 0u32, 2u32
        );
        let rs_vt = rd32(rs_obj);
        slot4(rs_vt, VT_BIND4, rs_obj, 0x4180_0000, rs_ret, 0, 2);
        slot1(rs_vt, VT_FLAG, rs_obj, 1);
        let use_ptr = rd16(face_ptr.wrapping_add(8)) != 0;
        let arg = if use_ptr { rd32(face_ptr.wrapping_add(4)) } else { DEFAULT_ID };
        slot2(rs_vt, VT_TITLE, rs_obj, 0, arg);
        slot1(rs_vt, VT_RES, rs_obj, 1);
        slot1(rs_vt, VT_FIN, rs_obj, 1);
        slot1(rs_vt, VT_SEAL, rs_obj, 1);
        emit(rs_obj, rs_vt, 0, 0x4000_0000, 4, 0x00f1_7f7c, 4, zeros_ptr);
        emit(rs_obj, rs_vt, 0, 0xc000_0000, 4, 0x00f1_7fd8, 0x10, zeros_ptr);
        emit(rs_obj, rs_vt, 0xc0a0_0000, 0, 8, 0x00f1_8018, 8, zeros_ptr);
        let handle = slot0(vt0, VT_HANDLE, this);
        slot1(rs_vt, VT_ATTACH, rs_obj, handle);
        slot1(rs_vt, VT_CURRENT, rs_obj, 1);

        // Teardown: release both title strings, rebind the stem slot,
        // notify this twice, return the last answer.
        lf_checker_rt::callee_cdecl!(19, u32, rd32(face_ptr.wrapping_add(4)));
        lf_checker_rt::callee_cdecl!(19, u32, rd32(head_ptr));
        lf_checker_rt::callee_thiscall!(
            20, u32, this.wrapping_add(STEM_SLOT),
            lf_checker_rt::relocated(STEM_KEY)
        );
        slot1(vt0, VT_NOTIFY, this, 1);
        let done = slot1(vt0, VT_DONE, this, 1);
        lf_checker_rt::callee_cdecl!(21, u32,);
        done
    }
});
