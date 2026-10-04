// original: 0x00B6E090 task_advance_position (proposed)

// Advance a position toward its target, or lower it when the owner is idle.
//
// Arguments (thiscall): `this` is the task object, `tgt` a descriptor struct,
// `pos` points to three floats (x, y, z) updated in place. Returns 1 when the
// position was updated, 0 when the step was refused. Only the low byte of the
// return value is significant (the original leaves the upper bytes stale).
//
// Layout read: `owner` = u32 at `this+0x14`; `mode` = u32 at `owner+0x1304`;
// `anchor` = u32 at `tgt+0x20` (its x/y/z at +0x30/+0x34/+0x38); `helper` =
// u32 at `tgt+0xA80`; the argument to the first scalar callee is the u32 at
// `this+0x18`.
//
// Behavior: when `mode` is 2 (idle), subtract 1.0 from `pos.z` and return 1.
// Otherwise let dx/dy/dz be `pos - anchor`; if dx*dx+dy*dy+dz*dz (formed as
// dz*dz + (dy*dy + dx*dx)) exceeds 64.0, return 0. Fetch a direction triple
// through the owner's virtual slot 0xEC (one out-pointer argument); if its
// squared length (x*x + y*y, then + z*z) is below 0.05, return 0. Call the
// first scalar helper with `this+0x18`, the second virtual slot 0xC on
// helper with (out-word, 4), and the second scalar helper with (out-word
// pointer, first result); if the second result is below 0.01, return 0.
// Else scale the direction by sqrt(dy*dy + dx*dx)/second-result and add it to
// `pos` (x and y as (dir*t)+pos, z as pos+(dir*t)), then return 1. The
// security-cookie check after the return value is set is forwarded to its
// callee stand-in, which preserves the registers.
//
// NaN notes: an unordered distance comparison takes the proceed path (the
// original branches on `jbe`/`ja`), which the `>`-form comparisons below
// reproduce. All float operations keep the original's SSE operand order,
// pinned with `black_box` so the compiler cannot reassociate.
lf_checker_rt::export!(thiscall, rw_00b6e090(this: u32, tgt: u32, pos: u32) -> u32 {
    unsafe {
        const OWNER_OFF: u32 = 0x14;
        const ARG_OFF: u32 = 0x18;
        const MODE_OFF: u32 = 0x1304;
        const IDLE_MODE: u32 = 2;
        const ANCHOR_OFF: u32 = 0x20;
        const ANCHOR_X: u32 = 0x30;
        const ANCHOR_Y: u32 = 0x34;
        const ANCHOR_Z: u32 = 0x38;
        const HELPER_OFF: u32 = 0xA80;
        const VT_DIRECTION: u32 = 0xEC;
        const VT_ADJUST: u32 = 0x0C;
        const ADJUST_KIND: u32 = 4;
        const STEP_DOWN: f32 = f32::from_bits(0x3F80_0000); // 1.0
        const MAX_DIST2: f32 = f32::from_bits(0x4280_0000); // 64.0
        const MIN_DIR2: f32 = f32::from_bits(0x3D4C_CCCD); // 0.05
        const MIN_SCALE: f32 = f32::from_bits(0x3C23_D70A); // 0.01
        const SCALAR1_CALLEE: u32 = 3;
        const SCALAR2_CALLEE: u32 = 4;
        const COOKIE_CALLEE: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsqrt(v: f32) -> f32 {
            core::hint::black_box(v).sqrt()
        }

        let owner = rd32(this + OWNER_OFF);
        if rd32(owner + MODE_OFF) == IDLE_MODE {
            let z = rdf(pos + 8);
            wrf(pos + 8, fsub(z, STEP_DOWN));
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return 1;
        }

        let anchor = rd32(tgt + ANCHOR_OFF);
        let dx = fsub(rdf(pos), rdf(anchor + ANCHOR_X));
        let dy = fsub(rdf(pos + 4), rdf(anchor + ANCHOR_Y));
        let dz = fsub(rdf(pos + 8), rdf(anchor + ANCHOR_Z));
        let partial = fadd(fmul(dy, dy), fmul(dx, dx));
        let dist2 = fadd(fmul(dz, dz), partial);
        if dist2 > MAX_DIST2 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return 0;
        }

        let mut dir = [0u32; 3];
        let fetch: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(owner) + VT_DIRECTION) as usize);
        fetch(owner, dir.as_mut_ptr() as u32);
        let dirx = f32::from_bits(dir[0]);
        let diry = f32::from_bits(dir[1]);
        let dirz = f32::from_bits(dir[2]);
        let dir2 = fadd(fadd(fmul(dirx, dirx), fmul(diry, diry)), fmul(dirz, dirz));
        if MIN_DIR2 > dir2 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return 0;
        }

        let first: f32 = lf_checker_rt::callee_cdecl!(SCALAR1_CALLEE, f32, rd32(this + ARG_OFF));
        let helper = rd32(tgt + HELPER_OFF);
        let mut slot = [0u32; 1];
        let adjust: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(helper) + VT_ADJUST) as usize);
        adjust(helper, slot.as_mut_ptr() as u32, ADJUST_KIND);
        let second: f32 =
            lf_checker_rt::callee_cdecl!(SCALAR2_CALLEE, f32, slot.as_mut_ptr() as u32, first.to_bits());
        if MIN_SCALE > second {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return 0;
        }

        let t = core::hint::black_box(fsqrt(partial)) / core::hint::black_box(second);
        wrf(pos, fadd(fmul(dirx, t), rdf(pos)));
        wrf(pos + 4, fadd(fmul(diry, t), rdf(pos + 4)));
        wrf(pos + 8, fadd(rdf(pos + 8), fmul(dirz, t)));
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
        1
    }
});
