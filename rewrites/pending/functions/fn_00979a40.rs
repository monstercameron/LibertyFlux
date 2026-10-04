// original: 0x00979a40 audio_collision_voice_update
/// Audio-collision voice update: resolves the sounding object and its voice,
/// scales the incoming level through the subsystem curve, and either stores
/// the result or, for looped pairs, submits a follow-up event first.
///
/// Returns nothing; observable effects are the outgoing calls and the words
/// stored into the voice object (level, selectors, flags).
export!(thiscall, rw_00979a40(this: u32, arg0: u32, arg1: u32, arg2: u32, _arg3: u32) -> u32 {
    unsafe {
        if *global::<u32>(0x11f7060) == 1 {
            return 0;
        }
        if *global::<u32>(0x12088b4) != *global::<u32>(0xf1c040) {
            return 0;
        }
        if *global::<u32>(0x1037720) == 0x12 {
            return 0;
        }
        let head = *((arg0) as *const u32);
        // Note the asymmetry: a null head skips the lookup but continues
        // with a null handle, while a failed lookup exits.
        let edi = if head == 0 {
            0
        } else {
            let r = callee_cdecl!(1, u32, head);
            if r == 0 {
                return 0;
            }
            r
        };
        let esi = callee_thiscall!(2, u32, this);
        if esi == 0 {
            return 0;
        }
        callee_thiscall!(3, u32, this, esi, arg0);
        if edi == 0 {
            // The original falls through to a null dereference here; fault
            // identically (volatile so the load is really emitted).
            let _ = core::ptr::read_volatile(0x28 as *const u32);
            return 0;
        }
        // Class flag at +0x28 selects the base gain and voice selector.
        let gain: f32;
        if (*((edi.wrapping_add(0x28)) as *const u32) & 0x3c0) == 0x80 {
            *((esi.wrapping_add(0x58)) as *mut u32) = *global::<u32>(0x12202c0);
            gain = f32::from_bits(0x40e00000);
            if *((edi.wrapping_add(0x1304)) as *const u32) == 1 {
                *((esi.wrapping_add(0x58)) as *mut u32) = *global::<u32>(0x12202c4);
            }
        } else {
            gain = f32::from_bits(0x40c00000);
        }
        // Level through the subsystem curve.
        let curved = callee_thiscall!(
            4, u32, this.wrapping_add(0xfa10),
            (f32::from_bits(arg1) * f32::from_bits(0x3d23d70a)).to_bits()
        );
        let level = f32::from_bits(curved) * gain;
        *((esi.wrapping_add(0x40)) as *mut u32) = level.to_bits();
        *((esi.wrapping_add(0x71)) as *mut u8) = 1;
        callee_thiscall!(5, u32, this, esi, 0);
        *((esi.wrapping_add(0x60)) as *mut u32) = 1;
        // Looped pairs submit a follow-up through the dispatch table.
        if ((*((edi.wrapping_add(0x28)) as *const u32) & 0x3c0) != 0xc0)
            || arg2 == 0
            || ((*((arg2.wrapping_add(0x28)) as *const u32) & 0x3c0) != 0xc0)
        {
            return 0;
        }
        // Dispatch table stored inline at the global (indexed, not indirect).
        let tab = relocated(0x1295cd8);
        let idx = *(((arg2.wrapping_add(0x2e)) as *const u16) as *const i16) as i32;
        let entry = *((tab.wrapping_add((idx as u32).wrapping_mul(4))) as *const u32);
        if entry != 0 {
            let alive = callee_thiscall!(6, u32, entry);
            if (alive & 0xff) == 0 {
                let link = *((esi.wrapping_add(0x28)) as *const u32);
                if link != 0 {
                    callee_thiscall!(7, u32, link, 0x258);
                    // Scratch block shared by the next two calls (both take the
                    // same base); the second call's snapshot covers words 0..8
                    // with 0x258 at [7].
                    let mut scratch = [0u32; 12];
                    callee_thiscall!(8, u32, scratch.as_mut_ptr() as u32, 0xd);
                    let sub = *(((link.wrapping_add(0xa4))) as *const u32);
                    scratch[7] = 0x258;
                    // Byte set by the original (kept for faithfulness).
                    (scratch.as_mut_ptr() as *mut u8).add(0x2d).write(8);
                    callee_cdecl!(9, u32, sub, scratch.as_mut_ptr() as u32);
                }
            }
        }
        let tag = *(((arg0.wrapping_add(0x52)) as *const u16)) as u32;
        callee_thiscall!(10, u32, edi.wrapping_add(0x3c0), tag, arg1);
        0
    }
});
