// original: 0x0088e8a0 audio voice per-frame update (unnamed in symbols)
#[inline(always)]
unsafe fn r8(base: *mut u8, off: u32) -> u8 {
    *base.add(off as usize)
}

#[inline(always)]
unsafe fn r16(base: *mut u8, off: u32) -> u16 {
    *(base.add(off as usize) as *const u16)
}

#[inline(always)]
unsafe fn r32(base: *mut u8, off: u32) -> u32 {
    *(base.add(off as usize) as *const u32)
}

#[inline(always)]
unsafe fn rf(base: *mut u8, off: u32) -> f32 {
    *(base.add(off as usize) as *const f32)
}

#[inline(always)]
unsafe fn wf(base: *mut u8, off: u32, v: f32) {
    *(base.add(off as usize) as *mut f32) = v;
}

/// Decode the 16-bit half float the voice header stores into f32 bits.
/// Bit-exact port of the original's integer sequence (sign, exponent+0x70,
/// mantissa placed into single-precision slots); zero stays positive zero.
#[inline(always)]
fn half_to_f32_bits(h: u16) -> u32 {
    if h == 0 {
        return 0;
    }
    let (mut edx, mut ecx, mut eax) = (h as u32, h as u32, h as u32);
    ecx &= 0xFFFF8000;
    edx >>= 10;
    ecx <<= 3;
    edx &= 0x1F;
    eax &= 0x3FF;
    ecx |= eax;
    edx += 0x70;
    ecx <<= 13;
    edx <<= 23;
    ecx | edx
}

/// Slew-rate limiter used by the voice update: nudge `cur` one step toward
/// `new`, then adopt the nudged value only when the gap exceeds the step.
/// The negated comparisons reproduce the original's jump-if-below after a
/// scalar float compare, which also takes the branch on unordered (NaN).
#[inline(always)]
fn snap_toward(new: f32, cur: f32, step: f32, abs_mask: u32) -> f32 {
    let d = new - cur;
    let adj = if !(d >= 0.0) { cur - step } else { cur + step };
    let m = f32::from_bits(d.to_bits() & abs_mask) - step;
    if m >= 0.0 {
        adj
    } else {
        new
    }
}

