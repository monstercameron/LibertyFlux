// original: 0x00ae1930 mouse_clip_update (proposed)

/// Refresh the two clamped mouse-clip coordinates from the polled device state.
///
/// Takes no arguments (cdecl, nothing read from the incoming stack). It asks
/// the device-table callee for the device object (one pushed zero word), then
/// asks the mode callee for the active scale selector: a zero answer selects
/// the integer at `SCALE_A`, any other value the integer at `SCALE_B`. That
/// integer converted to float is the working scale `w`.
///
/// When the device's enable flag (byte at `+0x328c`) is clear, the two output
/// globals are copied straight from the fallback pair and the function
/// returns. Otherwise two axis factors are polled from the sampler callee
/// (x87 float results) for the slots at `+0x2b38` and `+0x2b48`:
///
/// - axis 0: `x = f0 != 0.0 ? (f0 * w) * k + base0 : base0`, remembering
///   whether the scaled branch ran (`k` is the float multiplier global,
///   `base0` the first base integer as float);
/// - axis 1: `y = f1 != 0.0 ? (f1 * w) * k + base1 : base1`, where `base1` is
///   the second base global as float; when `f1 == 0.0` and the axis-0 branch
///   did not run, nothing is stored and the function returns.
///
/// The zero tests treat NaN as nonzero (the original's compare/flag/parity
/// sequence takes the scaled branch for unordered results), which is exactly
/// what `!= 0.0` does. Both results then gain the shared float bias, are
/// truncated toward zero with x86 convert semantics (out of range or NaN
/// yields `0x80000000`), have the matching integer base subtracted with
/// wraparound, and are stored to the two output globals before the notifier
/// callee runs with the truncated pair (x first).
///
/// Original: 0x00ae1930 (cdecl, no stack arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00ae1930() -> u32 {
    unsafe {
        const OBJ_ENABLE: u32 = 0x328c;
        const OBJ_SLOT_A: u32 = 0x2b38;
        const OBJ_SLOT_B: u32 = 0x2b48;
        const G_SCALE_A: u32 = 0x0105c884;
        const G_SCALE_B: u32 = 0x0105c888;
        const G_MULT: u32 = 0x0117359c;
        const G_BASE0: u32 = 0x018b7a80;
        const G_BASE1: u32 = 0x018b7a8c;
        const G_BIAS: u32 = 0x00fe8830;
        const G_FALLBACK0: u32 = 0x018b7a68;
        const G_FALLBACK1: u32 = 0x018b7a6c;
        const G_OUT0: u32 = 0x01593b84;
        const G_OUT1: u32 = 0x01593b88;
        const CALLEE_DEVICE: u32 = 1;
        const CALLEE_MODE: u32 = 2;
        const CALLEE_SAMPLE: u32 = 3;
        const CALLEE_NOTIFY: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// x86 `cvttss2si`: truncate toward zero; NaN, infinities and
        /// out-of-range magnitudes all yield the indefinite `0x80000000`.
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x80000000u32 as i32
            } else {
                x as i32
            }
        }

        let device: u32 = lf_checker_rt::callee_cdecl!(CALLEE_DEVICE, u32, 0u32);
        let mode: u32 = lf_checker_rt::callee_cdecl!(CALLEE_MODE, u32,);
        let scale_bits = if (mode & 0xff) != 0 {
            rd32(lf_checker_rt::relocated(G_SCALE_B))
        } else {
            rd32(lf_checker_rt::relocated(G_SCALE_A))
        };
        let w: f32 = core::hint::black_box(scale_bits as i32) as f32;
        let enabled = ((device.wrapping_add(OBJ_ENABLE)) as *const u8).read();
        if enabled == 0 {
            let f0 = rd32(lf_checker_rt::relocated(G_FALLBACK0));
            wr32(lf_checker_rt::relocated(G_OUT0), f0);
            let f1 = rd32(lf_checker_rt::relocated(G_FALLBACK1));
            wr32(lf_checker_rt::relocated(G_OUT1), f1);
            return f1;
        }
        let k = f32::from_bits(rd32(lf_checker_rt::relocated(G_MULT)));
        let base0 = rd32(lf_checker_rt::relocated(G_BASE0));
        let base0f: f32 = core::hint::black_box(base0 as i32) as f32;
        let base1 = rd32(lf_checker_rt::relocated(G_BASE1));
        let base1f: f32 = core::hint::black_box(base1 as i32) as f32;
        let f0: f32 = lf_checker_rt::callee_cdecl!(
            CALLEE_SAMPLE,
            f32,
            device.wrapping_add(OBJ_SLOT_A)
        );
        let f1: f32 = lf_checker_rt::callee_cdecl!(
            CALLEE_SAMPLE,
            f32,
            device.wrapping_add(OBJ_SLOT_B)
        );
        let mut scaled = false;
        let mut x = base0f;
        if f0 != 0.0 {
            x = add(mul(mul(f0, w), k), base0f);
            scaled = true;
        }
        let mut y: f32;
        if f1 != 0.0 {
            y = add(mul(mul(f1, w), k), base1f);
        } else if scaled {
            y = base1f;
        } else {
            return mode;
        }
        let bias = f32::from_bits(rd32(lf_checker_rt::relocated(G_BIAS)));
        x = add(x, bias);
        y = add(y, bias);
        let edx = cvtt(x);
        let ecx = cvtt(y);
        wr32(
            lf_checker_rt::relocated(G_OUT0),
            (edx as u32).wrapping_sub(base0),
        );
        wr32(
            lf_checker_rt::relocated(G_OUT1),
            (ecx as u32).wrapping_sub(base1),
        );
        let r: u32 = lf_checker_rt::callee_cdecl!(CALLEE_NOTIFY, u32, edx as u32, ecx as u32);
        r
    }
});
