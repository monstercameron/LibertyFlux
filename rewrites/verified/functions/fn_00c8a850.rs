// original: 0x00c8a850 audio_event_router
/// Audio event router: classifies a sound request and dispatches it.
///
/// Reads three threshold flags out of the request record, then switches
/// on the voice class in the low bits of the voice word:
/// - class 3 resolves the request key through a calibrated table and,
///   depending on the flags, fires one of three mixer requests, then an
///   optional playback request gated by a round-robin counter;
/// - class 2 runs a height/distance shaping stage and issues a shaped
///   playback request, or a fallback request when only the distance flag
///   is set;
/// - class 4 runs a height-gated request;
/// - anything else returns the class number.
///
/// Branch-for-branch equivalent to the original, including which running
/// value sits in EAX on every early exit. Float comparisons mirror the
/// original's comiss/NaN behavior exactly (never `f32::min`).
///
/// Original: thiscall/4 (this = owner, arg0 = voice, arg1 = slot hint,
/// arg2 = request record, arg3 = forwarded tag).
export!(thiscall, rw_00c8a850(this: u32, voice: *mut u8, hint: u32, req: *const u8, tag: u32) -> u32 {
    unsafe {
        let rf = |off: usize| f32::from_bits(*(req.add(off) as *const u32));
        let s60 = rf(0x60);
        let s64 = rf(0x64);
        let s68 = rf(0x68);
        let dl = s68 > 0.0 && s60 > s68;
        let cl = s64 == 0.0 && s68 > 0.0;
        let ch = s64 >= s60 && s60 > s68;
        let voice_w = *(voice.add(0x28) as *const u32);
        let class = (voice_w >> 6) & 0xF;
        let vtable = *(voice as *const u32);
        let hook_target = *((vtable as *const u8).add(0xEC) as *const u32);
        let hook: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(hook_target as usize);
        let one = *global::<f32>(0xFE88E8);
        // Clamp helper matching `comiss one,x; ja keep; else one`.
        let clamp1 = |x: f32| if one > x { x } else { one };
        if class == 3 {
            if !(dl || cl || ch) {
                return 3;
            }
            // Request distance: shaped by helper for live voices, else a
            // locally differenced length scaled by the global factor.
            let pa = *(voice.add(0x38) as *const u32);
            let d2 = if pa != 0 && pa == *(voice.add(0x7b4) as *const u32) {
                let mut out = [0u32; 3];
                callee_thiscall!(
                    1, u32, voice as u32, out.as_mut_ptr() as u32,
                    (req as u32).wrapping_add(0x20), 1,
                    *(req.add(0x6c) as *const u32)
                );
                let o0 = f32::from_bits(out[0]);
                let o1 = f32::from_bits(out[1]);
                let o2 = f32::from_bits(out[2]);
                ((o1 * o1 + o0 * o0) + o2 * o2).sqrt()
            } else {
                let dx = rf(0x20) - rf(0x10);
                let dy = rf(0x24) - rf(0x14);
                let dz = rf(0x28) - rf(0x18);
                ((dy * dy + dx * dx) + dz * dz).sqrt()
                    * *global::<f32>(0x11735C0)
            };
            let s6c = *(req.add(0x6c) as *const u32);
            let key: u32 = callee_cdecl!(2, u32, voice as u32, s6c);
            let mut index = 0u32;
            let mut hit = false;
            while index < 14 {
                if *global::<u32>(0x1050940 + index * 20) == key {
                    hit = true;
                    break;
                }
                index += 1;
            }
            if !hit {
                return 14;
            }
            let mut dirs = [0u32; 3];
            let rp = hook(voice as u32, dirs.as_mut_ptr() as u32);
            let r0 = f32::from_bits(*(rp as *const u32));
            let r1 = f32::from_bits(*((rp as *const u32).add(1)));
            let r2 = f32::from_bits(*((rp as *const u32).add(2)));
            let len_b = ((r0 * r0 + r1 * r1) + r2 * r2).sqrt();
            let off20 = index * 20;
            let is_live = *global::<u32>(0x1050940 + off20) == 0x36A0;
            let s70 = *(req.add(0x70) as *const u32);
            let gate_vol = *global::<f32>(0xEA42C8);
            if cl && *global::<u8>(0x1050950 + off20) != 0 && d2 >= *global::<f32>(0xEA42AC) {
                let t = clamp1((d2 - *global::<f32>(0xEA42AC)) * *global::<f32>(0xFE87D0));
                callee_thiscall!(
                    3, u32, (voice as u32).wrapping_add(0x3c0), d2.to_bits()
                );
                let r: u32 = callee_thiscall!(
                    4, u32, relocated(0x13B0EB0), voice as u32, s70, s6c,
                    t.to_bits(), is_live as u32, tag
                );
                if !(gate_vol > len_b) {
                    return r;
                }
                return play_mixer(
                    voice as u32, req as u32, key, off20, 0.0,
                    *global::<f32>(0x105094C + off20),
                );
            }
            if ch && *global::<u8>(0x1050951 + off20) != 0 {
                let t = clamp1(d2 * *global::<f32>(0xFE87D0));
                callee_thiscall!(
                    3, u32, (voice as u32).wrapping_add(0x3c0), d2.to_bits()
                );
                let r: u32 = callee_thiscall!(
                    5, u32, relocated(0x13B0EB0), voice as u32, s70, s6c,
                    t.to_bits(), is_live as u32
                );
                if !(gate_vol > len_b) {
                    return r;
                }
                return play_mixer(
                    voice as u32, req as u32, key, off20, 0.0,
                    *global::<f32>(0x105094C + off20),
                );
            }
            if !dl || *global::<u8>(0x1050952 + off20) == 0 {
                // EAX still holds the table offset from the scan above.
                return off20;
            }
            let g: u32 = callee_thiscall!(6, u32, voice as u32);
            if g & 0xFF != 0 {
                return g;
            }
            let t3 = clamp1(d2 * *global::<f32>(0xFE87D0));
            let tab4 = *global::<u32>(0x1050944 + off20);
            let _r7: u32 = callee_thiscall!(
                7, u32, relocated(0x13B0EB0), voice as u32, s70, s6c, tab4,
                t3.to_bits(), is_live as u32, hint
            );
            // Round-robin counter from the hook length.
            let (count, fade) = if gate_vol >= len_b {
                (*global::<u32>(0xEA42E0), 0.0f32)
            } else if len_b >= *global::<f32>(0xEA42D4) {
                (*global::<u32>(0xEA42EC), one)
            } else {
                let lo = gate_vol;
                let hi = *global::<f32>(0xEA42D4);
                let f = (len_b - lo) / (hi - lo);
                let span = (*global::<u32>(0xEA42EC))
                    .wrapping_sub(*global::<u32>(0xEA42E0)) as i32;
                let step = if f.is_nan() {
                    i32::MIN
                } else {
                    (span as f32 * f) as i32
                };
                let n = (step as u32).wrapping_add(*global::<u32>(0xEA42E0));
                (n, f)
            };
            let word = *(voice.add(0x2c) as *const u16) as u32;
            let n = word
                .wrapping_add(*global::<u32>(0x1173604))
                .wrapping_add(index);
            if n % count != 0 {
                return n / count;
            }
            play_mixer(
                voice as u32, req as u32, key, off20, fade,
                *global::<f32>(0x105094C + off20),
            )
        } else if class == 2 {
            if !(dl || cl) {
                return 2;
            }
            if *((this as *const u8).add(8) as *const u16) == 9 && hint == 4 {
                return this;
            }
            if cl {
                let mut dirs = [0u32; 3];
                let rp = hook(voice as u32, dirs.as_mut_ptr() as u32);
                let up = -f32::from_bits(*((rp as *const u32).add(2)));
                if !(up > *global::<f32>(0xEA42B8)) {
                    return rp;
                }
                let mut dirs2 = [0u32; 3];
                let rp2 = hook(voice as u32, dirs2.as_mut_ptr() as u32);
                let q0 = f32::from_bits(*(rp2 as *const u32));
                let q1 = f32::from_bits(*((rp2 as *const u32).add(1)));
                let q2 = f32::from_bits(*((rp2 as *const u32).add(2)));
                let len = ((q0 * q0 + q1 * q1) + q2 * q2).sqrt();
                let span = *global::<f32>(0xEA42BC) - *global::<f32>(0xEA42B8);
                let t = clamp1((up - *global::<f32>(0xEA42B8)) / span);
                let q = clamp1(len / *global::<f32>(0x1050A70));
                let e20 = (req as u32).wrapping_add(0x20);
                callee_thiscall!(
                    9, u32, relocated(0x13B0EB0), voice as u32, e20, t.to_bits()
                );
                callee_thiscall!(
                    10, u32, (voice as u32).wrapping_add(0x210), e20,
                    (req as u32).wrapping_add(0x30),
                    (req as u32).wrapping_add(0x40), q.to_bits()
                )
            } else {
                let e20 = (req as u32).wrapping_add(0x20);
                let v = (rf(0x28) - s60 * *global::<f32>(0xFE8830)) + s68;
                let mut frame = [rf(0x20).to_bits(), rf(0x24).to_bits(), v.to_bits(), 0u32];
                callee_thiscall!(
                    11, u32, relocated(0x13B0EB0), voice as u32,
                    frame.as_mut_ptr() as u32, e20, hint, s60.to_bits()
                )
            }
        } else if class == 4 {
            if !cl {
                return 4;
            }
            let mut dirs = [0u32; 3];
            let rp = hook(voice as u32, dirs.as_mut_ptr() as u32);
            let up = -f32::from_bits(*((rp as *const u32).add(2)));
            if !(up > *global::<f32>(0xEA42C0)) {
                return rp;
            }
            let span = *global::<f32>(0xEA42C4) - *global::<f32>(0xEA42C0);
            let t = clamp1((up - *global::<f32>(0xEA42C0)) / span);
            callee_thiscall!(
                12, u32, relocated(0x13B0EB0), voice as u32,
                (req as u32).wrapping_add(0x20), t.to_bits()
            )
        } else {
            class
        }
    }
});

/// Shared playback request at the end of the class-3 path.
unsafe fn play_mixer(
    voice: u32, req: u32, key: u32, off20: u32, fade: f32, tabf: f32,
) -> u32 {
    let voice_b = voice as *const u8;
    let v20 = *(voice_b.add(0x20) as *const u32);
    let s70 = *((req as *const u8).add(0x70) as *const u32);
    callee_thiscall!(
        8, u32, relocated(0x13B0EF0), voice, s70.wrapping_add(0x30),
        v20.wrapping_add(0x10), tabf.to_bits(), fade.to_bits(), 0, 0, key
    )
}
