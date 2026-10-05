// original: 0x00D4AD30 CTaskSimpleRunNamedAnim::vf17

#![allow(unsafe_code)]

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
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
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}

/// Run-named-anim task update (`vf17`): run one tick for the ped.
///
/// `this` is the task, `ped` the ped. When the vector flag at `+0xb0` is
/// set and the ped answers its pose hook, the facing vector is refreshed
/// from the game's data. A start bit at `+0x19` runs the pre-roll hook.
/// A finished task (bit 0 at `+0x18`) resolves its animation: with the
/// instant bit (`0x80`) the result is handed off at once, otherwise the
/// target point is measured and, when the blend rate is above its floor
/// and the target is farther than one unit, the clip is retimed
/// proportionally to the negated rate. An unfinished task first runs the
/// weapon-state hooks when its mode bits ask for them, then, once its
/// timer elapses, parks the animation and marks itself finished; with no
/// live animation it re-resolves, keeps a live one whose loop word is set,
/// and otherwise restarts through the anim-start helper. Returns 1 when
/// the tick finishes the task, else bit 0 of `+0x18`.
///
/// Float order follows the original exactly (differences, squares,
/// low-lane square root); the two threshold tests treat an unordered
/// comparison as below-or-equal, like the original's branch.
///
/// Original: 0x00D4AD30 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00D4AD30(this: u32, ped: u32) -> u32 {
    unsafe {
        const TIMER: u32 = 0x011735B4;
        const VEC0: u32 = 0x01050D30;
        const VEC1: u32 = 0x01050D34;
        const VEC2: u32 = 0x01050D38;
        const VEC3: u32 = 0x01050D3C;
        const C_RATE: u32 = 0x00FE8BB0;
        const C_DIST: u32 = 0x00FE88E8;
        const C_SCALE: u32 = 0x00FE8D94;
        const POSE_HOOK: u32 = 0;
        const PREROLL: u32 = 1;
        const ANIM_FIND: u32 = 2;
        const HANDOFF: u32 = 3;
        const MEASURE: u32 = 4;
        const RETIME: u32 = 5;
        const WEAPON_A: u32 = 6;
        const WEAPON_B: u32 = 7;
        const WEAPON_C: u32 = 8;
        const PARK: u32 = 9;
        const RESTART: u32 = 10;

        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn timer() -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(TIMER) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn fb(bits: u32) -> f32 {
            f32::from_bits(core::hint::black_box(bits))
        }
        #[inline(always)]
        fn fop(a: u32, b: u32, f: fn(f32, f32) -> f32) -> u32 {
            f(fb(a), fb(b)).to_bits()
        }
        /// Below-or-equal-or-unordered, like the original's branch: true
        /// unless `a` is ordered and strictly greater than `b`.
        #[inline(always)]
        fn fleu(a: u32, b: u32) -> bool {
            !(fb(a) > fb(b))
        }

        if rd32(this + 0xb0) & 0x200000 != 0 && ped != 0 {
            let slot = rd32(rd32(ped) + 0xd0);
            let pose: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
            if pose(ped) != 0 {
                let pose2: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
                pose2(ped);
                wr32(ped + 0xbd0, g32(VEC0));
                wr32(ped + 0xbd4, g32(VEC1));
                wr32(ped + 0xbd8, g32(VEC2));
                wr32(ped + 0xbdc, g32(VEC3));
            }
        }
        if rd8(this + 0x19) & 1 != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(PREROLL, u32, this, ped);
        }
        let flags = rd8(this + 0x18);
        if flags & 1 != 0 {
            let anim: u32 = lf_checker_rt::callee_thiscall!(ANIM_FIND, u32, rd32(ped + 0x78),
                this + 0x80, this + 0x60);
            if anim == 0 {
                return 1;
            }
            if flags & 0x80 != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(HANDOFF, u32, rd32(ped + 0x78), anim);
                return 1;
            }
            let mut out = [0u32; 2];
            let _: u32 = lf_checker_rt::callee_thiscall!(MEASURE, u32, ped,
                (&mut out as *mut u32) as u32, 0x4b3);
            let rate = rd32(this + 0x98);
            if fleu(rate, g32(C_RATE)) {
                return 1;
            }
            let base = rd32(ped + 0x20);
            let dx = fop(rd32(base + 0x34), out[0], |x, y| x - y);
            let dy = fop(rd32(base + 0x30), 0, |x, y| x - y);
            let dz = fop(rd32(base + 0x38), out[1], |x, y| x - y);
            let dx2 = fop(dx, dx, |x, y| x * y);
            let dy2 = fop(dy, dy, |x, y| x * y);
            let dz2 = fop(dz, dz, |x, y| x * y);
            let dist = fop(fop(dx2, dy2, |x, y| x + y), dz2, |x, y| x + y);
            let dist = fb(dist).sqrt().to_bits();
            if fleu(dist, g32(C_DIST)) {
                return 1;
            }
            wr32(anim + 4, rd32(anim + 4) | 0x4000);
            let scaled = fop(rate, g32(C_SCALE), |x, y| x * y);
            let _: u32 = lf_checker_rt::callee_thiscall!(RETIME, u32, anim, scaled);
            return 1;
        }
        let mode = rd32(this + 0xb0);
        if flags & 2 != 0 && (mode & 0x8000 == 0 || mode & 0x20 != 0) {
            let wb = rd32(ped + 0x224).wrapping_add(0x84);
            let _: u32 = lf_checker_rt::callee_thiscall!(WEAPON_A, u32, wb, 1);
            let _: u32 = lf_checker_rt::callee_thiscall!(WEAPON_B, u32, wb);
            if rd8(ped + 0x212) != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(WEAPON_C, u32, ped, 1);
            }
        }
        if rd8(this + 0xa8) != 0 {
            if rd8(this + 0xa9) != 0 {
                wr32(this + 0xa0, timer());
                wr8(this + 0xa9, 0);
            }
            let now = timer();
            let end = rd32(this + 0xa4).wrapping_add(rd32(this + 0xa0));
            if (end as i32) <= (now as i32) {
                if rd32(this + 0x14) != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(PARK, u32, rd32(this + 0x14), this);
                    let _: u32 = lf_checker_rt::callee_thiscall!(RETIME, u32, rd32(this + 0x14), 0xC0800000);
                }
                wr8(this + 0x18, rd8(this + 0x18) | 1);
                wr32(this + 0x14, 0);
                return 1;
            }
        }
        if rd32(this + 0x14) != 0 {
            return (rd8(this + 0x18) & 1) as u32;
        }
        let anim: u32 = lf_checker_rt::callee_thiscall!(ANIM_FIND, u32, rd32(ped + 0x78),
            this + 0x80, this + 0x60);
        if anim != 0 && rd32(anim + 0x24) != 0 {
            return 1;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(RESTART, u32, this, ped);
        (rd8(this + 0x18) & 1) as u32
    }
});
