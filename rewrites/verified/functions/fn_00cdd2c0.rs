// original: 0x00CDD2C0 CTaskSimpleNMBrace::vf27 (merged symbol, vtable slot 27)

/// Brace-task update: aim the ped's brace constraint at a reach point and
/// send the resulting NaturalMotion message.
///
/// `this` is the task, `ped` the ped. The update runs in stages:
///
/// 1. Retrigger stamp: when the ped's sub-object at `+0x16c` has mode bits
///    `0x80` (bits 6-9 of its word at `+0x28`) and the ped's timer at `+0x170`
///    is above a limit (5.0, or 50.0 when the flag at `+0x219` is set), the
///    stamp global is copied to the task word at `+0x24`.
/// 2. Gate: a shared per-task call runs on the sub-object at `+0x28`, then a
///    virtual call through the ped's context (`+0x7b4`, slot `0x50`) must not
///    answer -1, the task link at `+0x54` must exist, and its info block at
///    `+0x38` must exist. Any failure ends the update.
/// 3. Reach point: the reach callee fills a point (plus 0.4 on z), and the
///    link's anchor (matrix at `+0x20` plus `0x30`, else the link plus `0x10`)
///    minus the ped-matrix row gives a direction whose length feeds an
///    eligibility call with an out-pointer pair.
/// 4. Target: a nonzero eligibility answer copies 16 bytes of scratch (zero
///    under the checker's defined stack fill) as the target; a zero answer
///    takes the link anchor with 0.3 off z. Link mode bits 2-4 (of the word
///    at `+0x28` shifted right 6) additionally route the target through the
///    aim callee.
/// 5. Send: the link anchor is reread (a mode-`0x80` link also probes the
///    anchor callee), two offsets are transformed by the link matrix (each
///    null matrix slot first runs a resolve call pair that fills it), and a
///    message with two vec3 slots plus the info block's word is built on the
///    stack, sent through the ped's context, and followed by a tail call
///    carrying the transformed offsets.
///
/// All float operation order is the original's, including the row order of
/// the two matrix transforms (second-lane product first). Returns the last
/// callee's answer. Original: 0x00CDD2C0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00cdd2c0(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_STAMP: u32 = 0x24;
        const TASK_SUB: u32 = 0x28;
        const TASK_LINK: u32 = 0x54;
        const PED_MATRIX: u32 = 0x20;
        const PED_SUB: u32 = 0x16c;
        const PED_TIMER: u32 = 0x170;
        const PED_FLAG: u32 = 0x219;
        const PED_NMCTX: u32 = 0x7b4;
        const LINK_MATRIX: u32 = 0x20;
        const LINK_MODE: u32 = 0x28;
        const LINK_INFO: u32 = 0x38;
        const LINK_AUX: u32 = 0xf50;
        const MODE_MASK: u32 = 0x3c0;
        const MODE_BRACE: u32 = 0x80;
        const STAMP_SRC: u32 = 0x11735b4;
        const LIM_LO: u32 = 0xfe8ad8;
        const LIM_HI: u32 = 0xfe8b68;
        const REACH_LIFT: u32 = 0xfe881c;
        const TARGET_DROP: u32 = 0xfe87e8;
        const ANCHOR_CODE: u32 = 0x4b5;
        const VT_SLOT: u32 = 0x50;
        const SHARED_CALL: u32 = 1;
        const REACH_PT: u32 = 3;
        const DIR_CALLEE: u32 = 4;
        const ELIGIBLE: u32 = 5;
        const AIM_CALLEE: u32 = 6;
        const ANCHOR_CALLEE: u32 = 7;
        const RESOLVE_A: u32 = 8;
        const RESOLVE_B: u32 = 9;
        const NM_CTOR: u32 = 10;
        const NM_SET_VEC3: u32 = 11;
        const NM_SET_INT: u32 = 12;
        const NM_SEND: u32 = 13;
        const TAIL_CALL: u32 = 14;
        const NM_DTOR: u32 = 15;
        const COOKIE: u32 = 16;

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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn anchor_of(obj: u32) -> u32 {
            unsafe {
                let m = rd32(obj + LINK_MATRIX);
                if m != 0 { m.wrapping_add(0x30) } else { obj.wrapping_add(0x10) }
            }
        }
        /// One matrix row: (m1*dy + m0*dx) + m2*dz, the original's order.
        #[inline(always)]
        fn row(m0: f32, m1: f32, m2: f32, dx: f32, dy: f32, dz: f32) -> f32 {
            add(add(mul(m1, dy), mul(m0, dx)), mul(m2, dz))
        }

        let mut eax = 0u32;

        // Stage 1: retrigger stamp.
        let lim = if rd8(ped + PED_FLAG) != 0 { g32(LIM_HI) } else { g32(LIM_LO) };
        let psub = rd32(ped + PED_SUB);
        if psub != 0
            && (rd32(psub + LINK_MODE) & MODE_MASK) == MODE_BRACE
            && rdf(ped + PED_TIMER) > f32::from_bits(lim)
        {
            wr32(this + TASK_STAMP, g32(STAMP_SRC));
        }

        // Stage 2: gate.
        eax = lf_checker_rt::callee_thiscall!(SHARED_CALL, u32, this.wrapping_add(TASK_SUB), ped);
        let nmctx = rd32(ped + PED_NMCTX);
        let vslot: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(rd32(nmctx) + VT_SLOT) as usize) };
        eax = vslot(nmctx);
        if eax == 0xFFFF_FFFF {
            lf_checker_rt::callee_stdcall!(COOKIE, u32,);
            return eax;
        }
        let link = rd32(this + TASK_LINK);
        eax = link;
        if link == 0 || rd32(link + LINK_INFO) == 0 {
            lf_checker_rt::callee_stdcall!(COOKIE, u32,);
            return eax;
        }

        // Stage 3: reach point and direction.
        let mut reach = [0u32; 16];
        let rbuf = reach.as_mut_ptr() as u32;
        eax = lf_checker_rt::callee_thiscall!(REACH_PT, u32, ped, rbuf, 0);
        let pmat = rd32(ped + PED_MATRIX);
        let rx = f32::from_bits(reach[12]);
        let ry = f32::from_bits(reach[13]);
        let rz = add(f32::from_bits(reach[14]), f32::from_bits(g32(REACH_LIFT)));
        let ap = anchor_of(link);
        let dx = sub(rdf(ap), rdf(pmat.wrapping_add(0x30)));
        let dy = sub(rdf(ap.wrapping_add(4)), rdf(pmat.wrapping_add(0x34)));
        let dz = sub(rdf(ap.wrapping_add(8)), rdf(pmat.wrapping_add(0x38)));
        let mut pad = [0u32; 4];
        let pbuf = pad.as_mut_ptr() as u32;
        eax = lf_checker_rt::callee_thiscall!(DIR_CALLEE, u32, pbuf);
        let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        let dist = core::hint::black_box(d2).sqrt();
        let reach_pt = [rx.to_bits(), ry.to_bits(), rz.to_bits(), 0u32];
        let r = lf_checker_rt::callee_cdecl!(
            ELIGIBLE, u32, link, reach_pt.as_ptr() as u32, dist.to_bits(), pbuf, 1);
        eax = r;

        // Stage 4: target selection, then the aim call for modes 2-4.
        let (mut tx, mut ty, mut tz);
        let mode = (rd32(link + LINK_MODE) >> 6) & 0xf;
        let mut probe = [0u32; 4];
        let pptr = probe.as_mut_ptr() as u32;
        if (r as u8) != 0 {
            // The original copies 16 bytes of uninitialised scratch here;
            // under the checker's defined stack fill those bytes are zero.
            tx = 0.0;
            ty = 0.0;
            tz = 0.0;
            if mode > 1 && mode < 5 {
                eax = lf_checker_rt::callee_thiscall!(
                    AIM_CALLEE, u32, link, pptr, probe.as_mut_ptr() as u32, 1, 0);
            }
        } else {
            let q = anchor_of(link);
            tx = rdf(q);
            ty = rdf(q.wrapping_add(4));
            tz = sub(rdf(q.wrapping_add(8)), f32::from_bits(g32(TARGET_DROP)));
            if mode > 1 && mode < 5 {
                eax = lf_checker_rt::callee_thiscall!(
                    AIM_CALLEE, u32, link, pptr, pmat.wrapping_add(0x30), 1, 0);
            }
        }

        // Stage 5: reread the anchor, transform twice, build and send.
        let p2 = anchor_of(link);
        let mut ex = rdf(p2);
        let mut ey = rdf(p2.wrapping_add(4));
        let mut ez = rdf(p2.wrapping_add(8));
        if (rd32(link + LINK_MODE) & MODE_MASK) == MODE_BRACE {
            let aux = rd32(link + LINK_AUX);
            if aux != 0 {
                let mut aprobe = [ex.to_bits(), ey.to_bits(), ez.to_bits()];
                eax = lf_checker_rt::callee_thiscall!(
                    ANCHOR_CALLEE, u32, aux, aprobe.as_mut_ptr() as u32, ANCHOR_CODE);
            }
        }
        let saved_x = ex;
        // First null check plus transform of (tx, ty, tz).
        let mut m = rd32(link + LINK_MATRIX);
        if m == 0 {
            eax = lf_checker_rt::callee_thiscall!(RESOLVE_A, u32, link);
            eax = lf_checker_rt::callee_thiscall!(
                RESOLVE_B, u32, link.wrapping_add(0x10), rd32(link.wrapping_add(0x20)));
            m = rd32(link + LINK_MATRIX);
        }
        let ox = sub(tx, rdf(m.wrapping_add(0x30)));
        let oy = sub(ty, rdf(m.wrapping_add(0x34)));
        let oz = sub(tz, rdf(m.wrapping_add(0x38)));
        tx = row(rdf(m), rdf(m.wrapping_add(4)), rdf(m.wrapping_add(8)), ox, oy, oz);
        ty = row(rdf(m.wrapping_add(0x10)), rdf(m.wrapping_add(0x14)), rdf(m.wrapping_add(0x18)), ox, oy, oz);
        tz = row(rdf(m.wrapping_add(0x20)), rdf(m.wrapping_add(0x24)), rdf(m.wrapping_add(0x28)), ox, oy, oz);
        // Second null check plus transform of (ex, ey, ez).
        m = rd32(link + LINK_MATRIX);
        if m == 0 {
            eax = lf_checker_rt::callee_thiscall!(RESOLVE_A, u32, link);
            eax = lf_checker_rt::callee_thiscall!(
                RESOLVE_B, u32, link.wrapping_add(0x10), rd32(link.wrapping_add(0x20)));
            m = rd32(link + LINK_MATRIX);
        }
        let qx = sub(ex, rdf(m.wrapping_add(0x30)));
        let qy = sub(ey, rdf(m.wrapping_add(0x34)));
        let qz = sub(ez, rdf(m.wrapping_add(0x38)));
        ex = row(rdf(m), rdf(m.wrapping_add(4)), rdf(m.wrapping_add(8)), qx, qy, qz);
        ey = row(rdf(m.wrapping_add(0x10)), rdf(m.wrapping_add(0x14)), rdf(m.wrapping_add(0x18)), qx, qy, qz);
        ez = row(rdf(m.wrapping_add(0x20)), rdf(m.wrapping_add(0x24)), rdf(m.wrapping_add(0x28)), qx, qy, qz);

        let mut msg = [0u32; 16];
        let buf = msg.as_mut_ptr() as u32;
        eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf);
        eax = lf_checker_rt::callee_thiscall!(
            NM_SET_VEC3, u32, buf, g32(0x1051ed0), tx.to_bits(), ty.to_bits(), tz.to_bits());
        eax = lf_checker_rt::callee_thiscall!(
            NM_SET_VEC3, u32, buf, g32(0x1051ed4), ex.to_bits(), ey.to_bits(), ez.to_bits());
        let info = rd32(link + LINK_INFO);
        let word = rd16(info.wrapping_add(8));
        eax = lf_checker_rt::callee_thiscall!(NM_SET_INT, u32, buf, g32(0x1051ed8), word);
        eax = lf_checker_rt::callee_thiscall!(NM_SEND, u32, nmctx, g32(0x1051ec8), buf);
        let out_x = ex.to_bits();
        let pre_x = saved_x.to_bits();
        eax = lf_checker_rt::callee_thiscall!(
            TAIL_CALL, u32, this.wrapping_add(TASK_SUB), ped,
            &out_x as *const u32 as u32, &pre_x as *const u32 as u32, word);
        eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf);
        lf_checker_rt::callee_stdcall!(COOKIE, u32,);
        eax
    }
});

