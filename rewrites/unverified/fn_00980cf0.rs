// original: 0x00980CF0 audio_emitter_update
/// Update one audio emitter from the listener position: gates on the audio
/// state, measures distance, scales through the filter chain, looks up the
/// voice table twice, converts to integer gain and posts the voice update.
///
/// Behaviour: returns early unless the audio system is running, the mix
/// generation matches and the emitter has a voice bank; otherwise reads the
/// listener position through an out-parameter call, computes
/// `clamp(dist * k1 [* 0.5] / 6, 1.0) / rate`, pushes it through four
/// single-precision filter stages, resolves the voice id from a byte-indexed
/// table, scales by log-domain constants into an integer gain and posts the
/// update. The emitter's stored position is refreshed from the filter output
/// and the measured position.
export!(thiscall, rw_rb52_980cf0(this: *mut u8) -> u32 {
    unsafe {
        // Gate checks in the original's order; any failure skips to the end.
        let gated = *global::<u32>(0x11F7060) == 1
            || *global::<u32>(0x12088B4) != *global::<u32>(0xF1C040)
            || *global::<u32>(0x1037720) == 0x12
            || *(this.add(0xC) as *const u32) == 0;
        if !gated {
            // Listener position arrives through an out-parameter call.
            let mut pos = [0u32; 3];
            callee_thiscall!(1, u32, this as u32, pos.as_mut_ptr() as u32);
            let px = f32::from_bits(pos[0]);
            let py = f32::from_bits(pos[1]);
            let pz = f32::from_bits(pos[2]);
            // Squared distance; addition order matches the original.
            let dx = px - *(this.add(0x60) as *const f32);
            let dy = py - *(this.add(0x64) as *const f32);
            let dz = pz - *(this.add(0x68) as *const f32);
            let d2 = (dy * dy + dx * dx) + dz * dz;
            let mut v = d2.sqrt() * *global::<f32>(0x11735C0);
            if *this.add(0x12A) != 0 {
                v *= *global::<f32>(0xFE8830);
            }
            v /= *global::<f32>(0x103896C);
            let ceiling = *global::<f32>(0xFE88E8);
            // Ordered-less-than keeps NaN on the ceiling side, as comiss+ja.
            let clamped = if v < ceiling { v } else { ceiling };
            // Integer rate to float through the game's conversion table.
            let rate = *global::<u32>(0x1038970);
            let adjust = if rate >> 31 == 0 {
                *global::<f64>(0xFE8F50)
            } else {
                *global::<f64>(0xFE8F58)
            };
            let rate_f = ((rate as i32) as f64 + adjust) as f32;
            // The clamp result only reaches memory; the register still holds
            // the ceiling, so this quotient uses the ceiling, not the clamp.
            let scaled = ceiling / rate_f;
            let voice = (this as u32).wrapping_add(0x44);
            callee_thiscall!(2, u32, voice, scaled.to_bits(), scaled.to_bits());
            let mix_tag = *global::<u32>(0x11735B4);
            let stage1: u32 =
                callee_thiscall!(3, u32, voice, clamped.to_bits(), mix_tag);
            let f1 = f32::from_bits(stage1);
            // Pick the filter bank by the voice-class flag.
            let class_obj = *(this.add(0x120) as *const u32);
            let flag = *((class_obj as *const u8).add(0x219));
            let (first_bank, second_bank) = if flag == 0 {
                (relocated(0x12313F0), relocated(0x12313C8))
            } else {
                (relocated(0x12314A4), relocated(0x1231424))
            };
            let stage2a: u32 = callee_thiscall!(4, u32, first_bank, f1.to_bits());
            let stage2b: u32 = callee_thiscall!(4, u32, second_bank, f1.to_bits());
            let f2a = f32::from_bits(stage2a);
            let _f2b = f32::from_bits(stage2b);
            let stage3: u32 = callee_thiscall!(5, u32, second_bank, f2a.to_bits());
            let f3 = f32::from_bits(stage3);
            // Voice-table lookup shared by both postings.
            let bank = *(this.add(0xC) as *const u32);
            let mult = *global::<u32>(0x115D968);
            let table = *global::<u32>(0x115D988);
            let lookup = || {
                let kind = *((bank as *const u8).add(4));
                if kind == 0xFF {
                    0
                } else {
                    let row = *((bank as *const u8).add(0x40)) as u32;
                    let entry = *((table
                        .wrapping_add(row.wrapping_mul(0x6F40))
                        .wrapping_add(0x6F14))
                        as *const u32);
                    mult.wrapping_mul(kind as u32).wrapping_add(entry)
                }
            };
            callee_thiscall!(6, u32, lookup(), f3.to_bits());
            // Log-domain gain conversion through the vector-float helper.
            let gain_bits: u32 = callee_cdecl!(7, u32, f2a.to_bits());
            let mut gain = f32::from_bits(gain_bits);
            gain *= *global::<f32>(0xE78588);
            gain *= *global::<f32>(0xE7858C);
            // Truncate toward zero with cvttss2si's out-of-range value.
            let gain_i =
                if gain.is_nan() || gain >= 2147483648.0 { i32::MIN } else { gain as i32 };
            callee_thiscall!(8, u32, lookup(), gain_i as u32);
            // Post the voice update; the scratch block starts zeroed.
            let mut block = [0u8; 76];
            callee_cdecl!(9, u32, 0x4C);
            (block.as_mut_ptr().add(0xC) as *mut u32).write_unaligned(f2a.to_bits());
            (block.as_mut_ptr().add(0x1C) as *mut u32)
                .write_unaligned(gain_i as u32);
            *block.as_mut_ptr().add(0x2D) |= 0x0A;
            let handle = *((bank as *const u8).add(0xA4) as *const u32);
            callee_cdecl!(10, u32, handle, block.as_mut_ptr() as u32);
            // Refresh the stored position from the filter output, the
            // measured position and the (never written) scratch slot.
            *(this.add(0x60) as *mut u32) = f3.to_bits();
            *(this.add(0x64) as *mut u32) = pos[1];
            *(this.add(0x68) as *mut u32) = pos[2];
            *(this.add(0x6C) as *mut u32) = 0;
        }
        // Stack-cookie check; each side's cookie is self-consistent.
        callee_stdcall!(11, u32,);
        0
    }
});
