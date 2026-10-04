// original: 0x00a64f40 accept_aim_overlap
use lf_checker_rt::{callee_cdecl, export, global};

/// Decide whether two aim rays overlap closely enough to continue.
///
/// Measures the 2D gap between the probe and reference points: an
/// overlapping pair (within the snap radius) accepts immediately. Wider
/// pairs run three chained geometric screens over the entity's basis rows
/// — each screen normalizes one row's squared length (a zero row scores
/// zero, anything else scores its reciprocal root, NaN included) and folds
/// the scaled projections together. A lazily initialized global scale
/// (loaded from a constant double on first use, then cached) weights the
/// final blend, whose sign and magnitude against the cached scale pick the
/// outcome. Returns 1 on accept, 0 on reject.
export!(cdecl, rw_00a64f40(target: u32, _aux: u32, probe: u32, refer: u32) -> u32 {
    const SNAP_SQ: f32 = f32::from_bits(0x3C23D70A);
    const UNIT: f32 = 1.0;
    unsafe {
        let dx = f32::from_bits((refer as *const u32).read())
            - f32::from_bits((probe as *const u32).read());
        let dy = f32::from_bits(((refer + 4) as *const u32).read())
            - f32::from_bits(((probe + 4) as *const u32).read());
        let gap2 = dx * dx + dy * dy;
        if SNAP_SQ > gap2 {
            return 1;
        }
        let basis = ((target + 0x20) as *const u32).read();
        let row0x = f32::from_bits(((basis + 0x14) as *const u32).read());
        let row0y = f32::from_bits(((basis + 0x10) as *const u32).read());
        let rlen = row0y * row0y + row0x * row0x;
        let n0 = recip_root(rlen);
        let sx = row0x * n0;
        let sy = row0y * n0;
        let n0z = n0 * 0.0;
        let nq = recip_root(gap2);
        let px = dx * nq;
        let py = dy * nq;
        let mix_a = sy * px;
        let mix_z = nq * 0.0;
        let mix_b = sx * py;
        let mix_c = n0z * mix_z;
        let mut screen = mix_a + mix_b;
        screen += mix_c;
        if 0.0f32 > screen {
            return 1;
        }
        let row1x = f32::from_bits((basis as *const u32).read());
        let row1y = f32::from_bits(((basis + 4) as *const u32).read());
        let tlen = row1x * row1x + row1y * row1y;
        let n1 = recip_root(tlen);
        let row2z = f32::from_bits(((basis + 0x34) as *const u32).read());
        let row2x = f32::from_bits(((basis + 0x30) as *const u32).read());
        let row2y = f32::from_bits(((basis + 0x38) as *const u32).read());
        let qx = row1x * n1;
        let qy = row1y * n1;
        let mut acc = row2z * qy;
        let n1z = n1 * 0.0;
        let ax = f32::from_bits((probe as *const u32).read());
        acc += row2x * qx;
        let ay = f32::from_bits(((probe + 4) as *const u32).read());
        let mut side = ay * qy;
        side += ax * qx;
        acc += row2y * n1z;
        let az = f32::from_bits(((probe + 8) as *const u32).read());
        side += az * n1z;
        acc = -acc;
        acc += side;
        let scale = lazy_scale();
        let mut blend = qx * px + qy * py;
        blend += n1z * mix_z;
        let mag = blend.abs();
        if 0.0f32 > acc {
            if mag > scale {
                return 1;
            }
        } else if acc > 0.0f32 {
            if -scale > mag {
                return 1;
            }
        }
        0
    }
});

/// Reciprocal root with the original's zero/NaN shape: an exactly zero
/// input scores zero, anything else (including NaN) scores one over its
/// square root.
#[inline(always)]
fn recip_root(x: f32) -> f32 {
    if x != 0.0 {
        1.0f32 / x.sqrt()
    } else {
        0.0
    }
}

/// Lazily initialized global scale: the first call converts the constant
/// double, flags the cache valid, notifies the sink, and stores the float;
/// later calls reuse the cached value.
#[inline(always)]
fn lazy_scale() -> f32 {
    const FLAG: u32 = 0x012F8374;
    const CACHE: u32 = 0x012F8370;
    const SOURCE: u32 = 0x00E9B9E0;
    unsafe {
        let flag = global::<u32>(FLAG);
        if flag.read() & 1 == 0 {
            let v = global::<f64>(SOURCE).read() as f32;
            flag.write(flag.read() | 1);
            callee_cdecl!(1, u32,);
            global::<u32>(CACHE).write(v.to_bits());
            v
        } else {
            f32::from_bits(global::<u32>(CACHE).read())
        }
    }
}
