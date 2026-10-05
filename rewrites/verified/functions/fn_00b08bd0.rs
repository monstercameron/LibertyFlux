// original: 0x00B08BD0 net_signal_levels_update (proposed)
//
// Refresh three smoothed signal levels on the network object from three fresh
// integer readings, then clamp them into range.
//
// `this` points to the object. Levels live at `+0x1c8` (x), `+0x1cc` (y) and
// `+0x1d0` (z); a gain term is read from `+0x60`. The update first asks callee
// 1 (cdecl, two words) for a reader context. A null context skips the refresh
// and only decays the stored levels by `DECAY` (0.9).
//
// Otherwise three readings are taken: each is a no-argument thiscall on the
// context followed by a one-word cdecl conversion, then negated (call pairs
// 2/3, 4/5, 6/7 giving `va`, `vb`, `vc`). Each is converted to float and
// scaled by `K_W0`, then by the range constant of its lane (`K_PI` for x and
// y, `K_TOP` for z). A non-negative gain `g = max([this+0x60] + z_old, 0)`
// (a negative gain is replaced by zero; NaN is kept) is scaled by `K_W1`.
// Unless the context flag byte at `+0x328d` is set, the current levels are
// subtracted first (making the update relative); everything is then scaled by
// `K_W2`, the x and y lanes are multiplied by the gain, the old levels are
// added back and the three results are stored.
//
// After the store, if every reading is exactly zero and the context flag is
// clear, the levels are decayed by `DECAY` first (the same decay the
// null-context path applies); any nonzero reading or a set flag skips the
// decay and goes straight to the clamp. Each check is an ordered float
// compare of the converted reading against zero whose equal/not-equal result
// is tested through lahf (`(an instruction of the original); jp`), so it fires on any nonzero
// value, positive or negative. The clamp keeps x
// and y inside [-PI, PI] (a NaN becomes +PI) and z inside [0, K_TOP].
// Comparisons use the original's ordered-compare semantics, so NaN follows
// the original's path in every branch. All floating-point arithmetic runs in
// the original's operand order through order-pinned helpers.
//
// Original: 0x00B08BD0 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00B08BD0(this: u32) -> u32 {
    unsafe {
        const CAL_CTX: u32 = 1;
        const CAL_M_A: u32 = 2;
        const CAL_C_A: u32 = 3;
        const CAL_M_B: u32 = 4;
        const CAL_C_B: u32 = 5;
        const CAL_M_C: u32 = 6;
        const CAL_C_C: u32 = 7;
        const GAIN: u32 = 0x60;
        const LVL_X: u32 = 0x1c8;
        const LVL_Y: u32 = 0x1cc;
        const LVL_Z: u32 = 0x1d0;
        const CTX_FLAG: u32 = 0x328d;
        const K_W0: f32 = f32::from_bits(0x3c010204);
        const K_W1: f32 = f32::from_bits(0x3c4ccccd); // 0.0125
        const K_W2: f32 = f32::from_bits(0x3dcccccd); // 0.1
        const K_PI: f32 = f32::from_bits(0x4048f5c3); // 3.14
        const K_NEG_PI: f32 = f32::from_bits(0xc048f5c3);
        const K_TOP: f32 = f32::from_bits(0x3f4ccccd); // 0.8
        const DECAY: f32 = f32::from_bits(0x3f666666); // 0.9

        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        unsafe fn decay(this: u32) {
            unsafe {
                wrf(this + LVL_Y, mul(rdf(this + LVL_Y), DECAY));
                wrf(this + LVL_X, mul(rdf(this + LVL_X), DECAY));
                wrf(this + LVL_Z, mul(rdf(this + LVL_Z), DECAY));
            }
        }
        unsafe fn clamp(this: u32) {
            unsafe {
                let mut x = rdf(this + LVL_X);
                if x < K_NEG_PI {
                    x = K_NEG_PI;
                }
                if !(K_PI > x) {
                    x = K_PI;
                }
                wrf(this + LVL_X, x);
                let mut y = rdf(this + LVL_Y);
                if y < K_NEG_PI {
                    y = K_NEG_PI;
                }
                if !(K_PI > y) {
                    y = K_PI;
                }
                wrf(this + LVL_Y, y);
                let mut z = rdf(this + LVL_Z);
                if 0.0 > z {
                    z = 0.0;
                } else if !(K_TOP > z) {
                    z = K_TOP;
                }
                wrf(this + LVL_Z, z);
            }
        }

        let ctx: u32 = lf_checker_rt::callee_cdecl!(CAL_CTX, u32, 0, 1);
        if ctx == 0 {
            decay(this);
            clamp(this);
            return 0;
        }
        let ma: u32 = lf_checker_rt::callee_thiscall!(CAL_M_A, u32, ctx);
        let va = (lf_checker_rt::callee_cdecl!(CAL_C_A, u32, ma) as i32).wrapping_neg();
        let mb: u32 = lf_checker_rt::callee_thiscall!(CAL_M_B, u32, ctx);
        let vb = (lf_checker_rt::callee_cdecl!(CAL_C_B, u32, mb) as i32).wrapping_neg();
        let mc: u32 = lf_checker_rt::callee_thiscall!(CAL_M_C, u32, ctx);
        let vc = (lf_checker_rt::callee_cdecl!(CAL_C_C, u32, mc) as i32).wrapping_neg();

        let mut fx = mul(core::hint::black_box(va as f32), K_W0);
        let mut fy = mul(core::hint::black_box(vb as f32), K_W0);
        let mut fz = mul(core::hint::black_box(vc as f32), K_W0);
        let z_old = rdf(this + LVL_Z);
        let mut gain = add(rdf(this + GAIN), z_old);
        if gain < 0.0 {
            gain = 0.0;
        }
        let flag = (ctx + CTX_FLAG) as *const u8;
        let flag_set = flag.read() != 0;
        gain = mul(gain, K_W1);
        fx = mul(fx, K_PI);
        fy = mul(fy, K_PI);
        fz = mul(fz, K_TOP);
        if !flag_set {
            fx = sub(fx, rdf(this + LVL_Y));
            fy = sub(fy, rdf(this + LVL_X));
            fz = sub(fz, z_old);
        }
        fx = mul(fx, K_W2);
        fy = mul(fy, K_W2);
        fz = mul(fz, K_W2);
        fz = add(fz, rdf(this + LVL_Z));
        fx = mul(fx, gain);
        fy = mul(fy, gain);
        fx = add(fx, rdf(this + LVL_Y));
        fy = add(fy, rdf(this + LVL_X));
        wrf(this + LVL_Z, fz);
        wrf(this + LVL_Y, fx);
        wrf(this + LVL_X, fy);
        if va == 0 && vb == 0 && vc == 0 && !flag_set {
            decay(this);
        }
        clamp(this);
        0
    }
});
