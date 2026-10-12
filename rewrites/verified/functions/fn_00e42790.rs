// original: 0x00e42790 multi_pass_quad_draw_a
/// Multi-pass quad draw, variant A.
///
/// When the enable flag is clear it returns immediately. Otherwise it fetches
/// two config pairs and a color, derives an alpha byte from an adjustment
/// value, draws one quad, then draws one quad per row of the first list
/// (skipping row 1 in the non-mixed mode), one summary quad, and one quad
/// for the head row of the second list.
export!(thiscall, rw_00e42790(this: u32) -> () {
    unsafe {
        if u8_at(this, F_ENABLE) == 0 {
            return;
        }
        let y_base = f32_at(this, F_YBASE);
        let x_base = f32_at(this, F_XBASE);

        // Config pairs; every call answers through its returned pointer.
        let mut cfg_out = [0u32; 2];
        let p = callee_cdecl!(CFG, u32, cfg_out.as_mut_ptr() as u32, CFG_A);
        let fa = f32::from_bits((p as *const u32).read());
        let p = callee_cdecl!(CFG, u32, cfg_out.as_mut_ptr() as u32, CFG_A);
        let fb = f32::from_bits((p as *const u32).add(1).read());
        let p = callee_cdecl!(CFG, u32, cfg_out.as_mut_ptr() as u32, CFG_B);
        let mut ebx = cvtt(f32::from_bits((p as *const u32).read()));
        let p = callee_cdecl!(CFG, u32, cfg_out.as_mut_ptr() as u32, CFG_B);
        let mut s44 = cvtt(f32::from_bits((p as *const u32).add(1).read()));

        // When the gate flag is clear the whole adjustment block below is
        // skipped: no adjustment calls, no further config calls, and the
        // working alpha bytes keep their prologue values.
        let mut bl0 = ebx as u8;
        if global::<u8>(GATE_FLAG).read() != 0 {
            let lo = callee_thiscall!(ADJ, u32, relocated(GATE_ADDR));
            let hi = callee_thiscall!(ADJ, u32, relocated(GATE_ADDR));
            ebx = (ebx & !0xffff) | ((lo & 0xff) as i32) | (((hi & 0xff) as i32) << 8);
            let p = callee_cdecl!(CFG, u32, cfg_out.as_mut_ptr() as u32, CFG_B);
            let hi1 = f32::from_bits((p as *const u32).add(1).read());
            let t = clamp_lo0((ebx & 0xff) as f32, hi1);
            s44 = (s44 & !0xff) | (cvtt(t) & 0xff);
            let p = callee_cdecl!(CFG, u32, cfg_out.as_mut_ptr() as u32, CFG_B);
            let hi0 = f32::from_bits((p as *const u32).read());
            let t = clamp_lo0(((ebx >> 8) & 0xff) as f32, hi0);
            bl0 = cvtt(t) as u8;
        }

        // Color + header quad.
        let mut col_out = [0u32; 1];
        let p = callee_cdecl!(COL, u32, col_out.as_mut_ptr() as u32, COL_ID);
        let color = (p as *const u32).read();
        let s40 = y_base;
        let mut s52 = y_base + fa;
        let alpha = (bl0 as u32) << 24;
        let mut s60 = (color & 0x00ff_ffff) | alpha;
        let g1a = global::<u32>(G1A).read();
        let g1b = global::<u32>(G1B).read();
        let g2a = global::<u32>(G2A).read();
        let g2b = global::<u32>(G2B).read();
        let k = global::<f32>(K_POS).read();
        let b = coin_pick(g1a, g1b);
        let d = coin_pick(g2a, g2b);
        let s = coin_pick(g1a, g1b);
        let c = coin_pick(g2a, g2b);
        let mut p1 = [s60, fa.to_bits(), s52.to_bits(), alpha];
        let mut p2 = [
            ((c as i32) as f32 * s40).to_bits(),
            ((b as i32) as f32).to_bits(),
            ((d as i32) as f32 * s52).to_bits(),
            ((s as i32) as f32 * k).to_bits(),
        ];
        callee_cdecl!(DRAW, u32, p2.as_ptr() as u32, p1.as_ptr() as u32);

        // Row list.
        let count1 = u8_at(this, F_COUNT1);
        let y0 = f32_at(this, F_Y0);
        let ystep = f32_at(this, F_YSTEP);
        let yspan = f32_at(this, F_YSPAN);
        let mut edx: u32 = 0;
        let mut row_idx: u32 = 0;
        while edx < count1 as u32 {
            let mode = u8_at(this, F_MODE);
            if !(mode == 0 && edx == 1) {
                let eax = s44 as u32;
                s60 = ((eax & 0xff) << 24) | (s60 & 0x00ff_ffff);
                let mut x2 = fb;
                let x1: f32;
                if edx == 0 {
                    x1 = y0 + y_base;
                    x2 = fa;
                    s60 = (s60 & 0x00ff_ffff) | alpha;
                } else if edx == 1 {
                    x1 = y0 + y_base + ystep;
                    x2 = fa;
                    s60 = (s60 & 0x00ff_ffff) | alpha;
                } else if mode != 0 {
                    let e = (edx - 1) as i32 as f32;
                    x1 = y0 + y_base + ystep + e * yspan;
                } else {
                    let e = (edx - 1) as i32 as f32;
                    // Operand order preserved: (e*yspan) + (y0+ybase).
                    x1 = e * yspan + (y0 + y_base);
                }
                s52 = x1;
                let s32 = x2 + x1;
                let b = coin_pick(g1a, g1b);
                let d = coin_pick(g2a, g2b);
                let s = coin_pick(g1a, g1b);
                let c = coin_pick(g2a, g2b);
                p1 = [s60, fa.to_bits(), s52.to_bits(), alpha];
                p2 = [
                    ((c as i32) as f32 * s52).to_bits(),
                    ((b as i32) as f32).to_bits(),
                    ((d as i32) as f32 * s32).to_bits(),
                    ((s as i32) as f32 * k).to_bits(),
                ];
                callee_cdecl!(DRAW, u32, p2.as_ptr() as u32, p1.as_ptr() as u32);
                edx = row_idx;
            }
            edx += 1;
            row_idx = edx;
        }

        // Summary quad.
        s60 = (s60 & 0x00ff_ffff) | alpha;
        let s40b = y_base;
        let e = ((count1 as i32).wrapping_sub(2)) as f32;
        let mut x0 = y0 + y_base;
        let mut x1 = e * yspan;
        if u8_at(this, F_MODE) != 0 {
            x0 += ystep;
        }
        x1 += x0;
        let s32 = x_base;
        let s36 = x_base + fa;
        let s48f = x1;
        let b = coin_pick(g1a, g1b);
        let d = coin_pick(g2a, g2b);
        let s = coin_pick(g1a, g1b);
        let c = coin_pick(g2a, g2b);
        p1 = [s60, fa.to_bits(), s52.to_bits(), s48f.to_bits()];
        p2 = [
            ((c as i32) as f32 * s40b).to_bits(),
            ((b as i32) as f32 * s36).to_bits(),
            ((d as i32) as f32 * s48f).to_bits(),
            ((s as i32) as f32 * s32).to_bits(),
        ];
        callee_cdecl!(DRAW, u32, p2.as_ptr() as u32, p1.as_ptr() as u32);

        // Second list: only the head row draws.
        let count2 = u8_at(this, F_COUNT2);
        let x0b = f32_at(this, F_X0);
        let xstep = f32_at(this, F_XSTEP);
        let mut ecx: u32 = 0;
        let mut head_idx: u32 = 0;
        while ecx < count2 as u32 {
            let q0 = x0b + x_base;
            let mut q1 = (ecx as i32 as f32) * xstep;
            q1 += q0;
            let s32b = q1;
            let s36b = q1 + fa;
            if ecx == 0 {
                let b = coin_pick(g1a, g1b);
                let d = coin_pick(g2a, g2b);
                let s = coin_pick(g1a, g1b);
                let c = coin_pick(g2a, g2b);
                p1 = [s60, fa.to_bits(), s52.to_bits(), s48f.to_bits()];
                p2 = [
                    ((c as i32) as f32 * s40b).to_bits(),
                    ((b as i32) as f32 * s36b).to_bits(),
                    ((d as i32) as f32 * s48f).to_bits(),
                    ((s as i32) as f32 * s32b).to_bits(),
                ];
                callee_cdecl!(DRAW, u32, p2.as_ptr() as u32, p1.as_ptr() as u32);
                ecx = head_idx;
            }
            ecx += 1;
            head_idx = ecx;
        }
    }
});

