// original: 0x00bee3f0 ped_task_drive_blend (proposed)

/// Drive one ped-task blend step: either blend toward a target pose through
/// the lerp helper, or snap the current pose into the state block, then
/// refresh the packed mode fields.
///
/// `this` is the task (pose floats at `+0x70/+0x74/+0x78/+0x7c`, mode bytes
/// at `+0x80/+0x81`, level byte at `+0x82`); `dst` the state block; `target`
/// an optional target pose (null means none); `t` the blend factor as float
/// bits. A setup callee runs first with (`dst`, `target`, `t`).
///
/// The snap path runs when the state block carries flag `0x400` at `+0x24`,
/// when `t` is zero, or when the readiness callee answers `0xffffff`: the
/// pose is copied (`this+0x70/+0x74` to `dst+0x1ed4/+0x1ec0`, `this+0x78`
/// through the virtual slot at `+0xf4` of `dst` with a trailing zero word,
/// `this+0x7c` to both `dst+0x1ed8` and `dst+0x1edc`). Otherwise, when the
/// readiness callee answers nonzero on its second poll and `target` is not
/// null, the lerp helper blends into the state block. A NaN factor takes the
/// blend path, matching the original's `ucomiss`+`lahf` test.
///
/// The tail always runs: the top two bits of `this+0x80` go to `dst+0xd14`,
/// bit 0 of `this+0x81` to `dst+0xd4f`, the three 2-bit fields of `this+0x80`
/// are encoded to floats at `dst+0x1ef4/+0x1ef8/+0x1efc` by the code-to-float
/// helper (x87 result), and the level byte becomes a 0..1 float at
/// `dst+0x1f68` by scaling with 1/255.
///
/// Original: thiscall, three stack words (`dst`, `target`, `t`), no
/// meaningful return value.
lf_checker_rt::export!(thiscall, rw_00bee3f0(this: u32, dst: u32, target: u32, t: u32) -> u32 {
    unsafe {
        const POS_X: u32 = 0x70;
        const POS_Y: u32 = 0x74;
        const POS_Z: u32 = 0x78;
        const HEADING: u32 = 0x7c;
        const MODE: u32 = 0x80;
        const FLAG: u32 = 0x81;
        const LEVEL: u32 = 0x82;
        const DST_FLAGS: u32 = 0x24;
        const SNAP_FLAG: u32 = 0x400;
        const ST_X: u32 = 0x1ed4;
        const ST_Y: u32 = 0x1ec0;
        const ST_H: u32 = 0x1ed8;
        const ST_H_MIRROR: u32 = 0x1edc;
        const ST_F0: u32 = 0x1ef4;
        const ST_F1: u32 = 0x1ef8;
        const ST_F2: u32 = 0x1efc;
        const ST_TOP: u32 = 0xd14;
        const ST_FLAG: u32 = 0xd4f;
        const ST_LEVEL: u32 = 0x1f68;
        const VTABLE_SLOT_Z: u32 = 0xf4;
        const NOT_READY: u32 = 0xffffff;
        const INV_255: f32 = f32::from_bits(0x3b808081);
        const CALLEE_SETUP: u32 = 1;
        const CALLEE_READY: u32 = 2;
        const CALLEE_LERP: u32 = 4;
        const CALLEE_ENCODE: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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

        lf_checker_rt::callee_thiscall!(CALLEE_SETUP, u32, this, dst, target, t);
        let tt = f32::from_bits(t);
        // `tt == 0.0` is false for NaN, so NaN takes the blend path.
        let snap = rd32(dst + DST_FLAGS) & SNAP_FLAG != 0
            || tt == 0.0
            || lf_checker_rt::callee_thiscall!(CALLEE_READY, u32, this) == NOT_READY;
        if snap {
            wr32(dst + ST_X, rd32(this + POS_X));
            wr32(dst + ST_Y, rd32(this + POS_Y));
            // Indirect height call through the state's table, exactly like
            // the original; the checker plants its recorder stub at the slot.
            let slot: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(dst) + VTABLE_SLOT_Z) as usize);
            slot(dst, rd32(this + POS_Z), 0);
            let h = rdf(this + HEADING);
            wrf(dst + ST_H, h);
            wrf(dst + ST_H_MIRROR, h);
        } else if lf_checker_rt::callee_thiscall!(CALLEE_READY, u32, this) != 0 && target != 0 {
            lf_checker_rt::callee_thiscall!(CALLEE_LERP, u32, this, dst, target, t);
        }

        wr8(dst + ST_TOP, rd8(this + MODE) >> 6);
        wr8(dst + ST_FLAG, rd8(this + FLAG) & 1);
        let m = rd8(this + MODE);
        let f0: f32 = lf_checker_rt::callee_thiscall!(CALLEE_ENCODE, f32, this, (m & 3) as u32);
        wrf(dst + ST_F0, f0);
        let f1: f32 = lf_checker_rt::callee_thiscall!(CALLEE_ENCODE, f32, this, ((m >> 2) & 3) as u32);
        wrf(dst + ST_F1, f1);
        let f2: f32 = lf_checker_rt::callee_thiscall!(CALLEE_ENCODE, f32, this, ((m >> 4) & 3) as u32);
        wrf(dst + ST_F2, f2);
        wrf(dst + ST_LEVEL, mul(rd8(this + LEVEL) as f32, INV_255));
        0
    }
});
