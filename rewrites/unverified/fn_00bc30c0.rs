// original: 0x00BC30C0 CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>::vf19

/// Seek-step update for a move-to-entity task: refresh the task's timestamps,
/// ask the seek-position calculator where to go, and pick the next sub-step
/// from the distance to the target.
///
/// `this` is the task object, `entity` the target. The task keeps a ped
/// pointer at `+0x20`, the last computed seek position (x, y, z, heading) at
/// `+0x30`, a mode word at `+0x40` (`-1` means the second timestamp pair is
/// unused), a previous value at `+0x44`, the arrival radius at `+0x48`, a far
/// threshold at `+0x50`, timestamp pairs at `+0x5c`/`+0x68`, and flags at
/// `+0x80`. Both timestamps are the game timer global. The `+0x68` store is
/// the `f_68_int32` field the structure survey already knew.
///
/// Behaviour: when there is no ped, or the target is already fully handled
/// (flag bit 2 at entity `+0x26c` with a non-null handle at `+0xb30`), the
/// calculator is skipped and step `0x516` runs. Otherwise the calculator
/// callee fills a 16-byte position (read through its return pointer) and the
/// squared planar distance from the target's matrix position (entity `+0x20`,
/// x/y at `+0x30`/`+0x34`) is compared against the squared arrival radius:
/// inside it, flag bit 1 is set, the follow flag is stored through its
/// callee, and step `0x11e` runs; outside it, step `0x387` runs when the
/// distance does not exceed the squared far threshold, else step `0x3ae`.
///
/// The threshold comparison first tests the threshold against zero with an
/// unordered-or-equal check whose taken branch is dead: `lahf` after `ucomiss`
/// leaves `(ah & 0x44)` at `0` (less/greater) or `0x44` (equal/unordered),
/// both of even parity, so the parity jump never fires for any threshold,
/// NaN included. Execution always reaches the squared comparison. The
/// function's return value is whatever the final step callee returns.
///
/// Original: thiscall, one stack word. Float order is the original's
/// (dy^2 + dx^2, then radius^2 against it); NaN distances take the
/// not-greater side of every `comiss`/`jbe` pair, as unordered does.
lf_checker_rt::export!(thiscall, rw_00bc30c0(this: u32, entity: u32) -> u32 {
    unsafe {
        const TIMER_GLOBAL: u32 = 0x0117_35B4;
        const STEP_CALLEE: u32 = 1;
        const CALC_CALLEE: u32 = 2;
        const FLAG_CALLEE: u32 = 3;
        const STEP_SEEK: u32 = 0x516;
        const STEP_ARRIVED: u32 = 0x11E;
        const STEP_MID: u32 = 0x387;
        const STEP_FAR: u32 = 0x3AE;
        const T_PED: u32 = 0x20;
        const T_POS_X: u32 = 0x30;
        const T_POS_Y: u32 = 0x34;
        const T_POS_Z: u32 = 0x38;
        const T_POS_H: u32 = 0x3C;
        const T_MODE: u32 = 0x40;
        const T_PREV: u32 = 0x44;
        const T_NEAR_RADIUS: u32 = 0x48;
        const T_FAR_THRESH: u32 = 0x50;
        const T_STAMP_A: u32 = 0x5C;
        const T_STAMP_AUX: u32 = 0x60;
        const T_FLAG_A: u32 = 0x64;
        const T_STAMP_B: u32 = 0x68;
        const T_MODE_COPY: u32 = 0x6C;
        const T_FLAG_B: u32 = 0x70;
        const T_FLAGS: u32 = 0x80;
        const ENT_HANDLED_FLAG: u32 = 0x26C;
        const ENT_HANDLE: u32 = 0xB30;
        const ENT_MATRIX: u32 = 0x20;
        const ARRIVED_BIT: u8 = 2;
        const HANDLED_BIT: u8 = 4;

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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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
        /// `comiss a, b; jbe taken`: taken unless a is strictly greater,
        /// unordered included (an unordered compare sets CF, so it jumps).
        #[inline(always)]
        fn below_or_equal(a: f32, b: f32) -> bool {
            !(core::hint::black_box(a) > core::hint::black_box(b))
        }

        let now = (lf_checker_rt::global::<u32>(TIMER_GLOBAL) as *const u32).read();
        wr32(this + T_STAMP_A, now);
        wr32(this + T_STAMP_AUX, rd32(this + T_PREV));
        wr8(this + T_FLAG_A, 1);
        let mode = rd32(this + T_MODE);
        if mode != 0xFFFF_FFFF {
            wr32(this + T_STAMP_B, now);
            wr32(this + T_MODE_COPY, mode);
            wr8(this + T_FLAG_B, 1);
        }
        if rd32(this + T_PED) == 0 {
            return lf_checker_rt::callee_thiscall!(STEP_CALLEE, u32, this, STEP_SEEK, entity);
        }
        if rd8(entity + ENT_HANDLED_FLAG) & HANDLED_BIT != 0 && rd32(entity + ENT_HANDLE) != 0 {
            return lf_checker_rt::callee_thiscall!(STEP_CALLEE, u32, this, STEP_SEEK, entity);
        }
        let mut scratch = [0u32; 4];
        let pos = lf_checker_rt::callee_thiscall!(
            CALC_CALLEE, u32, this, scratch.as_mut_ptr() as u32, entity
        );
        let px = rd32(pos);
        let py = rdf(pos + 4);
        let pz = rdf(pos + 8);
        wrf(this + T_POS_Z, pz);
        wr32(this + T_POS_X, px);
        wrf(this + T_POS_Y, py);
        wr32(this + T_POS_H, rd32(pos + 12));
        let mat = rd32(entity + ENT_MATRIX);
        let dx = sub(rdf(mat + 0x30), f32::from_bits(px));
        let dy = sub(rdf(mat + 0x34), py);
        let dx2 = mul(dx, dx);
        let dy2 = mul(dy, dy);
        let dist2 = add(dy2, dx2);
        let rad = rdf(this + T_NEAR_RADIUS);
        let rad2 = mul(rad, rad);
        if below_or_equal(rad2, dist2) {
            // Far from the target. The zero-test on the threshold jumps
            // nowhere (see the doc comment), so the squared threshold
            // comparison always runs.
            let t = rdf(this + T_FAR_THRESH);
            let t2 = mul(t, t);
            if below_or_equal(dist2, t2) {
                return lf_checker_rt::callee_thiscall!(STEP_CALLEE, u32, this, STEP_MID, entity);
            }
            return lf_checker_rt::callee_thiscall!(STEP_CALLEE, u32, this, STEP_FAR, entity);
        }
        wr8(this + T_FLAGS, rd8(this + T_FLAGS) | ARRIVED_BIT);
        lf_checker_rt::callee_thiscall!(FLAG_CALLEE, u32, entity, 1);
        lf_checker_rt::callee_thiscall!(STEP_CALLEE, u32, this, STEP_ARRIVED, entity)
    }
});
