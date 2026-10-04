// original: 0x88ca40 audio_voice_update_volume
// Level follower for a voice: decode the half-precision peak, slew the six
// channel followers toward their targets, scale by the voice gain, take the
// peak, and drive the buffer volume through a log curve with a silence floor.
// The log-curve helper takes its argument in xmm0, so the checker ships the
// peak on the stack and the stub reloads it (xmm0 transport).

lf_k2_rt::export!(thiscall, rw_0088ca40(this: *mut u8) -> () {
    unsafe {
        let rd = |off: usize| -> u32 { *(this.add(off) as *const u32) };
        let params = rd(4);
        let m16 = |off: usize| -> u16 { *((params as *const u8).add(off) as *const u16) };
        let m8 = |off: usize| -> u8 { *((params as *const u8).add(off)) };
        let mf = |off: usize| -> f32 {
            f32::from_bits(*((params as *const u8).add(off) as *const u32))
        };

        // Half-precision peak decode (no subnormal/infinity cases).
        let h = m16(0x1a) as u32;
        let peak_bits = if h == 0 {
            0u32
        } else {
            let mut sign = h;
            let mut exp = h;
            let mut mant = h;
            sign &= 0xffff8000;
            exp >>= 10;
            sign <<= 3;
            exp &= 0x1f;
            mant &= 0x3ff;
            sign |= mant;
            exp += 0x70;
            sign <<= 13;
            exp <<= 23;
            sign | exp
        };
        let peak = f32::from_bits(peak_bits);

        // Voice gain scaled by the tuning constant.
        let scaled_gain = mf(0x20) * f32::from_bits(0x3fa123a3);

        // Slew limiting is off while the bypass flag is set or the gate
        // objects.
        let mut slew = false;
        if m8(0x18) & 0x40 == 0 {
            let vt = *(this as *const u32);
            let tgt = *((vt as *const u8).add(0x18) as *const u32);
            let gate: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tgt as usize);
            slew = gate(this as u32) & 0xff != 0;
        }

        // Six channel followers track their targets.
        let step = f32::from_bits(*lf_k2_rt::global::<u32>(0x103009c));
        let mut k = 0usize;
        while k < 6 {
            let target = mf(0x24 + k * 4) * peak;
            let cur = f32::from_bits(*((this as *const u8).add(0x20 + k * 4) as *const u32));
            let mut v = target;
            if slew {
                let d = target - cur;
                let cand = if d >= 0.0 { step + cur } else { cur - step };
                let ad = f32::from_bits(d.to_bits() & 0x7fffffff);
                if ad - step >= 0.0 {
                    v = cand;
                }
            }
            *((this as *mut u8).add(0x20 + k * 4) as *mut u32) = v.to_bits();
            k += 1;
        }

        // Peak of the gain-scaled followers, halved for the middle modes.
        let mut m = 0.0f32;
        let mut k = 0usize;
        while k < 6 {
            let v = f32::from_bits(*((this as *const u8).add(0x20 + k * 4) as *const u32))
                * scaled_gain;
            if v > m {
                m = v;
            }
            k += 1;
        }
        let mode = m8(0x6c);
        if mode == 1 || mode == 2 {
            m = m * f32::from_bits(*lf_k2_rt::global::<u32>(0xfe8830));
        }

        // Silence floor, else the log curve mapped to millibels.
        let buf = rd(0x90);
        let vt = *(buf as *const u32);
        let set_vol: extern "stdcall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vt as *const u8).add(0x3c) as *const u32));
        let floor = f32::from_bits(0x3727c5ac);
        if floor >= m {
            set_vol(buf, 0xffffd8f0);
        } else {
            let rbits: u32 = lf_k2_rt::callee_cdecl!(3, u32, m.to_bits());
            let x3 = f32::from_bits(rbits) * f32::from_bits(0x44fa0000);
            let x3b = x3.to_bits();
            let sign = x3b & 0x80000000;
            let ax = f32::from_bits(x3b ^ sign);
            let mask: u32 = if ax < f32::from_bits(0x4b000000) { 0xffffffff } else { 0 };
            let x2 = f32::from_bits((0x4b000000 & mask) | sign);
            let mut x1 = x3 + x2;
            x1 = x1 - x2;
            let frac = x1 - x3;
            let adjust: u32 = if frac > f32::from_bits(sign) { 0xffffffff } else { 0 };
            x1 = x1 - f32::from_bits(0x3f800000 & adjust);
            let edx: i32 = if x1.is_nan() || x1 >= 2147483648.0 || x1 < -2147483648.0 {
                0x80000000u32 as i32
            } else {
                x1 as i32
            };
            let vol: i32 = if edx < -10000 {
                -10000
            } else if edx > 0 {
                0
            } else {
                edx
            };
            set_vol(buf, vol as u32);
        }
    }
});
