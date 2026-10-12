// original: 0x00e42da0 multi_pass_quad_draw_b
/// Multi-pass quad draw, variant B.
///
/// When the enable flag is clear it returns immediately. Otherwise it clamps
/// an adjustment value into an alpha byte, fetches a color, scales one config
/// pair in place, and draws on every other row of its single list while
/// accumulating a running offset.
export!(thiscall, rw_00e42da0(this: u32) -> () {
    unsafe {
        if u8_at(this, F_ENABLE) == 0 {
            return;
        }
        // Shared out-buffer, exactly like the original's single stack slot:
        // both config calls and the color call target it, so the second word
        // still holds the second config call's answer when the draw reads it.
        let mut out2 = [0u32; 2];
        let p = callee_cdecl!(CFG, u32, out2.as_mut_ptr() as u32, CFG_C);
        let mut tmp = cvtt(f32::from_bits((p as *const u32).read()));
        if global::<u8>(GATE_FLAG).read() != 0 {
            tmp = callee_thiscall!(ADJ, u32, relocated(GATE_ADDR)) as i32;
        }
        let esi = (tmp & 0xff) as u32;
        let p = callee_cdecl!(CFG, u32, out2.as_mut_ptr() as u32, CFG_C);
        let himax = f32::from_bits((p as *const u32).read());
        let s52clamp = clamp_lo0(esi as f32, himax);
        let p = callee_cdecl!(COL, u32, out2.as_mut_ptr() as u32, COL_ID2);
        let color = (p as *const u32).read();
        let s40 = (((cvtt(s52clamp) & 0xff) as u32) << 24) | (color & 0x00ff_ffff);

        let mut outb = [0u32; 2];
        callee_cdecl!(CFG, u32, outb.as_mut_ptr() as u32, CFG_D);
        callee_cdecl!(F260, u32, 2, 0, outb.as_mut_ptr() as u32, 0);

        let e = ((u8_at(this, F_COUNT1) as i32).wrapping_sub(2)) as f32;
        let mut x2 = f32_at(this, F_B);
        x2 += f32_at(this, F_X0);
        let mut x3 = f32_at(this, F_B);
        x3 += f32::from_bits(outb[1]);
        let s32 = f32_at(this, F_A);
        let mut x0 = f32_at(this, F_YBASE);
        x0 += f32_at(this, F_Y0);
        let mut x1 = e * f32_at(this, F_YSPAN);
        if u8_at(this, F_MODE) != 0 {
            x0 += f32_at(this, F_YSTEP);
        }
        x1 += x0;
        let mut s52 = x2;
        let mut s48 = x3;
        let s44 = x1;
        let stash = out2[1];

        let g1a = global::<u32>(G1A).read();
        let g1b = global::<u32>(G1B).read();
        let g2a = global::<u32>(G2A).read();
        let g2b = global::<u32>(G2B).read();
        let count = u8_at(this, F_LOOP);
        let step = f32_at(this, F_XSTEP);
        let mut edx: u32 = 0;
        let mut cl = true;
        while edx < count as u32 {
            if cl {
                let b = coin_pick(g1a, g1b);
                let d = coin_pick(g2a, g2b);
                let s = coin_pick(g1a, g1b);
                let c = coin_pick(g2a, g2b);
                let p1 = [s40, edx, s32.to_bits(), stash];
                let p2 = [
                    ((c as i32) as f32 * s32).to_bits(),
                    ((b as i32) as f32 * s48).to_bits(),
                    ((d as i32) as f32 * s44).to_bits(),
                    ((s as i32) as f32 * s52).to_bits(),
                ];
                callee_cdecl!(DRAW, u32, p2.as_ptr() as u32, p1.as_ptr() as u32);
            }
            s52 += step;
            s48 += step;
            cl = !cl;
            edx += 1;
        }
    }
});

const F_ENABLE: usize = 0x44c;

#[inline(always)]
unsafe fn u8_at(this: u32, off: usize) -> u8 {
    unsafe { ((this as usize + off) as *const u8).read() }
}

/// Exact `cvttss2si` semantics: truncate toward zero, out-of-range or NaN
/// yields `i32::MIN` (Rust's `as` would saturate instead).
#[inline(always)]
fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// The `comiss`/`jbe`/`ja` clamp idiom: `v` clamped to `[0.0, hi]`,
/// propagating NaN exactly like the instruction sequence.
#[inline(always)]
fn clamp_lo0(v: f32, hi: f32) -> f32 {
    if v < 0.0 {
        0.0
    } else if !(v > hi) {
        v
    } else {
        hi
    }
}

#[inline(always)]
unsafe fn f32_at(this: u32, off: usize) -> f32 {
    unsafe { ((this as usize + off) as *const f32).read_unaligned() }
}

#[inline(always)]
unsafe fn coin_pick(a: u32, b: u32) -> u32 {
    let ans: u32 = callee_cdecl!(COIN, u32,);
    if (ans as u8) != 0 {
        b
    } else {
        a
    }
}

// Callee ids (shared by both contracts).
const CFG: u32 = 1; // config pair fetch: (out: *mut [u32; 2], id) -> answers

const CFG_C: u32 = 0x82;

const GATE_FLAG: u32 = 0x01161668;

const ADJ: u32 = 2; // adjustment fetch, this = GATE_ADDR, () -> u32

const GATE_ADDR: u32 = 0x01161668;

const COL: u32 = 3; // color fetch: (out: *mut u32, id) -> answers

const COL_ID2: u32 = 0x42;

const CFG_D: u32 = 0x81;

const F260: u32 = 6; // scaler: (mode, null, out: *mut [u32; 2], null)

const F_COUNT1: usize = 0x313;

const F_B: usize = 0x32c;

const F_X0: usize = 0x320;

const F_A: usize = 0x328;

const F_YBASE: usize = 0x43c;

const F_Y0: usize = 0x314;

const F_YSPAN: usize = 0x31c;

const F_MODE: usize = 0x395;

const F_YSTEP: usize = 0x318;

const G1A: u32 = 0x0105c880;

const G1B: u32 = 0x0105c87c;

const G2A: u32 = 0x0105c884;

const G2B: u32 = 0x0105c888;

const F_LOOP: usize = 0x310;

const F_XSTEP: usize = 0x324;

const DRAW: u32 = 5; // quad draw: (pos: *const [u32; 4], style: *const [u32; 4])

const COIN: u32 = 4; // two-way selector: () -> u32, low byte decides
