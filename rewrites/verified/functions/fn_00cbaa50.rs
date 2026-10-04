// original: 0x00cbaa50 slide_steer_toward_point (proposed)

/// Steer a slide task toward a target point when it lies outside a radius.
///
/// Arguments (cdecl, five stack words): `target` points to an (x, y) pair of
/// floats; `task` points to the task object; `gain`, `cap_in` and `radius`
/// arrive as raw float bits.
///
/// The task's anchor object at `ANCHOR` (+0x20) holds the current (x, y) at
/// +0x30/+0x34. The offset `dx = target.x - anchor.x`,
/// `dy = target.y - anchor.y` and `d2 = dy*dy + dx*dx` are formed in that
/// operand order. If `d2` is below `radius*radius` (an unordered result also
/// takes this path), the three output slots `OUT0` (+0xa90), `OUT1` (+0xa94)
/// and `OUT2` (+0xa98) are zeroed and 0 is returned.
///
/// Otherwise: `dist = sqrt(d2)`. A limit is clamped between the image
/// constants 2.0 and 0.1 with `cap_in` (`limit = min(max(cap_in, 0.1), 2.0)`
/// in ordered-compare form, NaN inputs propagate). A virtual helper (slot
/// 0x60 of the object at `HELPER` +0xa80, thiscall, x87 float result) is
/// called; `root = sqrt(helper * 0.8 * dist)` caps the limit from above
/// (ordered compare). The limit is then scaled by `gain` against the 0.9
/// constant: when `limit * 0.9 * gain` is above `dist` (ordered), the limit
/// is replaced by `dist * 0.9 / gain`, and finally divided by `dist`.
/// `OUT0 = limit * dx` and `OUT1 = limit * dy` are stored (`OUT2` is left
/// untouched on this path) and 1 is returned.
///
/// All float arithmetic uses the original's operand order; the two
/// below-or-unordered branches are written to match `jb`/`jbe` exactly,
/// including NaN inputs. Returns 0 or 1 in `al` (upper `eax` is leftover).
///
/// Original: 0x00cbaa50 (cdecl, five stack words).
lf_checker_rt::export!(cdecl, rw_00cbaa50(target: u32, task: u32, gain_b: u32, cap_b: u32, radius_b: u32) -> u32 {
    unsafe {
        const ANCHOR: u32 = 0x20;
        const ANCHOR_X: u32 = 0x30;
        const ANCHOR_Y: u32 = 0x34;
        const HELPER: u32 = 0xa80;
        const HELPER_SLOT: u32 = 0x60;
        const OUT0: u32 = 0xa90;
        const OUT1: u32 = 0xa94;
        const OUT2: u32 = 0xa98;
        const LO_ADDR: u32 = 0x01050e70;
        const HI_ADDR: u32 = 0x01050e6c;
        const K0_ADDR: u32 = 0x00fe8898;
        const K2_ADDR: u32 = 0x00fe88bc;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        /// `comiss a, b` + `jb`: taken when below or unordered.
        #[inline(always)]
        fn below(a: f32, b: f32) -> bool {
            a.is_nan() || b.is_nan() || a < b
        }
        /// `comiss a, b` + `jbe`: taken when below, equal or unordered.
        #[inline(always)]
        fn below_eq(a: f32, b: f32) -> bool {
            a.is_nan() || b.is_nan() || a <= b
        }

        let gain = f32::from_bits(gain_b);
        let cap_in = f32::from_bits(cap_b);
        let radius = f32::from_bits(radius_b);
        let anchor = rd32(task + ANCHOR);
        let dx = sub(
            f32::from_bits(rd32(target)),
            f32::from_bits(rd32(anchor + ANCHOR_X)),
        );
        let dy = sub(
            f32::from_bits(rd32(target + 4)),
            f32::from_bits(rd32(anchor + ANCHOR_Y)),
        );
        let d2 = add(mul(dy, dy), mul(dx, dx));
        let r2 = mul(radius, radius);
        if below(d2, r2) {
            wr32(task + OUT2, 0);
            wr32(task + OUT1, 0);
            wr32(task + OUT0, 0);
            return 0;
        }
        let dist = core::hint::black_box(d2).sqrt();
        let lo = f32::from_bits(rd32(lf_checker_rt::relocated(LO_ADDR)));
        let mut limit = lo;
        if !(cap_in > lo) {
            limit = cap_in;
        }
        let hi = f32::from_bits(rd32(lf_checker_rt::relocated(HI_ADDR)));
        let mut slot = hi;
        if !(hi > limit) {
            slot = limit;
        }
        let obj = rd32(task + HELPER);
        let vt = rd32(obj);
        let target_fn: extern "thiscall" fn(u32) -> f32 =
            unsafe { core::mem::transmute(rd32(vt + HELPER_SLOT) as usize) };
        let helper = target_fn(obj);
        let k0 = f32::from_bits(rd32(lf_checker_rt::relocated(K0_ADDR)));
        let root = mul(mul(helper, k0), dist).sqrt();
        if !below_eq(slot, root) {
            slot = root;
        }
        let k2 = f32::from_bits(rd32(lf_checker_rt::relocated(K2_ADDR)));
        let t = mul(mul(slot, k2), gain);
        if t > dist {
            slot = div(mul(dist, k2), gain);
        }
        slot = div(slot, dist);
        wr32(task + OUT0, mul(slot, dx).to_bits());
        wr32(task + OUT1, mul(slot, dy).to_bits());
        1
    }
});