/// Per-frame update of a software audio voice: smooth the two channel gains
/// and six band levels toward their targets (optionally through the virtual
/// slew check), rebuild the mix matrix rows for this voice kind, hand each
/// row to the mixer, and commit the voice state.
export!(thiscall, rw_0088e8a0(this: *mut u8) -> u32 {
    unsafe {
        const UNITY: u32 = 0xFE88E8;
        const DB_STEP: u32 = 0xE77F90;
        const SNAP: u32 = 0x103009C;
        const ABS_MASK: u32 = 0xFE8F80;
        const FX_CHAIN: u32 = 0x115DA4C;
        let unity: f32 = *global::<f32>(UNITY);
        let db: f32 = *global::<f32>(DB_STEP);
        let snap: f32 = *global::<f32>(SNAP);
        let abs: u32 = *global::<u32>(ABS_MASK);

        // Slew smoothing runs only when the flag is clear and the virtual
        // check on this voice answers nonzero.
        let smooth = if r8(this, 0x8C) & 8 == 0 {
            let vt = r32(this, 0) as *const u32;
            let check: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(*vt.add(6) as usize);
            check(this as u32) & 0xFF != 0
        } else {
            false
        };
        let hdr = r32(this, 4) as *mut u8;

        // Base gain from the voice header's half float, muted by its flag.
        let scale = if r8(hdr, 0x18) & 0x80 != 0 {
            0.0
        } else {
            unity
        };
        let half = r16(hdr, 0x1A);
        let mut g0 = if half == 0 {
            0.0
        } else {
            f32::from_bits(half_to_f32_bits(half))
        };

        // First channel gain.
        let mut g7 = rf(hdr, 0x20);
        g0 *= scale;
        g7 *= db;
        let band_scale = g0;
        if smooth {
            g7 = snap_toward(g7, rf(this, 0x80), snap, abs);
        }

        // Second channel gain.
        g0 = rf(hdr, 0x1C);
        g0 *= db;
        g7 *= scale;
        let row_scale = g7;
        if smooth {
            g0 = snap_toward(g0, rf(this, 0x84), snap, abs);
        }
        g0 *= scale;
        wf(this, 0x80, g7);
        wf(this, 0x84, g0);
        let tail_gain = g0;

        // Six band levels, each with a triple of coefficients.
        let mut src = hdr as u32 + 0x24;
        let mut coef = this as u32 + 0x38;
        let mut band = this.add(0x20);
        let mut aux = hdr as u32 + 0x40;
        let mut b = 0u32;
        loop {
            let mut target = *((src) as *const f32);
            target *= band_scale;
            let mut slot = [0.0f32; 3];
            if b != 2 && b != 3 {
                let mut x = *(((aux - 4) as *const u8) as *const f32);
                x *= band_scale;
                aux += 12;
                slot[0] = x;
                x = *(((aux - 12) as *const u8) as *const f32);
                x *= band_scale;
                slot[1] = x;
                x = *(((aux - 8) as *const u8) as *const f32);
                x *= band_scale;
                slot[2] = x;
            }
            let v = if smooth {
                snap_toward(target, *(band as *const f32), snap, abs)
            } else {
                target
            };
            *(band as *mut f32) = v;
            for k in 0..3 {
                let c = if smooth {
                    snap_toward(slot[k], *((coef) as *const f32), snap, abs)
                } else {
                    slot[k]
                };
                *((coef) as *mut f32) = c;
                coef += 4;
            }
            src += 4;
            band = band.add(4);
            b += 1;
            if b >= 6 {
                break;
            }
        }

        // Scratch rows for the mix matrix (cleared by the intercepted call
        // on the original side too, so only written words are observed).
        let mut rows = [0.0f32; 30];
        let mut copies = [0.0f32; 30];
        let _: u32 = callee_cdecl!(2, u32, rows.as_mut_ptr() as u32, 0, 0x78);
        let _: u32 = callee_cdecl!(2, u32, copies.as_mut_ptr() as u32, 0, 0x78);

        // One matrix row per active slot: scaled levels plus the raw copy.
        // Group g occupies words 6*g+2..6*g+8 of each buffer.
        let mut fill_row = |g: usize, base: u32| {
            for k in 0..6 {
                let v = rf(this, base.wrapping_add(k as u32 * 4));
                rows[g * 6 + 2 + k] = v * row_scale;
                copies[g * 6 + 2 + k] = v;
            }
        };
        let fx = relocated(FX_CHAIN);
        // (matrix rows filled, mixer rows expected)
        let (ngroups, nrows) = match r8(hdr, 0x6C) {
            1 | 2 => {
                let slot = if r8(hdr, 0x6C) == 1 { 3 } else { 4 };
                let found: u32 = callee_thiscall!(3, u32, fx, slot);
                if found == 0 {
                    (0, 0)
                } else {
                    fill_row(0, 0x20);
                    (1, 1)
                }
            }
            _ => {
                for slot in [5u32, 0, 1, 2] {
                    let _: u32 = callee_thiscall!(3, u32, fx, slot);
                }
                fill_row(0, 0x20);
                let offs = [-0xCi32, 0, 0xC, 0x18, 0x24, 0x30];
                for (gi, base) in [0x44u32, 0x48, 0x4C].iter().enumerate() {
                    for (k, o) in offs.iter().enumerate() {
                        let v = rf(this, base.wrapping_add(*o as u32));
                        rows[(gi + 1) * 6 + 2 + k] = v * row_scale;
                        copies[(gi + 1) * 6 + 2 + k] = v;
                    }
                }
                (4, 4)
            }
        };

        if r32(this, 0x144) != 0 {
            // Hand the raw rows to the mixer when the voice is routed.
            if r8(hdr, 0x18) & 1 != 0 {
                for i in 0..nrows {
                    let _: u32 = callee_thiscall!(
                        4,
                        u32,
                        r32(this, 0x144),
                        i as u32,
                        copies.as_mut_ptr().add(i * 6 + 2) as u32
                    );
                }
            }
            // Termination row; slot 4 overlaps the copy buffer's first words.
            let base = (6 * ngroups + 2) as usize;
            for k in 0..6 {
                let w = base + k;
                if w < 30 {
                    rows[w] = tail_gain;
                } else {
                    copies[w - 30] = tail_gain;
                }
            }
            for i in 0..ngroups + 1 {
                let _: u32 = callee_thiscall!(
                    5,
                    u32,
                    r32(this, 0x140),
                    i as u32,
                    rows.as_mut_ptr().add(i as usize * 6 + 2) as u32
                );
            }
        } else {
            for i in 0..ngroups {
                let _: u32 = callee_thiscall!(
                    5,
                    u32,
                    r32(this, 0x140),
                    i as u32,
                    rows.as_mut_ptr().add(i as usize * 6 + 2) as u32
                );
            }
        }

        // Commit the voice state.
        let lo = r16(hdr, 0x10) as u32;
        let hi = r16(hdr, 0x12) as u32;
        let r: u32 = callee_thiscall!(6, u32, r32(this, 0x140), lo, hi);
        r
    }
});
