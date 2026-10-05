// original: 0x00DC8F40 CTaskComplexWalkRoundEntity::vf20

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

/// Advance a walk-round-entity task: steer around the target, or replan
/// when close enough or when the target changed.
///
/// `this+0x70` holds the objective `w` (null keeps nothing: return 0, as
/// does a negative flag byte at `this+0x9c`). With bit 1 of that byte set,
/// callee 1 refreshes `arg1`, then the offset from `arg1`'s point
/// (`[arg1+0x20]+0x30..`) to the objective's point (`[w+0x20]+0x30..`, or
/// `w+0x10..` when null) and to this task's own point (`+0x20..`) are both
/// normalised (reciprocal length, zero for a zero squared length) and
/// dotted; the dot against 0.707 picks which point callee 2 (on `arg1`)
/// steers toward: the task's own on a weak alignment, the objective's
/// otherwise.
///
/// When `[w+0x28]` masked with `0x3c0` is `0xc0` and the squared planar
/// distance from `arg1`'s point to this task's (`+0x30/+0x34`, in
/// `y²+x²` order) exceeds the float at `+0x84`, callee 3 (on `[this+8]`,
/// `arg1`, 1, 0) may still end the task (nonzero low byte: return 0).
///
/// An integer at `[arg1+0x224]+0x264` below the global threshold 30 takes
/// the replan half; at or above, the task finishes through its own virtual
/// slot `+0x0c` and three construction calls, returning `[this+8]`.
///
/// The replan half: callee 8 (on `this+0x78`) gates; on success the words
/// at `+0x78/+0x7c/+0x80` are refreshed (global, `[this+0x88]`, 1) and the
/// classifier pair runs as in the neighbouring tasks (nine-word prepare
/// with `[this+0x70]`, the seed at `[arg1+0x20]+0x38` and 0.25, then two
/// asks). The two answers land at `+0x94/+0x98` (callee 11, or a change in
/// either, marks the frame changed); unchanged frames with bit 0 of
/// `+0x9c` clear return `[this+8]`. Else the sub-task at `[this+8]` is
/// asked through its slot `+0x14` (`arg1`, 1, 0) unless its bit 0 at `+0x0c`
/// is set (a zero answer also returns `[this+8]`), bit 1 there is set, and
/// this task's slot `+0x4c` (`arg1`) runs last, its answer returned.
///
/// Original: 0x00DC8F40 (thiscall, one stack word).
export!(thiscall, rw_dc8f40(this: u32, arg1: u32) -> u32 {
    const OBJ_OFF: u32 = 0x70;
    const FLAG_OFF: u32 = 0x9c;
    const SELF_PT: u32 = 0x20;
    const DST_PT: u32 = 0x30;
    const SUB_OFF: u32 = 0x08;
    const BLK_OFF: u32 = 0x20;
    const MEMBER_OFF: u32 = 0x224;
    const MODE_MASK: u32 = 0x3c0;
    const MODE_WANT: u32 = 0xc0;
    const DIST_LIM_OFF: u32 = 0x84;
    const CMP_OFF: u32 = 0x264;
    const STATE_OFF: u32 = 0x78;
    const DOTLIM: f32 = f32::from_bits(0x3f34_fdf4);
    const ONE: f32 = 1.0;
    const QUARTER: f32 = 0.25;
    const GCMP: u32 = 0x00e9_d7fc;
    const GSTATE: u32 = 0x0117_35b4;

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
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read() }
    }
    unsafe fn rdf(a: u32) -> f32 {
        unsafe { f32::from_bits(rd32(a)) }
    }
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe fn wr8(a: u32, v: u8) {
        unsafe { (a as *mut u8).write(v) }
    }

    unsafe {
        let done0 = | | -> u32 {
            callee_cdecl!(14, u32, );
            0
        };
        let w = rd32(this.wrapping_add(OBJ_OFF));
        if w == 0 {
            return done0();
        }
        let flag = rd8(this.wrapping_add(FLAG_OFF));
        if (flag as i8) < 0 {
            return done0();
        }
        let a = rd32(arg1.wrapping_add(BLK_OFF));
        let apt = a.wrapping_add(DST_PT);
        if flag & 2 != 0 {
            let _: u32 = callee_thiscall!(1, u32, arg1, 1);
            let w2 = rd32(w.wrapping_add(BLK_OFF));
            let pp = if w2 == 0 { w.wrapping_add(0x10) } else { w2.wrapping_add(DST_PT) };
            let x = sub(rdf(pp), rdf(apt));
            let y = sub(rdf(pp.wrapping_add(4)), rdf(apt.wrapping_add(4)));
            let z = sub(rdf(pp.wrapping_add(8)), rdf(apt.wrapping_add(8)));
            let ex = sub(rdf(this.wrapping_add(SELF_PT)), rdf(apt));
            let ey = sub(rdf(this.wrapping_add(SELF_PT).wrapping_add(4)), rdf(apt.wrapping_add(4)));
            let ez = sub(rdf(this.wrapping_add(SELF_PT).wrapping_add(8)), rdf(apt.wrapping_add(8)));
            let l2a = add(add(mul(x, x), mul(y, y)), mul(z, z));
            let inv_a = if l2a == 0.0 { 0.0 } else { core::hint::black_box(ONE) / core::hint::black_box(l2a.sqrt()) };
            let nx = mul(x, inv_a);
            let ny = mul(y, inv_a);
            let nz = mul(z, inv_a);
            let l2b = add(add(mul(ey, ey), mul(ex, ex)), mul(ez, ez));
            let inv_b = if l2b == 0.0 { 0.0 } else { core::hint::black_box(ONE) / core::hint::black_box(l2b.sqrt()) };
            let dot = add(add(mul(mul(ex, inv_b), nx), mul(mul(ey, inv_b), ny)), mul(mul(ez, inv_b), nz));
            let steer = if dot > DOTLIM { pp } else { this.wrapping_add(SELF_PT) };
            let _: u32 = callee_thiscall!(2, u32, arg1, steer);
        }
        if rd32(w.wrapping_add(0x28)) & MODE_MASK == MODE_WANT {
            let dy = sub(rdf(apt.wrapping_add(4)), rdf(this.wrapping_add(DST_PT).wrapping_add(4)));
            let dx = sub(rdf(apt), rdf(this.wrapping_add(DST_PT)));
            if add(mul(dy, dy), mul(dx, dx)) > rdf(this.wrapping_add(DIST_LIM_OFF)) {
                let o = rd32(this.wrapping_add(SUB_OFF));
                let r3: u32 = callee_thiscall!(3, u32, o, arg1, 1, 0);
                if (r3 & 0xff) != 0 {
                    return done0();
                }
            }
        }
        let a2 = rd32(arg1.wrapping_add(MEMBER_OFF));
        if (rd32(a2.wrapping_add(CMP_OFF)) as i32) >= global::<i32>(GCMP).read() {
            let f4: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(this).wrapping_add(0x0c)) as usize);
            let r4 = f4(this);
            let obj = [0u32; 2];
            let base = obj.as_ptr() as u32;
            let _: u32 = callee_thiscall!(5, u32, base, r4);
            let _: u32 = callee_thiscall!(6, u32, a2.wrapping_add(0x84), base, 0, 1);
            let _: u32 = callee_thiscall!(7, u32, base);
            let o = rd32(this.wrapping_add(SUB_OFF));
            callee_cdecl!(14, u32, );
            return o;
        }
        // Replan half.
        let o = rd32(this.wrapping_add(SUB_OFF));
        let r8: u32 = callee_thiscall!(8, u32, this.wrapping_add(STATE_OFF));
        let mut changed = 0u8;
        if (r8 & 0xff) != 0 {
            wr32(this.wrapping_add(0x7c), rd32(this.wrapping_add(0x88)));
            wr32(this.wrapping_add(STATE_OFF), global::<u32>(GSTATE).read());
            wr8(this.wrapping_add(0x80), 1);
            let seed = rdf(a.wrapping_add(0x38));
            let obj = [0u32; 1];
            let base = obj.as_ptr() as u32;
            let _: u32 = callee_thiscall!(9, u32, base, w, seed.to_bits(), QUARTER.to_bits(), 0, 0, 0, 0, 0, 0);
            let idx1: u32 = callee_thiscall!(10, u32, base, apt);
            let idx2: u32 = callee_thiscall!(15, u32, base, this.wrapping_add(SELF_PT));
            let r11: u32 = callee_thiscall!(11, u32, this);
            if (r11 & 0xff) != 0 {
                changed = 1;
            } else if rd32(this.wrapping_add(0x94)) != idx1 || rd32(this.wrapping_add(0x98)) != idx2 {
                changed = 1;
            }
            wr32(this.wrapping_add(0x94), idx1);
            wr32(this.wrapping_add(0x98), idx2);
        }
        if rd8(this.wrapping_add(FLAG_OFF)) & 1 == 0 && changed == 0 {
            callee_cdecl!(14, u32, );
            return o;
        }
        if rd8(o.wrapping_add(0x0c)) & 1 == 0 {
            let f12: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(o).wrapping_add(0x14)) as usize);
            let r12 = f12(o, arg1, 1, 0);
            if (r12 & 0xff) == 0 {
                callee_cdecl!(14, u32, );
                return o;
            }
            wr32(o.wrapping_add(0x0c), rd32(o.wrapping_add(0x0c)) | 2);
        }
        let f13: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(this).wrapping_add(0x4c)) as usize);
        let r13 = f13(this, arg1);
        callee_cdecl!(14, u32, );
        r13
    }
});
