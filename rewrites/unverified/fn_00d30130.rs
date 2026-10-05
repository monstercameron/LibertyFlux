// original: 0x00d30130 task_navmesh_route_request (proposed)

/// Request a navigation route through the navmesh system, in stages.
///
/// `a0` points to the task object: dword at `+0x270` holds flag
/// `0x10000000` (request pending; cleared when the request starts), dword at
/// `+0xb30` is the navmesh context (with a word at `+0x38` that must be
/// nonzero), dword at `+0x20` is a position object (coordinates at
/// `+0x30/+0x34/+0x38`) and dword at `+0x224` is a config object. `a1` is a
/// mode word passed to one query; `a2` is a float word passed on to a later
/// stage. Nine callees do the work: an obstacle-buffer constructor (thiscall
/// on a frame buffer, fills the probe words), two route queries (cdecl, 5
/// and 7 words; nonzero low byte means hit), a route-info fetch (cdecl, 2
/// words; its answer's float at `+0x1c` must exceed 0.1), a delta consumer
/// (thiscall on the delta triple), a scalar stage (cdecl, 1 word), a route
/// builder (thiscall, 7 words), a route finalizer (thiscall, 3 words) and a
/// route commit (thiscall, no words).
///
/// Any failed guard or query ends the request silently (no return value). On
/// the full path the constructor's probe floats are subtracted from the
/// position coordinates (the third lane subtracts the coordinate from
/// itself, yielding positive zero, or NaN from NaN, exactly as written) and
/// handed with the probe words through the remaining stages in order. All
/// float operations run in the original's operand order.
///
/// Original: 0x00d30130 (cdecl: three stack words, caller cleans, no return
/// value). The constructor lives in the executable's encrypted first
/// megabyte, but the call site is a direct call in plain code, so the
/// checker intercepts it like any other callee.
lf_checker_rt::export!(cdecl, rw_00d30130(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const REQ_FLAG: u32 = 0x270;
        const REQ_BIT: u32 = 0x10000000;
        const NAVCTX: u32 = 0xb30;
        const CTX_READY: u32 = 0x38;
        const POS_OBJ: u32 = 0x20;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const POS_Z: u32 = 0x38;
        const CFG_OBJ: u32 = 0x224;
        const CFG_OFF: u32 = 0x84;
        const INFO_RATIO: u32 = 0x1c;
        const QUARTER: f32 = f32::from_bits(0x3e800000); // 0.25
        const TENTH: f32 = f32::from_bits(0x3dcccccd); // 0.1
        const ONE_F: f32 = 1.0;
        const CTOR: u32 = 0;
        const QUERY1: u32 = 1;
        const QUERY2: u32 = 2;
        const INFO: u32 = 3;
        const DELTA: u32 = 4;
        const SCALAR: u32 = 5;
        const BUILD: u32 = 6;
        const FINALIZE: u32 = 7;
        const COMMIT: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let flags = rd32(a0 + REQ_FLAG);
        if flags & REQ_BIT == 0 {
            return 0;
        }
        ((a0 + REQ_FLAG) as *mut u32).write_unaligned(flags & !REQ_BIT);
        let ctx = rd32(a0 + NAVCTX);
        if ctx == 0 {
            return 0;
        }
        if rd32(ctx + CTX_READY) == 0 {
            return 0;
        }
        // Obstacle buffer; the constructor fills the probe words.
        let mut obuf = [0u32; 24];
        let _: u32 = lf_checker_rt::callee_thiscall!(CTOR, u32, obuf.as_mut_ptr() as u32);
        let pos = rd32(a0 + POS_OBJ);
        let e30 = pos.wrapping_add(POS_X);
        let q1: u32 = lf_checker_rt::callee_cdecl!(
            QUERY1,
            u32,
            ctx,
            e30,
            QUARTER.to_bits(),
            obuf.as_mut_ptr() as u32,
            1u32
        );
        if q1 & 0xff == 0 {
            let q2: u32 = lf_checker_rt::callee_cdecl!(
                QUERY2,
                u32,
                ctx,
                e30,
                a1,
                QUARTER.to_bits(),
                obuf.as_mut_ptr() as u32,
                1u32,
                1u32
            );
            if q2 & 0xff == 0 {
                return 0;
            }
        }
        let tag = ((obuf.as_ptr() as *const u8).add(0x52) as *const u16).read_unaligned() as u32;
        let info: u32 = lf_checker_rt::callee_cdecl!(INFO, u32, ctx, tag);
        if info == 0 {
            return 0;
        }
        if !(rdf(info + INFO_RATIO) > TENTH) {
            return 0;
        }
        let ox = f32::from_bits(obuf[0x10 / 4]);
        let oy = f32::from_bits(obuf[0x14 / 4]);
        let oz = f32::from_bits(obuf[0x18 / 4]);
        let cx = rdf(pos + POS_X);
        let cy = rdf(pos + POS_Y);
        let cz = rdf(pos + POS_Z);
        let mut dbuf = [0u32; 3];
        dbuf[0] = sub(cx, ox).to_bits();
        dbuf[1] = sub(cy, oy).to_bits();
        dbuf[2] = sub(cz, cz).to_bits();
        let _: u32 = lf_checker_rt::callee_thiscall!(DELTA, u32, dbuf.as_mut_ptr() as u32);
        let s: u32 = lf_checker_rt::callee_cdecl!(SCALAR, u32, a2);
        let x20 = [ox.to_bits(), oy.to_bits(), cz.to_bits()];
        let mut x04 = [0u32; 1];
        let mut x90 = [0u32; 4];
        let w82 = ((obuf.as_ptr() as *const u8).add(0x52) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_thiscall!(
            BUILD,
            u32,
            x90.as_mut_ptr() as u32,
            ctx,
            x04.as_mut_ptr() as u32,
            x20.as_ptr() as u32,
            ONE_F.to_bits(),
            w82,
            s,
            0u32
        );
        let cfg = rd32(a0 + CFG_OBJ).wrapping_add(CFG_OFF);
        let mut flag = [0u8; 1];
        flag[0] = 1;
        let _: u32 = lf_checker_rt::callee_thiscall!(FINALIZE, u32, cfg, x90.as_mut_ptr() as u32, 0u32, 1u32);
        let _ = &flag;
        let _: u32 = lf_checker_rt::callee_thiscall!(COMMIT, u32, x90.as_mut_ptr() as u32);
        0
    }
});
