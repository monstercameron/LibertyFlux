// original: 0x00CDCD50 CTaskSimpleNMBalance::vf27 (merged symbol, vtable slot 27)

/// Balance-task update: forward the ped's position to the NaturalMotion
/// engine and keep the task's balance/brace messages alive.
///
/// `this` is the task, `ped` the ped. The work runs in four stages, each
/// guarded by the task and ped state:
///
/// 1. Retrigger stamp: when the ped's sub-object at `+0x16c` has mode bits
///    `0xc0` (bits 6-9 of its word at `+0x28`) and its flag at `+0x219` is
///    set, and the ped's timer at `+0x170` is above zero, the stamp global
///    is copied to the task word at `+0x24`.
/// 2. Position message: when the task object at `+0x28` exists, its anchor
///    point (the matrix at `+0x20` plus `0x30`, or the object itself plus
///    `0x10`) is read as three floats. A mode-`0xc0` anchor is first passed
///    to the anchor callee with code `0x4b5`; then a message carrying the
///    point as a vec3 slot is built on the stack and sent.
/// 3. Balance message: when the timer at `+0x50` is above zero a message is
///    built. Flag bits 9-10 of the word at `+0xc0` select a short form (one
///    cleared boolean slot, timer reset to zero). Otherwise the second
///    timer at `+0xe4` ticks down by the frame step and expiry also selects
///    the short form. The full form blends the lean vector at `+0x30`/`+0x34`
///    (scaled by its own length through the square-root callee when the
///    squared length exceeds 0.01, else the ped matrix row at `+0x10`) with
///    two boolean slots, one vec3 slot and the `+0x50` timer as a float slot.
/// 4. Shared update plus brace message: the shared timer routine (the
///    function at 0x00CDCBF0) runs on the sub-object at `+0x60`. When the
///    flag at `+0x54` is set and the `+0xc0` hold bits are clear, a set flag
///    at `+0x56` selects a tiny message (one cleared boolean slot, flag
///    cleared); set hold bits select the brace message instead: the reach
///    callee fills a point, its length scaled by the per-profile factor
///    table feeds a clamped float slot, and profile booleans/floats from the
///    same table fill the remaining slots.
///
/// Returns the last callee's answer. Original: 0x00CDCD50 (thiscall, one
/// stack word). Float operation order is the original's.
lf_checker_rt::export!(thiscall, rw_00cdcd50(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_STAMP: u32 = 0x24;
        const TASK_ANCHOR: u32 = 0x28;
        const TASK_LEAN_X: u32 = 0x30;
        const TASK_LEAN_Y: u32 = 0x34;
        const TASK_TIMER: u32 = 0x50;
        const TASK_FLAG54: u32 = 0x54;
        const TASK_FLAG56: u32 = 0x56;
        const TASK_SUB: u32 = 0x60;
        const TASK_HOLD: u32 = 0xc0;
        const TASK_PROFILE: u32 = 0xe0;
        const TASK_TIMER2: u32 = 0xe4;
        const PED_MATRIX: u32 = 0x20;
        const PED_SUB: u32 = 0x16c;
        const PED_TIMER: u32 = 0x170;
        const PED_NMCTX: u32 = 0x7b4;
        const ANCHOR_MATRIX: u32 = 0x20;
        const ANCHOR_MODE: u32 = 0x28;
        const ANCHOR_FLAG: u32 = 0x219;
        const MODE_MASK: u32 = 0x3c0;
        const MODE_LIVE: u32 = 0xc0;
        const ANCHOR_CODE: u32 = 0x4b5;
        const LEN_LIMIT_BITS: u32 = 0x3c23d70a; // 0.01
        const DT_STEP: u32 = 0x11735bc;
        const STAMP_SRC: u32 = 0x11735b4;
        const ZERO_F: u32 = 0xfe8628;
        const NM_CTOR: u32 = 2;
        const NM_SET_BOOL: u32 = 3;
        const NM_SEND: u32 = 4;
        const NM_DTOR: u32 = 5;
        const NM_SET_VEC3: u32 = 7;
        const NM_SET_FLOAT: u32 = 8;
        const ANCHOR_CALLEE: u32 = 9;
        const SQRT_CALLEE: u32 = 10;
        const REACH_CALLEE: u32 = 11;
        const SHARED_FN: u32 = 12;
        const COOKIE: u32 = 6;

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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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
        unsafe fn anchor_point(obj: u32) -> u32 {
            unsafe {
                let m = rd32(obj + ANCHOR_MATRIX);
                if m != 0 { m.wrapping_add(0x30) } else { obj.wrapping_add(0x10) }
            }
        }

        let zero = f32::from_bits(g32(ZERO_F));
        let mut eax = 0u32;

        // Stage 1: retrigger stamp.
        let psub = rd32(ped + PED_SUB);
        if psub != 0
            && (rd32(psub + ANCHOR_MODE) & MODE_MASK) == MODE_LIVE
            && rd8(psub + ANCHOR_FLAG) != 0
            && rdf(ped + PED_TIMER) > zero
        {
            wr32(this + TASK_STAMP, g32(STAMP_SRC));
        }

        // Stage 2: position message.
        let anchor = rd32(this + TASK_ANCHOR);
        if anchor != 0 {
            let p = anchor_point(anchor);
            let px = rdf(p);
            let py = rdf(p.wrapping_add(4));
            let pz = rdf(p.wrapping_add(8));
            if (rd32(anchor + ANCHOR_MODE) & MODE_MASK) == MODE_LIVE {
                let mut probe = [0u32; 3];
                probe[0] = px.to_bits();
                probe[1] = py.to_bits();
                probe[2] = pz.to_bits();
                eax = lf_checker_rt::callee_thiscall!(
                    ANCHOR_CALLEE, u32, anchor,
                    probe.as_mut_ptr() as u32, ANCHOR_CODE);
            }
            let mut msg = [0u32; 16];
            let buf = msg.as_mut_ptr() as u32;
            eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SET_BOOL, u32, buf, g32(0x1051e18), 1);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SET_VEC3, u32, buf, g32(0x1051e1c), px.to_bits(), py.to_bits(), pz.to_bits());
            eax = lf_checker_rt::callee_thiscall!(
                NM_SEND, u32, rd32(ped + PED_NMCTX), g32(0x1051dfc), buf);
            eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf);
        }

        // Stage 3: balance message.
        if rdf(this + TASK_TIMER) > zero {
            let mut msg = [0u32; 16];
            let buf = msg.as_mut_ptr() as u32;
            eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf);
            let hold = rd32(this + TASK_HOLD);
            let short = ((hold >> 9) & 1) != 0 || ((hold >> 10) & 1) != 0;
            let mut short2 = short;
            if !short2 {
                let t = rdf(this + TASK_TIMER2);
                if t > 0.0 {
                    let rest = sub(t, f32::from_bits(g32(DT_STEP)));
                    wrf(this + TASK_TIMER2, rest);
                    if 0.0 >= rest {
                        short2 = true;
                    }
                }
            }
            if short2 {
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_BOOL, u32, buf, g32(0x1051cc8), 0);
                wr32(this + TASK_TIMER, 0);
            } else {
                let lx = rdf(this + TASK_LEAN_X);
                let ly = rdf(this + TASK_LEAN_Y);
                let x2 = mul(lx, lx);
                let y2 = mul(ly, ly);
                let len2 = add(x2, y2);
                let (vx, vy, vz);
                if len2 > f32::from_bits(LEN_LIMIT_BITS) {
                    let root: f32 = lf_checker_rt::callee_cdecl!(
                        SQRT_CALLEE, f32, len2.to_bits());
                    vx = mul(root, lx);
                    vy = mul(root, ly);
                    vz = mul(root, zero);
                    eax = root.to_bits();
                } else {
                    let m = rd32(ped + PED_MATRIX);
                    vx = rdf(m.wrapping_add(0x10));
                    vy = rdf(m.wrapping_add(0x14));
                    vz = rdf(m.wrapping_add(0x18));
                }
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_BOOL, u32, buf, g32(0x1051cc8), 1);
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_BOOL, u32, buf, g32(0x1051e98), 1);
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_VEC3, u32, buf, g32(0x1051e90), vx.to_bits(), vy.to_bits(), vz.to_bits());
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_FLOAT, u32, buf, g32(0x1051e94), rd32(this + TASK_TIMER));
            }
            eax = lf_checker_rt::callee_thiscall!(
                NM_SEND, u32, rd32(ped + PED_NMCTX), g32(0x1051e88), buf);
            eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf);
        }

        // Stage 4: shared update, then the brace message.
        eax = lf_checker_rt::callee_thiscall!(SHARED_FN, u32, this.wrapping_add(TASK_SUB), ped);
        if rd8(this + TASK_FLAG54) != 0 {
            let hold = rd32(this + TASK_HOLD);
            if ((hold >> 9) & 1) == 0 && ((hold >> 10) & 1) == 0 {
                if rd8(this + TASK_FLAG56) != 0 {
                    let mut msg = [0u32; 16];
                    let buf = msg.as_mut_ptr() as u32;
                    eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf);
                    eax = lf_checker_rt::callee_thiscall!(
                        NM_SET_BOOL, u32, buf, g32(0x1051cc8), 0);
                    eax = lf_checker_rt::callee_thiscall!(
                        NM_SEND, u32, rd32(ped + PED_NMCTX), g32(0x1051dcc), buf);
                    wr8(this + TASK_FLAG56, 0);
                    eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf);
                }
            } else {
                let mut pin = [0u32; 3];
                let mut pout = [0u32; 3];
                eax = lf_checker_rt::callee_thiscall!(
                    REACH_CALLEE, u32, ped, pout.as_mut_ptr() as u32,
                    pin.as_mut_ptr() as u32, 0, 0);
                let px = f32::from_bits(pout[0]);
                let py = f32::from_bits(pout[1]);
                let pz = f32::from_bits(pout[2]);
                let row = rd32(this + TASK_PROFILE).wrapping_mul(0xb0);
                let prof = |va: u32| unsafe { rd32(lf_checker_rt::relocated(va).wrapping_add(row)) };
                let proff = |va: u32| f32::from_bits(prof(va));
                let d2 = add(add(mul(px, px), mul(py, py)), mul(pz, pz));
                // Original order: packed square root of the sum, times the factor.
                let root = core::hint::black_box(d2).sqrt();
                let dist = mul(root, proff(0x171cb38));
                let mut msg = [0u32; 16];
                let buf = msg.as_mut_ptr() as u32;
                eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf);
                if rd8(this + TASK_FLAG56) == 0 {
                    eax = lf_checker_rt::callee_thiscall!(
                        NM_SET_BOOL, u32, buf, g32(0x1051cc8), 1);
                    eax = lf_checker_rt::callee_thiscall!(
                        NM_SET_FLOAT, u32, buf, g32(0x1051dd8), prof(0x171cb24));
                    eax = lf_checker_rt::callee_thiscall!(
                        NM_SET_FLOAT, u32, buf, g32(0x1051de4), prof(0x171cb28));
                    eax = lf_checker_rt::callee_thiscall!(
                        NM_SET_FLOAT, u32, buf, g32(0x1051dec), prof(0x171cb2c));
                    eax = lf_checker_rt::callee_thiscall!(
                        NM_SET_FLOAT, u32, buf, g32(0x1051df0), prof(0x171cb30));
                }
                let top = proff(0x171cb34);
                let clamped = if prof(0x171cbb8) & 0xff == 0 {
                    top
                } else if 0.0 > dist {
                    0.0
                } else if dist > top {
                    top
                } else {
                    dist
                };
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_FLOAT, u32, buf, g32(0x1051de8), clamped.to_bits());
                let w = proff(0x171cb94);
                let ge = if w >= zero { 1u32 } else { 0u32 };
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_BOOL, u32, buf, g32(0x1051df4), ge);
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_FLOAT, u32, buf, g32(0x1051df8), w.to_bits());
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SET_BOOL, u32, buf, g32(0x1051dd4), 0);
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SEND, u32, rd32(ped + PED_NMCTX), g32(0x1051dcc), buf);
                wr8(this + TASK_FLAG56, 1);
                eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf);
            }
        }

        lf_checker_rt::callee_stdcall!(COOKIE, u32,);
        eax
    }
});
