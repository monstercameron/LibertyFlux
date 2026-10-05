// original: 0x00cd8b70 CTaskComplexSeekEntityAiming::vf19

/// Range-split task setup for a seek-entity-aiming task (thiscall, one stack word).
///
/// `this` is the task object: `+0x14` the target record, `+0x18` a tuning
/// float passed to the setup call, `+0x1c` the engagement-range float. `p0`
/// is the event record whose `+0x20` points at the subject's position block
/// (x at `+0x30`, y at `+0x34`); unlike its sibling this function does not
/// null-check it, so a null event faults on both sides.
///
/// The anchor position comes from the target record: when its `+0x20` link
/// is set the anchor is that block's `+0x30`/`+0x34`, otherwise the record's
/// own `+0x10`/`+0x14`. The planar offset (subject minus anchor) is formed
/// with two scalar-float subtracts and its squared length kept in a frame
/// slot (the original also overwrites its incoming argument slot with it;
/// that slot is dead after the callee-popped return, so the rewrite leaves
/// it alone and the proof runs with the stack check off).
///
/// A null target record returns 0 at once. Otherwise a first builder fetched
/// through the manager getter runs the setup call (target, 50000, 1000, the
/// tuning bits, 2.0, 2.0, 1) whose answer is kept, or zero when the builder
/// is null. Then the squared range is compared against the squared offset
/// length: strictly greater takes the near path, anything else (including an
/// unordered NaN compare) the far path. Near fetches a second builder and
/// runs the build call (2, target, 0, 60.0, 0, 1, 1, 1.0); far fetches one,
/// runs the no-argument initialiser on it and stamps the fresh object (table
/// address at `+0x00`, zero at `+0x14`, zero byte at `+0x18`, zero at
/// `+0x1c`). A null builder on either path yields zero instead.
///
/// Finally a last builder is fetched: when null the function returns 0,
/// otherwise it runs the finish call (setup answer, path value, 0, 0) and
/// returns its answer.
///
/// Original: 0x00cd8b70 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00cd8b70(this: u32, p0: u32) -> u32 {
    unsafe {
        const TARGET_OFF: u32 = 0x14;
        const TUNE_OFF: u32 = 0x18;
        const RANGE_OFF: u32 = 0x1c;
        const POS_LINK: u32 = 0x20;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const ANCHOR_X: u32 = 0x10;
        const ANCHOR_Y: u32 = 0x14;
        const FRESH_VTABLE: u32 = 0x00eb391c;
        const MANAGER: u32 = 0x0167e2a0;
        const TWO_BITS: u32 = 0x4000_0000;
        const ONE_BITS: u32 = 0x3f80_0000;
        const SIXTY: u32 = 0x4270_0000;
        const CALLEE_GET1: u32 = 1;
        const CALLEE_GET_NEAR: u32 = 2;
        const CALLEE_GET_FAR: u32 = 3;
        const CALLEE_GET_LAST: u32 = 4;
        const CALLEE_SETUP: u32 = 5;
        const CALLEE_BUILD: u32 = 6;
        const CALLEE_INIT: u32 = 7;
        const CALLEE_FINISH: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let target = rd32(this + TARGET_OFF);
        if target == 0 {
            return 0;
        }
        let mid = rd32(target + POS_LINK);
        let (ax, ay) = if mid != 0 {
            (rd32(mid + POS_X), rd32(mid + POS_Y))
        } else {
            (rd32(target + ANCHOR_X), rd32(target + ANCHOR_Y))
        };
        let subj = rd32(p0 + POS_LINK);
        let dx = sub(
            f32::from_bits(rd32(subj + POS_X)),
            f32::from_bits(ax),
        );
        let dy = sub(
            f32::from_bits(rd32(subj + POS_Y)),
            f32::from_bits(ay),
        );
        let dist2 = add(mul(dx, dx), mul(dy, dy));
        // First builder + setup call (or zero when null).
        let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
        let g1: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GET1, u32, mgr);
        let tune = rd32(this + TUNE_OFF);
        let first = if g1 == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(
                CALLEE_SETUP, u32, g1, target, 50000u32, 1000u32, tune, TWO_BITS, TWO_BITS, 1u32
            )
        };
        let r = f32::from_bits(rd32(this + RANGE_OFF));
        let r2 = mul(r, r);
        // comiss+ja semantics: near only when strictly greater (NaN goes far).
        let near = r2 > dist2;
        let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
        let path_value = if near {
            let g2: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GET_NEAR, u32, mgr);
            if g2 == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    CALLEE_BUILD, u32, g2, 2u32, target, 0u32, SIXTY, 0u32, 1u32, 1u32, ONE_BITS
                )
            }
        } else {
            let g3: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GET_FAR, u32, mgr);
            if g3 == 0 {
                0
            } else {
                let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_INIT, u32, g3);
                wr32(g3, lf_checker_rt::relocated(FRESH_VTABLE));
                wr32(g3 + 0x14, 0);
                ((g3 + 0x18) as *mut u8).write(0);
                wr32(g3 + 0x1c, 0);
                g3
            }
        };
        let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
        let g4: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GET_LAST, u32, mgr);
        if g4 == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CALLEE_FINISH, u32, g4, first, path_value, 0u32, 0u32)
    }
});
