// original: 0x009A08A0 audio_voice_update (proposed name)
/// Per-voice audio update: picks two mix parameters from a mode byte and a
/// float table, resolves the voice's output slots through engine calls, and
/// programs them.
///
/// `this` points at the voice object; `a0..a2` are engine handles forwarded
/// to the slot resolvers, `a3`/`a4` are small selectors (only the low byte of
/// each is read), `a5` is stored into the frame for a later stage. Returns the
/// engine's answer for the programmed voice, or an early-out value when the
/// voice or the global audio state says there is nothing to do.
export!(thiscall, rb53_fn1(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    #[inline(always)]
    fn g32(file_va: u32) -> u32 {
        unsafe { *(lf_k2_rt::global::<u32>(file_va) as *const u32) }
    }

    #[inline(always)]
    fn gf(file_va: u32) -> f32 {
        unsafe { *(lf_k2_rt::global::<f32>(file_va) as *const f32) }
    }

    #[inline(always)]
    fn g8(file_va: u32) -> u8 {
        unsafe { *(lf_k2_rt::global::<u8>(file_va) as *const u8) }
    }

    #[inline(always)]
    fn h32(base: u32, off: u32) -> u32 {
        unsafe { *(base.wrapping_add(off) as *const u32) }
    }

    #[inline(always)]
    fn wh32(base: u32, off: u32, v: u32) {
        unsafe { *(base.wrapping_add(off) as *mut u32) = v; }
    }

    #[inline(always)]
    fn h8(base: u32, off: u32) -> u8 {
        unsafe { *(base.wrapping_add(off) as *const u8) }
    }

    #[inline(always)]
    fn wh8(base: u32, off: u32, v: u8) {
        unsafe { *(base.wrapping_add(off) as *mut u8) = v; }
    }

    #[inline(always)]
    fn hf(base: u32, off: u32) -> f32 {
        unsafe { *(base.wrapping_add(off) as *const f32) }
    }

    /// Scratch frame mirroring the original's stack frame. Offsets are the
    /// original's (relative to its resting esp); only relative positions and
    /// pointed-to contents are observable through the checker.
    struct Frame([u8; 0xB0]);

    impl Frame {
        fn zeroed() -> Frame {
            Frame([0u8; 0xB0])
        }
        #[inline(always)]
        fn r32(&self, o: usize) -> u32 {
            u32::from_le_bytes([self.0[o], self.0[o + 1], self.0[o + 2], self.0[o + 3]])
        }
        #[inline(always)]
        fn w32(&mut self, o: usize, v: u32) {
            let b = v.to_le_bytes();
            self.0[o..o + 4].copy_from_slice(&b);
        }
        #[inline(always)]
        fn rf(&self, o: usize) -> f32 {
            f32::from_bits(self.r32(o))
        }
        #[inline(always)]
        fn wf(&mut self, o: usize, v: f32) {
            self.w32(o, v.to_bits());
        }
        #[inline(always)]
        fn r8(&self, o: usize) -> u8 {
            self.0[o]
        }
        #[inline(always)]
        fn w8(&mut self, o: usize, v: u8) {
            self.0[o] = v;
        }
        #[inline(always)]
        fn ptr(&mut self, o: usize) -> u32 {
            self.0[o..].as_mut_ptr() as u32
        }
    }

    let mut fr = Frame::zeroed();
    // Global state gates: an enabled flag, a generation check, and the
    // audio-mode word. Any mismatch returns without touching the voice.
    if g32(0x11f7060) != 1 {
        let gen = g32(0x12088b4);
        if gen == g32(0xf1c040) && g32(0x1037720) != 0x12 {
            return gen;
        }
    }
    // Slot-waiting voices (slot 0x60 already filled) are skipped.
    if h32(this, 0x60) != 0 {
        return g32(0x12088b4);
    }
    // Mode byte: the global word wins when non-negative, else the argument.
    let gw = g32(0x1038d44);
    let mode: u32 = if (gw as i32) >= 0 { gw & 0xff } else { a4 & 0xff };
    fr.w32(0x20, mode);
    fr.w32(0x1c, this);
    // Resolve the four voice-table handles.
    let mut i: u32 = 0;
    while i < 0x10 {
        let ans: u32 = callee_cdecl!(1, u32, g32(0x1038e04u32.wrapping_add(i)), 0);
        wh32(this, 0x70u32.wrapping_add(i), ans);
        wh32(this, 0x80u32.wrapping_add(i), 0xffff_ffff);
        i = i.wrapping_add(4);
    }
    fr.wf(0x18, gf(0xfe88e8));
    wh8(this, 0x209, 0);
    fr.wf(0x10, gf(0x12831d8));
    let ready: u32 = callee_thiscall!(2, u32, lf_k2_rt::relocated(0x128e400));
    let flag17: u8 = if (ready & 0xff) != 0 {
        1
    } else if g8(0x1283049) == (ready as u8) {
        0
    } else {
        1
    };
    fr.w8(0x17, flag17);
    let tmp = fr.ptr(0x68);
    let _: u32 = callee_thiscall!(3, u32, tmp);
    fr.w32(0x84, g32(0x1284398));
    wh32(this, 0x1f0, 0);
    fr.w8(0x28, 0);
    fr.w8(0x1c, 0);
    let m = (mode & 0xff) as u8;
    // Mode switch over the two mix parameters f10/f18.
    if m == 3 || flag17 != 0 {
        fr.wf(0x10, gf(0x1038d7c));
        fr.wf(0x18, gf(0x1038d80));
    } else if m == 2 {
        let mut x1 = gf(0x1038d9c) * fr.rf(0x10);
        fr.w8(0x28, 1);
        x1 += gf(0x1038d7c);
        fr.wf(0x10, x1);
        fr.wf(0x18, gf(0x1038d80));
    } else if m == 1 {
        fr.wf(0x10, gf(0x1038d9c) * fr.rf(0x10) + gf(0x1038d84));
    } else {
        let child = h32(this, 8);
        let mut alt = false;
        if child != 0 {
            let t: u32 = callee_thiscall!(4, u32, this, child);
            if (t & 0xff) != 0 {
                alt = true;
            }
        }
        if alt {
            fr.wf(0x10, gf(0x1038d9c) * fr.rf(0x10) + gf(0x1038d84));
        } else if m == 6 {
            let mut x1 = gf(0x1038d9c) * fr.rf(0x10);
            fr.w8(0x28, 1);
            fr.w8(0x1c, 1);
            x1 += gf(0x1038d94);
            fr.wf(0x10, x1);
            fr.wf(0x18, gf(0x1038d80));
        } else if m == 4 {
            fr.wf(0x10, gf(0x1038d74));
            fr.wf(0x18, gf(0x1038d8c));
        } else if m == 5 {
            fr.wf(0x10, gf(0x1038d88));
        } else {
            let x3 = fr.rf(0x10);
            let mut x2 = gf(0xfe88e8) - x3;
            let mut x1 = gf(0x1038d9c) * x3;
            let x0 = gf(0x1038da4) * x3;
            x2 *= gf(0x1038d78);
            x1 += gf(0x1038d74);
            x2 += x0;
            fr.wf(0x10, x1);
            fr.wf(0x18, x2);
        }
    }
    // Resolve the primary output slot.
    let child = h32(this, 8);
    fr.0[0xae] &= 0xdf;
    let mut slot = g32(0x128456c);
    fr.w32(0x24, slot);
    if child != 0 {
        let t: u32 = callee_thiscall!(5, u32, this, child);
        if (t & 0xff) == 0 && (mode & 0xff) != 1 {
            slot = g32(0x1284570);
            fr.w32(0x24, slot);
        }
    }
    let a: u32 = callee_thiscall!(6, u32, this, fr.r32(0x18));
    fr.w32(0x84, a);
    fr.w32(0x8c, a5);
    // Clamp the first mix parameter at zero (NaN clamps too).
    let f10 = fr.rf(0x10);
    if !(0.0f32 > f10) {
        fr.wf(0x10, 0.0);
    }
    let p68 = fr.ptr(0x68);
    let field60 = this.wrapping_add(0x60);
    let arg0 = fr.r32(0x24);
    let _: u32 = callee_thiscall!(7, u32, this, arg0, field60, p68, 0xffff_ffff, 0, 0);
    let obj = h32(field60, 0);
    if obj != 0 {
        let b4 = h8(obj, 4);
        let picked: u32 = if b4 == 0xff {
            0
        } else {
            let mult = g32(0x115d968);
            let base = g32(0x115d988);
            let stride = (h8(obj, 0x40) as u32).wrapping_mul(0x6f40);
            let entry = h32(base.wrapping_add(stride).wrapping_add(0x6f14), 0);
            (b4 as u32).wrapping_mul(mult).wrapping_add(entry)
        };
        let _: u32 = callee_thiscall!(8, u32, picked, fr.r32(0x10));
    }
    let obj = h32(field60, 0);
    if obj == 0 {
        // eax still holds the reloaded (null) slot word here.
        return obj;
    }
    // Program the slot object both resolvers agree on.
    if h8(obj, 0x3b) == 4 {
        let t: u32 = callee_thiscall!(9, u32, obj, a0, a1, a2, 0);
        if (t & 0xff) == 0 {
            let obj2 = h32(field60, 0);
            let r: u32 = callee_thiscall!(10, u32, obj2, 0);
            return r;
        }
    } else {
        let r: u32 = callee_thiscall!(10, u32, obj, 0);
        return r;
    }
    wh8(this, 0x188, h8(this, 0x188) | 2);
    wh8(this, 0x1a4, h8(this, 0x1a4) | 2);
    let p50 = fr.ptr(0x50);
    let p20 = fr.ptr(0x20);
    let p18 = fr.ptr(0x18);
    fr.w32(0x18, 0);
    fr.w32(0x20, 0x46abe000);
    let _: u32 = callee_thiscall!(11, u32, this, p18, p20, p50);
    let obj = h32(field60, 0);
    let _: u32 = callee_thiscall!(12, u32, obj, fr.r32(0x20));
    let f = fr.rf(0x18) + hf(this, 0x1f0);
    let obj = h32(field60, 0);
    let _: u32 = callee_thiscall!(13, u32, obj, f.to_bits());
    let obj = h32(field60, 0);
    let p50b = fr.ptr(0x50);
    let _: u32 = callee_thiscall!(14, u32, obj, p50b);
    if fr.r32(0x24) != g32(0x1284570) {
        // Tail: pick the mixer voice and program both slots.
        let sel = ((a3 as u8) as i8) as i32 as u32;
        let mut mix: u32 = callee_cdecl!(25, u32, sel);
        if mix == 0 {
            mix = callee_cdecl!(26, u32, g32(0x1038d10));
        }
        fr.w32(0x1c, mix);
        let obj = h32(field60, 0);
        let _: u32 = callee_thiscall!(27, u32, obj, mix, 1, 0xffff_ffff);
        let o64 = h32(this, 0x64);
        if o64 != 0 {
            let _: u32 = callee_thiscall!(28, u32, o64, fr.r32(0x1c), 1, 0xffff_ffff);
        }
        let h: u32 = callee_cdecl!(29, u32, a1, 0);
        wh32(this, 0x90, a2);
        wh32(this, 0x68, h);
        let r: u32 = callee_thiscall!(30, u32, lf_k2_rt::relocated(0x1284a60), a0, h, a2);
        return r;
    }
    // Secondary-voice path.
    let v1c = fr.r32(0x1c);
    let p24 = fr.ptr(0x24);
    let p1f4 = this.wrapping_add(0x1f4);
    let v28 = fr.r32(0x28);
    // These two stores execute before the call (attempt-5 fix).
    fr.w32(0x24, 0);
    fr.w32(0x30, p1f4);
    let _: u32 = callee_thiscall!(15, u32, this, v28, p1f4, p24, v1c);
    let p68b = fr.ptr(0x68);
    let f64 = this.wrapping_add(0x64);
    let _: u32 = callee_thiscall!(16, u32, this, g32(0x12844e4), f64, p68b, 0xffff_ffff, 0, 0);
    fr.w32(0x90, 0);
    let o64 = h32(this, 0x64);
    if o64 == 0 {
        let sel = ((a3 as u8) as i8) as i32 as u32;
        let mut mix: u32 = callee_cdecl!(25, u32, sel);
        if mix == 0 {
            mix = callee_cdecl!(26, u32, g32(0x1038d10));
        }
        fr.w32(0x1c, mix);
        let obj = h32(field60, 0);
        let _: u32 = callee_thiscall!(27, u32, obj, mix, 1, 0xffff_ffff);
        let o64b = h32(this, 0x64);
        if o64b != 0 {
            let _: u32 = callee_thiscall!(28, u32, o64b, fr.r32(0x1c), 1, 0xffff_ffff);
        }
        let h: u32 = callee_cdecl!(29, u32, a1, 0);
        wh32(this, 0x90, a2);
        wh32(this, 0x68, h);
        let r: u32 = callee_thiscall!(30, u32, lf_k2_rt::relocated(0x1284a60), a0, h, a2);
        return r;
    }
    let _: u32 = callee_thiscall!(17, u32, o64, fr.r32(0x10));
    let o64 = h32(this, 0x64);
    let t: u32 = callee_thiscall!(18, u32, o64, a0, a1, a2, 0);
    if (t & 0xff) == 0 {
        let o64 = h32(this, 0x64);
        let _: u32 = callee_thiscall!(24, u32, o64, 0);
    } else {
        let o64 = h32(this, 0x64);
        let _: u32 = callee_thiscall!(19, u32, o64, fr.r32(0x20));
        let o64 = h32(this, 0x64);
        let e = fr.r32(0x30);
        let f = fr.rf(0x18) + hf(this, 0x1f0) + hf(e, 0);
        let _: u32 = callee_thiscall!(20, u32, o64, f.to_bits());
        let p: u32 = callee_thiscall!(21, u32, lf_k2_rt::relocated(0x115def0), 0);
        let f1 = unsafe { *(p as *const f32) };
        let f2 = unsafe { *((p.wrapping_add(4)) as *const f32) };
        let f3 = unsafe { *((p.wrapping_add(8)) as *const f32) };
        // The two argument pushes shift every slot down by 8 (attempt-5 fix).
        fr.wf(0x30, f1);
        fr.wf(0x40, fr.rf(0x50) - f1);
        fr.w32(0x1c, f2.to_bits());
        fr.w32(0x28, f3.to_bits());
        fr.wf(0x44, fr.rf(0x54) - f2);
        fr.wf(0x48, fr.rf(0x58) - f3);
        let _: u32 = callee_stdcall!(22, u32, gf(0x1038de0).to_bits(), 0x7a);
        // The argument push shifts these by 4 (attempt-5 fix).
        fr.wf(0x40, fr.rf(0x40) + fr.rf(0x30));
        fr.wf(0x44, fr.rf(0x44) + fr.rf(0x1c));
        fr.wf(0x48, fr.rf(0x48) + fr.rf(0x28));
        let o64 = h32(this, 0x64);
        let p40 = fr.ptr(0x40);
        let _: u32 = callee_thiscall!(23, u32, o64, p40);
    }
    // Tail: pick the mixer voice and program both slots.
    let sel = ((a3 as u8) as i8) as i32 as u32;
    let mut mix: u32 = callee_cdecl!(25, u32, sel);
    if mix == 0 {
        mix = callee_cdecl!(26, u32, g32(0x1038d10));
    }
    fr.w32(0x1c, mix);
    let obj = h32(field60, 0);
    let _: u32 = callee_thiscall!(27, u32, obj, mix, 1, 0xffff_ffff);
    let o64 = h32(this, 0x64);
    if o64 != 0 {
        let _: u32 = callee_thiscall!(28, u32, o64, fr.r32(0x1c), 1, 0xffff_ffff);
    }
    let h: u32 = callee_cdecl!(29, u32, a1, 0);
    wh32(this, 0x90, a2);
    wh32(this, 0x68, h);
    let r: u32 = callee_thiscall!(30, u32, lf_k2_rt::relocated(0x1284a60), a0, h, a2);
    r
});
