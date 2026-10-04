// original: 0x00cb3a10 CTaskComplexMoveCrowdAroundLocation::vf18

/// Pick the crowd-movement subtask for this tick from distance thresholds.
///
/// `this` is the complex task, `ped` the pedestrian. Reads two anchor points
/// from the task (A at `+0x20`, B at `+0x30`, C at `+0x40`), a radius at
/// `+0x50`, state bytes at `+0x59`/`+0x5a`, and the pedestrian's position
/// (matrix at `[ped+0x20]`, position at `+0x30`/`+0x34`/`+0x38`).
///
/// Flow: ask the movement check callee (id 1) whether to proceed; on false,
/// clear the state byte and request subtask `0x3ae`. Otherwise, unless the
/// state byte is already clear, measure |C-A| squared and request `0x11a`
/// when it is below 1. Then clear the state byte, run the two refresh
/// callees (ids 3, 4), measure the 2D distance `e2` between B and the
/// pedestrian, and, when the mode byte is set, the 3D distance `D2` between
/// A and the pedestrian: request `0x11a` when the radius exceeds `sqrt(D2)`
/// or when `0.25` exceeds `e2`; request `0x3ae` when `9.0` does not exceed
/// `e2` or the mode byte is set; else request `0x384`.
///
/// All float comparisons use the original's ordered-compare semantics: the
/// taken-on-unordered edges (`jbe`) are written as `!(a > b)` so NaN takes
/// the same path as in the original. Operation order matches the original.
/// The check callee answers in `al` only (upper bits are residue); the
/// value is masked before use.
///
/// Original: 0x00cb3a10 (thiscall, one stack word; returns the subtask id).
lf_checker_rt::export!(thiscall, rw_00cb3a10(this: u32, ped: u32) -> u32 {
    unsafe {
        const ANCHOR_A: u32 = 0x20;
        const ANCHOR_B: u32 = 0x30;
        const ANCHOR_C: u32 = 0x40;
        const RADIUS: u32 = 0x50;
        const STATE: u32 = 0x59;
        const MODE: u32 = 0x5a;
        const PED_MATRIX: u32 = 0x20;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const POS_Z: u32 = 0x38;
        const FAR_TASK: u32 = 0x3ae;
        const NEAR_TASK: u32 = 0x11a;
        const MID_TASK: u32 = 0x384;
        const CHECK_CALLEE: u32 = 1;
        const REQUEST_CALLEE: u32 = 2;
        const REFRESH_A_CALLEE: u32 = 3;
        const REFRESH_B_CALLEE: u32 = 4;

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
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let ok = (lf_checker_rt::callee_thiscall!(CHECK_CALLEE, u32, this, ped) & 0xff) as u8;
        if ok == 0 {
            ((this + STATE) as *mut u8).write(ok);
            return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, FAR_TASK, ped);
        }
        if ((this + STATE) as *const u8).read() != 0 {
            let dx = sub(rdf(this + ANCHOR_C), rdf(this + ANCHOR_A));
            let dy = sub(rdf(this + ANCHOR_C + 4), rdf(this + ANCHOR_A + 4));
            let dz = sub(rdf(this + ANCHOR_C + 8), rdf(this + ANCHOR_A + 8));
            let dist2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
            if 1.0f32 > dist2 {
                return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, NEAR_TASK, ped);
            }
        }
        ((this + STATE) as *mut u8).write(0);
        let _: u32 = lf_checker_rt::callee_thiscall!(REFRESH_A_CALLEE, u32, this, ped);
        let _: u32 = lf_checker_rt::callee_thiscall!(REFRESH_B_CALLEE, u32, this, ped);
        let mtx = rd32(ped + PED_MATRIX);
        let ex = sub(rdf(this + ANCHOR_B), rdf(mtx + POS_X));
        let ey = sub(rdf(this + ANCHOR_B + 4), rdf(mtx + POS_Y));
        let e2 = add(mul(ex, ex), mul(ey, ey));
        let mode = ((this + MODE) as *const u8).read();
        if mode != 0 {
            let fx = sub(rdf(this + ANCHOR_A), rdf(mtx + POS_X));
            let fy = sub(rdf(this + ANCHOR_A + 4), rdf(mtx + POS_Y));
            let fz = sub(rdf(this + ANCHOR_A + 8), rdf(mtx + POS_Z));
            let d2 = add(add(mul(fy, fy), mul(fx, fx)), mul(fz, fz));
            let radius = rdf(this + RADIUS);
            if radius > d2.sqrt() {
                return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, NEAR_TASK, ped);
            }
        }
        if 0.25f32 > e2 {
            return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, NEAR_TASK, ped);
        }
        if !(9.0f32 > e2) {
            return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, FAR_TASK, ped);
        }
        if mode != 0 {
            return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, FAR_TASK, ped);
        }
        lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, MID_TASK, ped)
    }
});
