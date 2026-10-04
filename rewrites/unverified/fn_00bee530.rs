// original: 0x00bee530 ped_task_lerp_position_heading (proposed)

/// Blend a ped task's position and heading from the current pose toward a
/// target pose by factor `t`, writing the result into a task-state block.
///
/// `this` is the current pose (position floats at `+0x70/+0x74/+0x78`,
/// heading at `+0x7c`); `src` is the target pose with the same layout; `t`
/// is the blend factor as float bits. Each position component is the linear
/// blend `(src-this)*t+this` in that operation order: x lands in
/// `dst+0x1ed4`, y in `dst+0x1ec0`, and z is passed (with a trailing zero
/// word) to the virtual slot at `+0xf4` of the object at `dst`.
///
/// The heading takes the short way around the circle: when the absolute
/// difference exceeds pi the source heading is first shifted down by 2*pi,
/// then blended the same way; if that result falls below -2*pi it is shifted
/// back up by 2*pi. The final heading is stored twice, at `dst+0x1ed8` and
/// `dst+0x1edc`. A NaN difference compares unordered and takes the short
/// (unshifted) path, matching the original's branch-free `comiss`+`jbe`.
///
/// Original: thiscall, three stack words (`dst`, `src`, `t`), no meaningful
/// return value (eax holds stale stack data on exit).
lf_checker_rt::export!(thiscall, rw_00bee530(this: u32, dst: u32, src: u32, t: u32) -> u32 {
    unsafe {
        const POS_X: u32 = 0x70;
        const POS_Y: u32 = 0x74;
        const POS_Z: u32 = 0x78;
        const HEADING: u32 = 0x7c;
        const OUT_X: u32 = 0x1ed4;
        const OUT_Y: u32 = 0x1ec0;
        const OUT_H: u32 = 0x1ed8;
        const OUT_H_MIRROR: u32 = 0x1edc;
        const VTABLE_SLOT_Z: u32 = 0xf4;
        const PI: f32 = f32::from_bits(0x40490fdb);
        const TAU: f32 = f32::from_bits(0x40c90fdb);
        const NEG_TAU: f32 = f32::from_bits(0xc0c90fdb);
        const ABS_MASK: u32 = 0x7fff_ffff;

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
        #[inline(always)]
        fn absf(a: f32) -> f32 {
            f32::from_bits(a.to_bits() & ABS_MASK)
        }

        let tt = f32::from_bits(t);
        wrf(dst + OUT_X, add(mul(sub(rdf(src + POS_X), rdf(this + POS_X)), tt), rdf(this + POS_X)));
        wrf(dst + OUT_Y, add(mul(sub(rdf(src + POS_Y), rdf(this + POS_Y)), tt), rdf(this + POS_Y)));
        let z = add(mul(sub(rdf(src + POS_Z), rdf(this + POS_Z)), tt), rdf(this + POS_Z));
        // Indirect call through the destination object's table, exactly like
        // the original; the checker plants its recorder stub at the slot.
        let slot: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(dst) + VTABLE_SLOT_Z) as usize);
        slot(dst, z.to_bits(), 0);

        let cur = rdf(this + HEADING);
        let want = rdf(src + HEADING);
        // Short path unless the gap exceeds half a turn (NaN takes it too).
        let blended = if !(absf(sub(cur, want)) > PI) {
            add(mul(sub(want, cur), tt), cur)
        } else {
            let mut b = add(mul(sub(sub(want, TAU), cur), tt), cur);
            if NEG_TAU > b {
                b = add(b, TAU);
            }
            b
        };
        wrf(dst + OUT_H, blended);
        wrf(dst + OUT_H_MIRROR, blended);
        0
    }
});
