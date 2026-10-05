// original: 0x00d536c0 CCamFree::vf4
/// Free-camera frame update: samples the four movement axes (live device
/// polls or the stored config block), integrates position, orientation,
/// zoom and roll, and notifies dependents.
///
/// Original: thiscall/0 (`this` in ECX), always returns 1.
/// Callee ids (all direct, patched): 1 device-open (cdecl/0); 2-5 channel
/// reads (thiscall/0); 6 int convert (cdecl/1); 7 rate notify (thiscall/1);
/// 8 pitch easing (cdecl/1, float answer); 9 pose publish (thiscall/1,
/// frame-pointer arg skipped, 3-word snapshot compared); 10 dependent
/// notify (cdecl/1); 11 attach (thiscall/1).
/// Reads globals for gates, toggles, axis config and float constants;
/// writes the camera object (+0x40 pose, +0x60 zoom, +0x140-0x158 rates),
/// two latch bytes, and two flag bits inside the id11 gate.
/// Float edge cases: int-to-float widens SIGNED after negation (0x80000000
/// stays negative); zero-magnitude vertical drive stores +0.0; symmetric
/// clamps preserve signed zeros; integrate/strafe operand order is pinned
/// (NaN payloads compared bit for bit).
/// Proof limits: `stack` check off (frame scratch uncomparable); the
/// alternate-path latch never fires ([0x118D400] pinned 0); fulldata off.
export!(thiscall, rw_00d536c0(this: u32) -> u32 {
    unsafe {
        // One device poll: open the device, read the channel, convert.
        let poll = |id: u32| {
            let dev: u32 = callee_cdecl!(1, u32,);
            let raw: u32 = callee_thiscall!(id, u32, dev);
            callee_cdecl!(6, u32, raw)
        };
        let cvt = |n: u32| (n as i32) as f32;
        // Latching toggle for the alternate input path.
        if *global::<u32>(0x118D400) != 0 && *global::<u32>(0x118D450) == 0 {
            let f = global::<u8>(0x1720FE9);
            *f = (*f == 0) as u8;
        }
        let alt = *global::<u8>(0x1720FE9);
        let live = *global::<u8>(0x117E6D8);
        // Four axis values, polled live or taken from the config block.
        let (v30, v2c, v10, v28): (f32, f32, f32, f32);
        if alt == 0 {
            if live != 0 {
                v30 = cvt(poll(2));
                v2c = cvt((poll(3) as i32).wrapping_neg() as u32);
                v10 = cvt(poll(4));
                v28 = cvt((poll(5) as i32).wrapping_neg() as u32);
            } else {
                v30 = cvt(*global::<u32>(0x118D3B4));
                v2c = cvt((*global::<u32>(0x118D3B8) as i32).wrapping_neg() as u32);
                v10 = cvt(*global::<u32>(0x118D3BC));
                v28 = cvt((*global::<u32>(0x118D3C0) as i32).wrapping_neg() as u32);
            }
        } else if live != 0 {
            v30 = cvt(poll(4));
            v2c = cvt(poll(5));
            v10 = cvt(poll(2));
            v28 = 0.0;
        } else {
            v30 = cvt(*global::<u32>(0x118D3BC));
            v2c = cvt(*global::<u32>(0x118D3C0));
            v10 = cvt(*global::<u32>(0x118D3B4));
            v28 = 0.0;
        }
        // Rate scale from the global timer (or unity) over the zoom factor.
        let one = *global::<f32>(0xFE88E8);
        let s = if *((this + 0x15c) as *const u8) != 0 {
            *global::<f32>(0x11735BC) * *global::<f32>(0xFE8B48)
        } else {
            one
        };
        let d = one / *((this + 0x160) as *const f32);
        let m = *((this + 0x164) as *const f32) * s;
        // Per-axis rates.
        let r4c = *((this + 0x168) as *const f32) * m * d;
        let r50 = m;
        let r54 = *((this + 0x184) as *const f32) * m * d;
        let r58 = *((this + 0x178) as *const f32) * m * d;
        let r5c = *((this + 0x16c) as *const f32) * m * d;
        let r60 = *((this + 0x170) as *const f32) * m * d;
        let r64 = *((this + 0x174) as *const f32) * m * d;
        let r68 = *((this + 0x17c) as *const f32) * m * d;
        let r6c = *((this + 0x180) as *const f32) * m * d;
        let _: u32 = callee_thiscall!(7, u32, this, m.to_bits());
        // Vertical rate: stepped by the toggle pair, else driven polled.
        let c = *global::<u8>(0x1720FE9);
        if c == 0 {
            if *global::<u32>(0x118D3DC) != 0 {
                let v = *((this + 0x140) as *const f32);
                *((this + 0x140) as *mut f32) = v + r4c;
            } else if *global::<u32>(0x118D3E8) != 0 {
                let v = *((this + 0x140) as *const f32);
                *((this + 0x140) as *mut f32) = v - r4c;
            }
        } else {
            let n: u32 = if *global::<u8>(0x117E6D8) != 0 {
                poll(3)
            } else {
                *global::<u32>(0x118D3B8)
            };
            // SIGNED conversion (the original widens with a signed
            // dword-to-float op after negating): 0x80000000 negates to
            // itself and converts to -2147483648.0, not +2147483648.0.
            let f = (n as i32).wrapping_neg() as f32;
            if f.abs() > 0.0 {
                *((this + 0x140) as *mut f32) = f * *global::<f32>(0xFE8960) * r4c;
            } else {
                *((this + 0x140) as *mut u32) = 0;
            }
        }
        // Clamp the three rates into their symmetric limits.
        let lo = if r58 > r5c { r5c } else { r58 };
        let v = *((this + 0x140) as *const f32);
        let x = if -lo > v { -lo } else if v > lo { lo } else { v };
        *((this + 0x140) as *mut f32) = x;
        let y0 = r60 * v10;
        let neg8 = -r58;
        let y = if neg8 > y0 { neg8 } else if y0 > r58 { r58 } else { y0 };
        *((this + 0x144) as *mut f32) = y;
        let z0 = r64 * v28;
        let z = if neg8 > z0 { neg8 } else if z0 > r58 { r58 } else { z0 };
        *((this + 0x148) as *mut f32) = z;
        // Integrate the position along the view direction. Operand order is
        // pinned with black_box: the multiply orders are (x*r20), (r24*x),
        // (r28*x) and each add is ([mem]+product); rustc otherwise commutes
        // one of them, which changes NaN payloads (trial 119 caught it).
        let r20 = *((this + 0x20) as *const f32);
        let r24 = *((this + 0x24) as *const f32);
        let r28 = *((this + 0x28) as *const f32);
        let bb = core::hint::black_box;
        let t_nx = bb(x) * bb(r20);
        let nx = bb(*((this + 0x40) as *const f32)) + bb(t_nx);
        let t_ny = bb(r24) * bb(x);
        let ny = bb(*((this + 0x44) as *const f32)) + bb(t_ny);
        let t_nz = bb(r28) * bb(x);
        let nz = bb(*((this + 0x48) as *const f32)) + bb(t_nz);
        *((this + 0x40) as *mut f32) = nx;
        *((this + 0x44) as *mut f32) = ny;
        *((this + 0x48) as *mut f32) = nz;
        // Strafe on the plane, or on the full basis when tilted. Order
        // pinned (see above): planar is (r144*r50)+nx / (r148*r50)+ny;
        // full-basis builds (([0x10/14/18]*r144)*r50)+n_ then adds
        // ((r148*[0x30])*r50), (([0x34]*r148)*r50), (([0x38]*r148)*r50).
        if *global::<u32>(0x118D3C4) == 0 && c == 0 {
            let r144 = *((this + 0x144) as *const f32);
            let r148 = *((this + 0x148) as *const f32);
            *((this + 0x40) as *mut f32) = bb(bb(r144) * bb(r50)) + bb(nx);
            *((this + 0x44) as *mut f32) = bb(bb(r148) * bb(r50)) + bb(ny);
        } else {
            let r144 = *((this + 0x144) as *const f32);
            let r148 = *((this + 0x148) as *const f32);
            let t4 = bb(bb(bb(*((this + 0x10) as *const f32)) * bb(r144)) * bb(r50)) + bb(nx);
            let t5 = bb(bb(bb(*((this + 0x14) as *const f32)) * bb(r144)) * bb(r50)) + bb(ny);
            let t6 = bb(bb(bb(*((this + 0x18) as *const f32)) * bb(r144)) * bb(r50)) + bb(nz);
            *((this + 0x40) as *mut f32) =
                bb(bb(bb(r148) * bb(*((this + 0x30) as *const f32))) * bb(r50)) + bb(t4);
            *((this + 0x44) as *mut f32) =
                bb(bb(bb(*((this + 0x34) as *const f32)) * bb(r148)) * bb(r50)) + bb(t5);
            *((this + 0x48) as *mut f32) =
                bb(bb(bb(*((this + 0x38) as *const f32)) * bb(r148)) * bb(r50)) + bb(t6);
        }
        // Zoom stepped by its toggle pair, or snapped to the default.
        let mut zoom = *((this + 0x60) as *const f32);
        if *global::<u32>(0x118D3F8) != 0 && *global::<f32>(0xEE5AE0) > zoom {
            zoom += *global::<f32>(0xFE87E8);
        }
        if *global::<u32>(0x118D3F4) != 0 && zoom > *global::<f32>(0xFE881C) {
            zoom -= *global::<f32>(0xFE87E8);
        }
        if *global::<u32>(0x118D3F0) != 0
            && *global::<u32>(0x118D3DC) == 0
            && *global::<u32>(0x118D3E8) == 0
            && *global::<u32>(0x118D3CC) == 0
            && *global::<u32>(0x118D3D8) == 0
        {
            zoom = *global::<f32>(0xFE8B64);
        }
        *((this + 0x60) as *mut f32) = zoom;
        // Roll stepped by its toggle pair.
        if *global::<u32>(0x118D3E0) != 0 {
            if *global::<u32>(0x118D3D0) != 0 {
                *((this + 0x14c) as *mut f32) = r54 + *((this + 0x14c) as *const f32);
            } else if *global::<u32>(0x118D3C4) != 0 {
                *((this + 0x14c) as *mut f32) = *((this + 0x14c) as *const f32) - r54;
            } else {
                *((this + 0x14c) as *mut u32) = 0;
            }
            if *global::<u32>(0x118D3D0) != 0 && *global::<u32>(0x118D3C4) != 0 {
                *((this + 0x158) as *mut u32) = 0;
                *((this + 0x14c) as *mut u32) = 0;
            }
        } else {
            *((this + 0x14c) as *mut u32) = 0;
        }
        // Pitch solved through the easing helper, then clamped.
        let r150 = *((this + 0x150) as *const f32);
        *((this + 0x150) as *mut f32) = r150 - r68 * v2c;
        let r154 = *((this + 0x154) as *const f32);
        let e: f32 = callee_cdecl!(8, f32, (r154 - r6c * v30).to_bits());
        *((this + 0x154) as *mut f32) = e;
        let p0 = *((this + 0x150) as *const f32);
        let plo = *global::<f32>(0xEE5AE4);
        let phi = *global::<f32>(0xEE5ADC);
        let p = if plo > p0 { plo } else if p0 > phi { phi } else { p0 };
        *((this + 0x150) as *mut f32) = p;
        let r158 = r50 * *((this + 0x14c) as *const f32) + *((this + 0x158) as *const f32);
        *((this + 0x158) as *mut f32) = r158;
        let frame = [p.to_bits(), r158.to_bits(), e.to_bits()];
        let _: u32 = callee_thiscall!(9, u32, this + 0x10, frame.as_ptr() as u32);
        // Latching toggle for the dependent notify.
        if *global::<u32>(0x118D3C8) != 0 && *global::<u32>(0x118D418) == 0 {
            let f = global::<u8>(0x1720FE8);
            *f = (*f == 0) as u8;
        }
        if *global::<u8>(0x1720FE8) != 0 {
            let _: u32 = callee_cdecl!(10, u32, this + 0x40);
        }
        // Final attach while its toggle pair is armed, then flag clears.
        // The flag clears are INSIDE this gate: both early exits jump past
        // them to the epilogue (a-Q01: the v2 lane misread this as an
        // independent trailing if, which caused its 22/60 failures).
        if *global::<u32>(0x118D3D4) != 0 && *global::<u32>(0x118D424) == 0 {
            let _: u32 = callee_thiscall!(11, u32, this, this + 0x40);
            if *global::<u32>(0x118D3C4) != 0 {
                let b = (*((this + 0x118) as *const u32) + 0x13c) as *mut u8;
                *b &= 0xFB;
                *b &= 0xF7;
            }
        }
        1
    }
});
