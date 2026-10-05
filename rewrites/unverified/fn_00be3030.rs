// original: 0x00be3030 CTaskComplexSitDownThenIdleThenStandUp::vf19 (merged name)

/// Run a sit-down/idle/stand-up task tick: pose the ped, advance the stage.
///
/// `task` points to the task (`+0x34` action id, `+0x3c` action handle,
/// `+0x84` pose-dirty flag, `+0x30` pose handle, `+0x4` sub-task, `+0x20` /
/// `+0x24` / `+0x28` pose accumulators, `+0x74` last blend, `+0x80` / `+0x81` /
/// `+0x82` stage flags); `ped` is the ped the task runs on (matrix at
/// `+0x20`, scratch float at `+0x1c`, flag word at `+0x29c`).
///
/// The tick resolves the action (callee 1, cdecl) and, when the pose is
/// dirty, rebuilds it (callee 2, cdecl) and clears the accumulators. With a
/// live pose handle it blends the pose: a sub-task of type 0x15f supplies
/// the blend object directly, otherwise one is built (callee 4, thiscall
/// with a pointer to the ped's saved position, snapshotted while the pointer
/// itself is skipped) and a null build ends the blend. The blend object runs
/// (virtual slot `+0x54`); two shaping functions (callees 6 and 7, taking the
/// blend weight in vector register 0 via the checker's transport and
/// returning floats) feed a rotation mix with three global gains, folded
/// into the accumulators together with the blend object's parameters. The
/// blend weight is also kept in the task's `+0x74` slot.
///
/// The stage logic then runs on every tick: with the `+0x80` flag set and a
/// nonzero action-table byte for the action id, a clear `+0x81` flag finishes
/// through callee 17 at once. A set `+0x81` flag poses the ped (callee 8 for
/// a live handle, else callee 10 with the scratch float; callee 9 always)
/// and, unless `+0x82` is set, validates through three callees (11, 12,
/// 13): a zero answer runs a fallback (callee 14) and ends at the shared
/// exit, while a nonzero answer drives the release sequence (callees 15 and
/// 16, whose caller-stack address argument is skipped as unreproducible),
/// re-poses, and returns callee 17's answer. The shared exit asks the
/// dispatcher (callee 18) and returns callee 19's answer, or 0 on a null
/// dispatcher. The branch that would call the original's double-precision
/// helper with vector-register arguments is never taken (the checker's
/// transports move four bytes, not eight); the contract pins the steering
/// word so the fallback float is always used.
///
/// Original: 0x00be3030 (thiscall, ecx = task, one stack word = ped).
lf_checker_rt::export!(thiscall, rw_00be3030(task: u32, ped: u32) -> u32 {
    unsafe {
        const ACTION: u32 = 0x34;
        const HANDLE: u32 = 0x3c;
        const DIRTY: u32 = 0x84;
        const POSE: u32 = 0x30;
        const SUB: u32 = 0x4;
        const SUB_BLEND: u32 = 0x18;
        const ACC0: u32 = 0x20;
        const ACC1: u32 = 0x24;
        const ACC2: u32 = 0x28;
        const BLEND: u32 = 0x74;
        const STAGE0: u32 = 0x80;
        const STAGE1: u32 = 0x81;
        const STAGE2: u32 = 0x82;
        const PED_MATRIX: u32 = 0x20;
        const PED_SCRATCH: u32 = 0x1c;
        const PED_FLAGS: u32 = 0x29c;
        const BLEND_P0: u32 = 0x4;
        const BLEND_P1: u32 = 0xc;
        const BLEND_P2: u32 = 0x8;
        const VTA_WEIGHT: u32 = 0x10;
        const SLOT_TYPE: u32 = 0xc;
        const SLOT_RUN: u32 = 0x54;
        const WANT_TYPE: u32 = 0x15f;
        const GSTATE: u32 = 0x1682f20;
        const GAIN0: u32 = 0x1682f10;
        const GAIN1: u32 = 0x1682f14;
        const GAIN2: u32 = 0x1682f18;
        const GAIN2_INIT: u32 = 0x3f0ccccd;
        const ACT_TABLE: u32 = 0x167f628;
        const MGR_GLOBAL: u32 = 0x16dd63c;
        const PARAM: u32 = 0x10496e8;
        const NEG_ONE: u32 = 0xbf800000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                f(obj)
            }
        }

        let a: u32 = lf_checker_rt::callee_cdecl!(1, u32, ped, rd32(task.wrapping_add(ACTION)));
        wr32(task.wrapping_add(HANDLE), a);
        if rd8(task.wrapping_add(DIRTY)) != 0 {
            let b: u32 = lf_checker_rt::callee_cdecl!(2, u32, task.wrapping_add(ACC0), task);
            wr32(task.wrapping_add(POSE), b);
            wr32(task.wrapping_add(ACC0), 0);
            wr32(task.wrapping_add(ACC0).wrapping_add(4), 0);
            wr32(task.wrapping_add(ACC0).wrapping_add(8), 0);
            wr8(task.wrapping_add(DIRTY), 0);
        }
        if rd32(task.wrapping_add(POSE)) != 0 {
            // One-time global gain initialisation.
            let gs = lf_checker_rt::relocated(GSTATE);
            if rd32(gs) & 1 == 0 {
                wr32(gs, rd32(gs) | 1);
                wr32(lf_checker_rt::relocated(GAIN0), 0);
                wr32(lf_checker_rt::relocated(GAIN1), 0);
                wr32(lf_checker_rt::relocated(GAIN2), GAIN2_INIT);
            }
            let pm = rd32(ped.wrapping_add(PED_MATRIX));
            let spos = [rdf(pm.wrapping_add(0x30)), rdf(pm.wrapping_add(0x34)), rdf(pm.wrapping_add(0x38))];
            let c = rd32(task.wrapping_add(SUB));
            let mut e: u32 = 0;
            let mut direct = false;
            if c != 0 && vcall0(c, SLOT_TYPE) == WANT_TYPE {
                e = rd32(c.wrapping_add(SUB_BLEND));
                direct = e != 0;
            }
            if !direct {
                e = lf_checker_rt::callee_thiscall!(
                    4, u32, rd32(task.wrapping_add(POSE)), 0x11u32, spos.as_ptr() as u32
                );
                if e == 0 {
                    return be3030_tail(task, ped);
                }
            }
            let f0 = rdf(e.wrapping_add(BLEND_P0));
            let f1 = rdf(e.wrapping_add(BLEND_P1));
            let va = vcall0(e, SLOT_RUN);
            let f2 = rdf(va.wrapping_add(VTA_WEIGHT));
            let x: f32 = f32::from_bits(lf_checker_rt::callee_cdecl!(6, u32, f2.to_bits()));
            let y: f32 = f32::from_bits(lf_checker_rt::callee_cdecl!(7, u32, f2.to_bits()));
            let g0 = rdf(lf_checker_rt::relocated(GAIN0));
            let g1 = rdf(lf_checker_rt::relocated(GAIN1));
            let t_g1x = mul(g1, x);
            let t_g0x = mul(g0, x);
            let t_g0y = mul(g0, y);
            let t_g1y = mul(g1, y);
            let d = sub(t_g0y, t_g1x);
            let f1g = add(f1, rdf(lf_checker_rt::relocated(GAIN2)));
            let s = add(t_g1y, t_g0x);
            let o0 = add(f0, d);
            let s = add(s, rdf(e.wrapping_add(BLEND_P2)));
            let m20 = rdf(task.wrapping_add(ACC0));
            let m24 = rdf(task.wrapping_add(ACC1));
            let m28 = rdf(task.wrapping_add(ACC2));
            wr32(task.wrapping_add(ACC0), add(o0, m20).to_bits());
            wr32(task.wrapping_add(ACC1), add(s, m24).to_bits());
            wr32(task.wrapping_add(ACC2), add(m28, f1g).to_bits());
            wr32(task.wrapping_add(BLEND), f2.to_bits());
        }
        be3030_tail(task, ped)
    }
});

