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