const F_ENABLE: usize = 0x44c;

#[inline(always)]
unsafe fn u8_at(this: u32, off: usize) -> u8 {
    unsafe { ((this as usize + off) as *const u8).read() }
}

#[inline(always)]
unsafe fn f32_at(this: u32, off: usize) -> f32 {
    unsafe { ((this as usize + off) as *const f32).read_unaligned() }
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
unsafe fn coin_pick(a: u32, b: u32) -> u32 {
    let ans: u32 = callee_cdecl!(COIN, u32,);
    if (ans as u8) != 0 {
        b
    } else {
        a
    }
}

const F_YBASE: usize = 0x43c;

const F_XBASE: usize = 0x440;

// Callee ids (shared by both contracts).
const CFG: u32 = 1; // config pair fetch: (out: *mut [u32; 2], id) -> answers

const CFG_A: u32 = 0x7e;

const CFG_B: u32 = 0x5f;

const GATE_FLAG: u32 = 0x01161668;

const ADJ: u32 = 2; // adjustment fetch, this = GATE_ADDR, () -> u32

const GATE_ADDR: u32 = 0x01161668;

const COL: u32 = 3; // color fetch: (out: *mut u32, id) -> answers

const COL_ID: u32 = 0x41;

const G1A: u32 = 0x0105c880;

const G1B: u32 = 0x0105c87c;

const G2A: u32 = 0x0105c884;

const G2B: u32 = 0x0105c888;

const K_POS: u32 = 0x00fe8628;

const DRAW: u32 = 5; // quad draw: (pos: *const [u32; 4], style: *const [u32; 4])

const F_COUNT1: usize = 0x313;

const F_Y0: usize = 0x314;

const F_YSTEP: usize = 0x318;

const F_YSPAN: usize = 0x31c;

const F_MODE: usize = 0x395;

// Field offsets in the object.
const F_COUNT2: usize = 0x312;

const F_X0: usize = 0x320;

const F_XSTEP: usize = 0x324;

const COIN: u32 = 4; // two-way selector: () -> u32, low byte decides