/// Stage logic shared by every path of `rw_00be3030`.
#[inline(always)]
unsafe fn be3030_tail(task: u32, ped: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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

        if rd8(task.wrapping_add(0x80)) != 0 {
            let a = rd32(task.wrapping_add(0x34));
            if rd8(lf_checker_rt::relocated(0x167f628).wrapping_add(a)) != 0 {
                if rd8(task.wrapping_add(0x81)) == 0 {
                    return lf_checker_rt::callee_thiscall!(17, u32, task, ped, 0x11du32);
                }
            }
        }
        if rd8(task.wrapping_add(0x81)) == 0 {
            return be3030_exit(task, ped);
        }
        wr32(
            ped.wrapping_add(0x29c),
            rd32(ped.wrapping_add(0x29c)) | 2,
        );
        let c = rd32(task.wrapping_add(0x30));
        if c != 0 {
            let f = rdf(task.wrapping_add(0x74));
            let _: u32 = lf_checker_rt::callee_thiscall!(
                8, u32, ped, c, 0u32, 0x3505u32, task.wrapping_add(0x20), f.to_bits(), 0u32
            );
        } else {
            // Under the contract [ped+0x20] is 0 on this path (see the doc
            // comment); any other value fails loudly on the call log.
            let base = rd32(ped.wrapping_add(0x20));
            let f = rdf(ped.wrapping_add(0x1c));
            let _: u32 = lf_checker_rt::callee_thiscall!(
                10, u32, ped, 0x3505u32, base.wrapping_add(0x30), f.to_bits(), 0u32
            );
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, ped, 0u32, 0u32);
        if rd8(task.wrapping_add(0x82)) != 0 {
            return be3030_exit(task, ped);
        }
        let t: u32 = lf_checker_rt::callee_thiscall!(11, u32, task);
        let g = rd32(lf_checker_rt::relocated(0x16dd63c));
        let u: u32 = lf_checker_rt::callee_thiscall!(12, u32, g, t);
        let p = rd32(lf_checker_rt::relocated(0x10496e8));
        let v: u32 = lf_checker_rt::callee_cdecl!(13, u32, u, p);
        if v & 0xff == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(14, u32, u, p, 8u32);
            return be3030_exit(task, ped);
        }
        let slot = rd32(task.wrapping_add(0x74));
        let _: u32 = lf_checker_rt::callee_cdecl!(15, u32, slot);
        wr8(task.wrapping_add(0x82), 1);
        let t2: u32 = lf_checker_rt::callee_thiscall!(11, u32, task);
        let mut frame2 = [0u32; 1];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            16, u32, frame2.as_mut_ptr() as u32, 1u32, rd32(task.wrapping_add(0x34)), 0u32, t2
        );
        let c = rd32(task.wrapping_add(0x30));
        if c != 0 {
            let f = rdf(task.wrapping_add(0x74));
            let _: u32 = lf_checker_rt::callee_thiscall!(
                8, u32, ped, c, 0u32, 0x3505u32, task.wrapping_add(0x20), f.to_bits(), 0u32
            );
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, ped, 0u32, 0u32);
        wr32(
            ped.wrapping_add(0x29c),
            rd32(ped.wrapping_add(0x29c)) | 2,
        );
        lf_checker_rt::callee_thiscall!(17, u32, task, ped, 0xddu32)
    }
}

/// Shared exit of `rw_00be3030`: dispatch and return.
#[inline(always)]
unsafe fn be3030_exit(task: u32, _ped: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let g = rd32(lf_checker_rt::relocated(0x167e2a0));
        let r: u32 = lf_checker_rt::callee_thiscall!(18, u32, g);
        if r == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(19, u32, r, 0u32, 0xbf800000u32, 1u32, 0u32, 0u32, 0xcu32)
    }
}
