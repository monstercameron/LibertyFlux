// original: 0x00cb92d0 route_follower_update (proposed)

/// Advance a route-following move task one update towards its target point.
///
/// `this` is the task object (status flags at `+0xc4`, remembered target at
/// `+0x60`/`+0x64` with two more words at `+0x68`/`+0x6c`). `ped` is the
/// pedestrian (pointer to its entity at `+0x20`, validity flag bit 0 at
/// `+0x118`) and `target` holds the goal point (three floats then one word).
///
/// Behaviour: return 1 at once when the task's update flag (bit 0x400) is
/// clear or the pedestrian is invalid. Otherwise load a cached drift triple
/// from three globals, lazily initialised once (flag word bit 0) to
/// (0, 0, -4). When the goal is within 0.01 (squared planar distance to the
/// entity), snap the remembered target to the goal, set the arrived flag
/// (bit 0x80000) and return 1. Otherwise steer from the goal along the
/// normalised planar direction, scaled by 1/sqrt(d2) (zero only for a zero
/// distance, which the snap check always catches first, so the square-root
/// path is always taken here) and clamped so the blend factor never exceeds
/// 0.5. When the steered point is within 0.0625 (squared) of the remembered
/// target, return the arrived flag as 0/1. Otherwise call the guided-approach
/// helper (cdecl, six words: two overlapping frame structs, the pedestrian,
/// the selector 0x8e, two zero words) with the steered point plus the cached
/// drift, then store the steered point itself (plus a zero word) as the
/// remembered target: the helper's structs are never read back, only its
/// low answer byte matters, setting the arrived flag to (answer == 0),
/// which is also returned.
///
/// The float operation order is the original's, pinned through `black_box`
/// helpers; comparisons are ordered (NaN takes the far branch every time).
/// Original: 0x00cb92d0 (thiscall, two stack words, low byte of the result).
lf_checker_rt::export!(thiscall, rw_00cb92d0(this: u32, ped: u32, target: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0xc4;
        const NEED_UPDATE: u32 = 0x400;
        const ARRIVED: u32 = 0x80000;
        const OUT_X: u32 = 0x60;
        const OUT_Y: u32 = 0x64;
        const OUT_Z: u32 = 0x68;
        const OUT_W: u32 = 0x6c;
        const PED_ENT: u32 = 0x20;
        const PED_VALID: u32 = 0x118;
        const ENT_X: u32 = 0x30;
        const ENT_Y: u32 = 0x34;
        const APPROACH_CALLEE: u32 = 1;
        const APPROACH_SELECTOR: u32 = 0x8e;
        // File VAs of the cached drift triple, its init flag and constants.
        const G_X: u32 = 0x0171bf50;
        const G_Y: u32 = 0x0171bf54;
        const G_Z: u32 = 0x0171bf58;
        const G_INIT: u32 = 0x0171bf60;
        const C_SINK: u32 = 0x00fe8dc8; // -4.0
        const C_NEAR2: u32 = 0x00fe870c; // 0.01
        const C_ONE: u32 = 0x00fe88e8; // 1.0
        const C_HALF: u32 = 0x00fe8830; // 0.5
        const C_FAR2: u32 = 0x00fe8778; // 0.0625

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
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn gwr32(va: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(va).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
        }
        #[inline(always)]
        unsafe fn gwrf(va: u32, v: f32) {
            unsafe { gwr32(va, v.to_bits()) }
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        if rd32(this + FLAGS) & NEED_UPDATE == 0 {
            return 1;
        }
        if rd8(ped + PED_VALID) & 1 != 0 {
            return 1;
        }
        // Cached drift triple, lazily initialised.
        let init = g32(G_INIT);
        let (drift_x, drift_y, drift_z);
        if init & 1 == 0 {
            let sink = gf(C_SINK);
            gwr32(G_INIT, init | 1);
            gwrf(G_X, 0.0);
            gwrf(G_Y, 0.0);
            gwrf(G_Z, sink);
            drift_x = 0.0f32;
            drift_y = 0.0f32;
            drift_z = sink;
        } else {
            drift_z = gf(G_Z);
            drift_y = gf(G_Y);
            drift_x = gf(G_X);
        }
        let ent = rd32(ped + PED_ENT);
        let gx = rdf(target);
        let gy = rdf(target + 4);
        let dx = sub(gx, rdf(ent + ENT_X));
        let dy = sub(gy, rdf(ent + ENT_Y));
        let dist2 = add(mul(dy, dy), mul(dx, dx));
        if gf(C_NEAR2) > dist2 {
            wrf(this + OUT_X, gx);
            wrf(this + OUT_Y, gy);
            wrf(this + OUT_Z, rdf(target + 8));
            wr32(this + OUT_W, rd32(target + 0x0c));
            wr32(this + FLAGS, rd32(this + FLAGS) | ARRIVED);
            return 1;
        }
        let one = gf(C_ONE);
        // The original's test is ucomiss+lahf+test+jp: the jp reads the
        // test instruction's parity flag, which is set exactly when the
        // distance is not +0.0 (NaN included), so this is a plain == 0.0,
        // not a NaN check.
        let k = if dist2 == 0.0 {
            0.0f32
        } else {
            div(one, core::hint::black_box(dist2).sqrt())
        };
        let mut s = div(one, k);
        if !(gf(C_HALF) > s) {
            s = 0.5f32;
        }
        let ox = mul(k, dx);
        let oy = mul(k, dy);
        let oz = mul(k, 0.0f32);
        let tx = add(mul(s, ox), gx);
        let ty = add(mul(s, oy), gy);
        let tz = add(rdf(target + 8), mul(s, oz));
        let ex = sub(tx, rdf(this + OUT_X));
        let ey = sub(ty, rdf(this + OUT_Y));
        let err2 = add(mul(ey, ey), mul(ex, ex));
        if gf(C_FAR2) > err2 {
            return (rd32(this + FLAGS) >> 19) & 1;
        }
        let tz2 = add(tz, drift_z);
        let ax = add(tx, drift_x);
        let ay = add(ty, drift_y);
        // Out-structs as the original lays them out: the seven-word struct
        // starts 16 bytes before the three-word one, so its last three
        // words are the goal's words; the fourth word keeps the zero stack
        // fill. Neither struct is read back after the call.
        let goal = [ax.to_bits(), ay.to_bits(), tz2.to_bits()];
        let probe = [
            tx.to_bits(),
            ty.to_bits(),
            tz.to_bits(),
            0u32,
            ax.to_bits(),
            ay.to_bits(),
            tz2.to_bits(),
        ];
        let answer: u32 = lf_checker_rt::callee_cdecl!(
            APPROACH_CALLEE,
            u32,
            probe.as_ptr() as u32,
            goal.as_ptr() as u32,
            ped,
            APPROACH_SELECTOR,
            0,
            0
        );
        let mut bit = if (answer as u8) == 0 { 1u32 } else { 0u32 };
        bit <<= 19;
        let flags = rd32(this + FLAGS);
        bit ^= flags;
        bit &= ARRIVED;
        wr32(this + FLAGS, flags ^ bit);
        // The first word is read before the callee's six words are popped,
        // so it is the steered x, not anything the helper wrote.
        wrf(this + OUT_X, tx);
        wrf(this + OUT_Y, ty);
        wrf(this + OUT_Z, tz);
        wr32(this + OUT_W, 0);
        (rd32(this + FLAGS) >> 19) & 1
    }
});
