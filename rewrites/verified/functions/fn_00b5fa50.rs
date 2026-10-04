// original: 0x00B5FA50 audio_voice_update
/// Audio voice update (original 0x00B5FA50): validates a voice object looked
/// up from the parameter block, runs position/attenuation helpers, then hands
/// the voice to the mixer tail call. Returns the mixer's double result, or
/// +0.0 on any early exit. `this` is only passed through to the tail call.
export!(thiscall, rw_00b5fa50(this: u32, p1: u32, p2: u32, p3: u32, _p4: u32) -> f64 {
    unsafe {
        // Look up the voice object from the parameter block's handle.
        let edi: u32 = callee_cdecl!(1, u32, (p3 as *const u32).read_unaligned());
        // Voice state nibble must be >= 2.
        if (edi as *const u8).add(0x1e2).read_unaligned() & 0x0f < 2 {
            return 0.0;
        }
        let esi = (edi as *const u32).add(0x1bc / 4).read_unaligned();
        if esi == 0 {
            return 0.0;
        }
        // Channel flags must select mode 0xC0.
        if (esi as *const u32).add(0x28 / 4).read_unaligned() & 0x3c0 != 0xc0 {
            return 0.0;
        }
        let gate = (esi as *const u32).add(0x6c / 4).read_unaligned();
        if gate != 0 && (gate as *const u8).add(0x0e).read_unaligned() != 0 {
            return 0.0;
        }
        // Virtual validation hook on the channel (slot 0x28 words in).
        let vt = (esi as *const u32).read_unaligned();
        let tgt = ((vt as *const u8).add(0x128) as *const u32).read_unaligned();
        let validate: extern "thiscall" fn(u32) -> u8 =
            core::mem::transmute(tgt as usize);
        if validate(esi) != 0 {
            return 0.0;
        }
        // Listener flag: set when p1 is a live channel whose hook fires.
        let mut flag: u8 = 0;
        if p1 != 0 && (p1 as *const u32).add(0x28 / 4).read_unaligned() & 0x3c0 == 0xc0 {
            let vt1 = (p1 as *const u32).read_unaligned();
            let t1 = ((vt1 as *const u8).add(0x128) as *const u32).read_unaligned();
            let hook: extern "thiscall" fn(u32) -> u8 =
                core::mem::transmute(t1 as usize);
            if hook(p1) != 0 {
                flag = 1;
            }
        }
        // Relative position vector; stays zero when the block is skipped
        // (matches the checker's defined zero fill for untouched frame slots).
        let mut rel = [0.0f32; 3];
        if (esi as *const u8).add(0xa60).read_unaligned() != 2 && flag != 0 {
            let q = (edi as *const u32).add(0x20 / 4).read_unaligned();
            rel[0] = f32::from_bits((p3 as *const u32).add(0x10 / 4).read_unaligned())
                - f32::from_bits((q as *const u32).add(0x30 / 4).read_unaligned());
            rel[1] = f32::from_bits((p3 as *const u32).add(0x14 / 4).read_unaligned())
                - f32::from_bits((q as *const u32).add(0x34 / 4).read_unaligned());
            rel[2] = f32::from_bits((p3 as *const u32).add(0x18 / 4).read_unaligned())
                - f32::from_bits((q as *const u32).add(0x38 / 4).read_unaligned());
            let hit: u32 = callee_thiscall!(4, u32, p1.wrapping_add(0x2b0));
            if hit != 0 {
                let key = ((hit as *const u32).add(0x18 / 4)).read_unaligned();
                let a1: u32 = callee_cdecl!(5, u32, key);
                if a1 != 0 {
                    let a2: u32 = callee_cdecl!(5, u32, key);
                    if (a2 as *const u32).read_unaligned() != 0 {
                        let ok: u8 = callee_thiscall!(6, u8,
                            esi.wrapping_add(0x2b0),
                            (esi as *const u32).add(0x2c4 / 4).read_unaligned());
                        if ok == 0 {
                            callee_cdecl!(7, u32, esi);
                        }
                    }
                }
            }
            // Timer-driven state bit.
            if (esi as *const u8).add(0x268).read_unaligned() & 8 == 0 {
                let n: u32 = callee_cdecl!(8, u32,);
                let limit = *global::<f32>(0x00fe87e8);
                let scaled = (n as i32) as f32 * *global::<f32>(0x00fe8684);
                // Original is `comiss; jbe`: fall through (set the bit) exactly
                // when limit > scaled, NaN-safe in the same way.
                if limit > scaled {
                    let w = (edi as *mut u32).add(0x210 / 4);
                    w.write_unaligned(w.read_unaligned() | 0x02000000);
                }
            }
            // Displacement vector between parameter and reference points.
            let mut d = [0.0f32; 3];
            d[0] = f32::from_bits((p3 as *const u32).add(0x10 / 4).read_unaligned())
                - f32::from_bits((p2 as *const u32).add(0x30 / 4).read_unaligned());
            d[1] = f32::from_bits((p3 as *const u32).add(0x14 / 4).read_unaligned())
                - f32::from_bits((p2 as *const u32).add(0x34 / 4).read_unaligned());
            d[2] = f32::from_bits((p3 as *const u32).add(0x18 / 4).read_unaligned())
                - f32::from_bits((p2 as *const u32).add(0x38 / 4).read_unaligned());
            callee_thiscall!(9, u32, d.as_ptr() as u32);
            // Attenuation sample through the render object's slot-9 hook.
            let r: u32 = callee_thiscall!(10, u32, edi);
            let vt4 = (r as *const u32).read_unaligned();
            let t4 = ((vt4 as *const u8).add(0x24) as *const u32).read_unaligned();
            let sample: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(t4 as usize);
            let k = sample(r) * *global::<f32>(0x01046a84);
            d[0] *= k;
            d[1] *= k;
            d[2] *= k;
            let w16 = (p3 as *const u16).add(0x52 / 2).read_unaligned() as u32;
            callee_thiscall!(12, u32, edi, d.as_ptr() as u32, rel.as_ptr() as u32, w16);
        }
        // Scratch cell filled by the allocator hook.
        let mut cell = [0u32; 1];
        callee_thiscall!(13, u32, cell.as_mut_ptr() as u32);
        // Source selector: live solver result or the channel default.
        let sel: u32 = if (edi as *const i16).add(0x1e0 / 2).read_unaligned() > 0
            && (esi as *const u8).add(0x29f).read_unaligned() & 1 != 0
        {
            callee_thiscall!(14, u32, esi,
                (edi as *const u16).add(0x1e0 / 2).read_unaligned() as u32)
        } else {
            (esi as *const u32).add(0x20 / 4).read_unaligned()
        };
        let mut quad = [0.0f32; 4];
        quad[0] = f32::from_bits((sel as *const u32).add(0x30 / 4).read_unaligned());
        quad[1] = f32::from_bits((sel as *const u32).add(0x34 / 4).read_unaligned());
        quad[2] = f32::from_bits((sel as *const u32).add(0x38 / 4).read_unaligned());
        quad[3] = f32::from_bits((sel as *const u32).add(0x3c / 4).read_unaligned());
        let n: u32 = callee_thiscall!(15, u32, *global::<u32>(0x012b9c78),
            quad.as_ptr() as u32, 0x3f000000, cell.as_ptr() as u32, 0,
            0x20, 0xffffffff, 7, 1, 0);
        if (n as i32) <= 0 {
            return 0.0;
        }
        let back: u32 = callee_cdecl!(16, u32, cell[0]);
        if back != esi {
            return 0.0;
        }
        callee_thiscall!(17, f64, this, p1, p2, cell.as_ptr() as u32, 0x3f800000)
    }
});

/// Mutant: drops the timer state-bit store (heap check must catch it).
