// original: 0x00aceca0 audio_voice_update
// 0x00ACECA0: audio voice update (proposed name: audio_voice_update).
// The per-frame update for one voice. Bails out early unless the window
// is iconic-state gated (callee 1, USER32!IsIconic on the window handle
// global) combined with two flag bytes, a second enable call (callee 2)
// and voice-state words at arg0+0x1304/+0x28. Otherwise it picks a base
// rate (1.0, or 4.0 when callee 3 resolves arg0's owner back to arg0),
// compares callee 4's measured double against base*3600, and, when bit 1
// of this+0x164 is set, runs the positional check (callee 5, a 10-word
// cdecl call with an out-word), derives a mix factor clamped at zero
// (zeroed when bit 27 of arg0+0x24 or the mute flag byte is set), and
// fires the stage calls (callees 6-8, the last being 0xACE640). Then it
// conditionally runs the setup (callee 9, 0xACEF70) and release
// (callee 10) tail calls and counts down the timer at this+0x168.
export!(thiscall, rw_00aceca0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        // Gate A: iconic state plus two flag bytes plus two OR bytes.
        let iconic: u32 = callee_stdcall!(1, u32, *global::<u32>(0x017ACCD8));
        let mut code: u8 = if iconic != 0 {
            1
        } else {
            ((*global::<u8>(0x0105B48F) != 0) && (*global::<u8>(0x017ED8D1) != 0)) as u8
        };
        code |= *global::<u8>(0x01173590);
        code |= *global::<u8>(0x01173591);
        if code != 0 {
            return 0;
        }
        // Gate B: second enable call.
        let enabled: u32 = callee_cdecl!(2, u32,);
        if (enabled as u8) != 0 {
            return 0;
        }
        // Gates C/D: voice-state words.
        let state = *((arg0 + 0x1304) as *const u32);
        if state == 2 || state == 4 {
            return 0;
        }
        if *((arg0 + 0x28) as *const u32) & 0x7C00 == 0xC00 {
            return 0;
        }
        // Base rate: 4.0 when the owner resolves back to arg0.
        let mut base = *global::<f32>(0x00FE88E8);
        let owner: u32 = callee_cdecl!(3, u32,);
        if owner != 0 {
            let mid = *((owner.wrapping_add(0x598)) as *const u32);
            if *((mid.wrapping_add(0xB30)) as *const u32) == arg0 {
                base = *global::<f32>(0x00FE8AB8);
            }
        }
        // Measured value against base*3600. The original stores the
        // callee's float into its own incoming arg0 slot; the rewrite
        // keeps it in a local (contract: stack not compared, q-13
        // precedent, same as fn3's out-word slot).
        let measured: f64 = callee_cdecl!(4, f64, this.wrapping_add(0xA0));
        let got = measured as f32;
        let limit = base * *global::<f32>(0x00FE8C88);
        if got > limit {
            return 0;
        }
        let flags = *((this + 0x164) as *const u32);
        if ((flags >> 1) & 1) != 0 {
            let f_a0 = *((this + 0xA0) as *const u32);
            let f_a4 = *((this + 0xA4) as *const u32);
            let f_a8 = *((this + 0xA8) as *const u32);
            // The out-word lands in this function's own frame (below the
            // incoming stack pointer, invisible to the stack channel);
            // the original zeroes the slot first, so does the rewrite.
            // NOTE: the stub logs words 0..7 of the 10; words 8-9 are
            // the constant pushes 20.0 and 0, passed faithfully.
            let mut slot = 0u32;
            let ok: u32 = callee_cdecl!(
                5, u32, f_a0, f_a4, f_a8, &mut slot as *mut u32 as u32, 1, 0, 0,
                0x40C00000, 0x41A00000, 0
            );
            let mut cl: u8 = 0;
            if (ok as u8) != 0 {
                let w = f32::from_bits(slot);
                let r = f32::from_bits(f_a8);
                if w > r && *global::<f32>(0x00FE8830) > w - r {
                    *((this + 0x164) as *mut u32) = flags | 0x2000000;
                    cl = 1;
                } else {
                    *((this + 0x164) as *mut u32) = flags & 0xFDFFFFFF;
                }
            } else {
                *((this + 0x164) as *mut u32) = flags & 0xFDFFFFFF;
            }
            // Mix factor, clamped at zero (NaN-safe: `0.0 > mix` keeps NaN).
            let e = *((this + 0x8C) as *const f32);
            let lo = *global::<f32>(0x00ECA814);
            let mut div = 0.0f32;
            if e > lo {
                div = (e - lo) / (*global::<f32>(0x00ECA818) - lo);
            }
            let mut mix = *global::<f32>(0x012DDE98) - div;
            if 0.0 > mix {
                mix = 0.0;
            }
            if (((*((arg0 + 0x24) as *const u32) >> 27) & 1) != 0)
                || *global::<u8>(0x015DBE01) != 0
            {
                mix = 0.0;
            }
            if cl == 0 {
                callee_thiscall!(6, u32, this, arg0, mix.to_bits(), arg1);
            }
            // The original reloads its frame byte-slot as a dword; the
            // slot's upper bytes are the defined zero fill, so this is cl.
            // Its incoming arg0 slot is reused: the mix factor overwrites
            // the measured float there, so call 8 takes mix, not got.
            let esi2 = cl as u32;
            callee_thiscall!(7, u32, this, arg0, mix.to_bits(), esi2, arg1);
            callee_thiscall!(8, u32, this, arg0, mix.to_bits(), esi2, base.to_bits());
        }
        // Tail: setup call unless bit 3 is clear and energy is below 70
        // (bare `jb`-skip, so NaN skips: `>=` does nothing on NaN).
        let tail_flags = *((this + 0x164) as *const u32);
        if ((tail_flags >> 3) & 1) != 0 || *((this + 0x8C) as *const f32) >= 70.0 {
            callee_thiscall!(9, u32, this, arg0);
        }
        if *((this + 0xD4) as *const u32) != 0 {
            callee_thiscall!(10, u32, this, arg0);
        } else {
            *((this + 0x164) as *mut u32) &= 0xBFFFFFFF;
        }
        // Countdown at this+0x168. The second test is a bare `jb`-exit,
        // taken also when unordered: `!(0.0 >= next)`, not `0.0 < next`.
        let cd = *((this + 0x168) as *const f32);
        if cd > 0.0 {
            let next = cd - *global::<f32>(0x011735BC);
            *((this + 0x168) as *mut f32) = next;
            if !(0.0 >= next) {
                return 0;
            }
            *((this + 0x164) as *mut u32) &= 0xFFFFFFCF;
        }
        0
    }
});
