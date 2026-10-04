// original: 0x00c8ae50 audio_voice_update
/// Audio voice update: validates a voice and fans work out to the slots.
///
/// Resolves the voice's solo flag from the global voice table, samples
/// the voice distance and drops out-of-range or inactive voices, then
/// switches on the voice class: class 3 checks audibility state, class 4
/// checks height, and every surviving voice reaches a common tail that
/// either runs the directional check plus slot gate plus mixer kick, or
/// walks the slot list and routes each live slot through the event
/// router (iterating by computed stride or by dense index depending on
/// the mode byte).
///
/// Original: thiscall/3 (this = owner, arg0 = voice, arg1 = slot block,
/// arg2 = mode byte).
export!(thiscall, rw_00c8ae50(this: u32, voice: *mut u8, slots: u32, mode: u32) -> u32 {
    unsafe {
        // Solo flag from the global voice table (pristine key is -1, so
        // the flag is 0 unless the game has selected a voice).
        let key = *global::<u32>(0x1036F14);
        let mut flag = 0u32;
        if key != 0xFFFFFFFF {
            let entry = *global::<u32>(key.wrapping_mul(4).wrapping_add(0x11A8808));
            if entry != 0
                && *(voice.add(0x28) as *const u32) & 0x3C0 == 0xC0
                && *((entry as *const u8).add(0x598) as *const u32) == voice as u32
            {
                flag = 1;
            }
        }
        // Distance sample for the voice position block (or the inline
        // origin when the voice has no block yet).
        let pb = *(voice.add(0x20) as *const u32);
        let sample_arg = if pb != 0 {
            pb.wrapping_add(0x30)
        } else {
            (voice as u32).wrapping_add(0x10)
        };
        let d: f32 = callee_cdecl!(1, f32, sample_arg);
        if flag == 0 && d > *global::<f32>(0xFE8CB0) {
            return d.to_bits();
        }
        let r: u32 = callee_thiscall!(2, u32, voice as u32, 0);
        if r & 0xFF == 0 {
            return r;
        }
        let voice_w = *(voice.add(0x28) as *const u32);
        let class = (voice_w >> 6) & 0xF;
        if class == 3 {
            // NOTE: EAX holds the switch value here, not r: the shift-and
            // above overwrote the helper answer.
            if d > *global::<f32>(0xE9CAE0) {
                if flag == 0 {
                    return 3;
                }
            }
            if *(voice.add(0x210) as *const u8) != 0 {
                let a = *(voice.add(0xa74) as *const u32);
                if a == 1 || a == 2 {
                    return a;
                }
                if *(voice.add(0x224) as *const u32) != 0 {
                    return common_tail(this, voice as u32, slots, mode, flag);
                }
                return a;
            }
            if *(voice.add(0x224) as *const u32) != 0 {
                return common_tail(this, voice as u32, slots, mode, flag);
            }
            return 3;
        }
        if class == 4 {
            if *(voice.add(0x24) as *const u32) & 0x200000 != 0 {
                return 4;
            }
            if d > *global::<f32>(0xE9CAE0) {
                return 4;
            }
            let vtable = *(voice as *const u32);
            let target = *((vtable as *const u8).add(0xEC) as *const u32);
            let hook: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let mut dirs = [0u32; 3];
            let rp = hook(voice as u32, dirs.as_mut_ptr() as u32);
            let up = -f32::from_bits(*((rp as *const u32).add(2)));
            if *global::<f32>(0xEA42C0) > up {
                return rp;
            }
        }
        common_tail(this, voice as u32, slots, mode, flag)
    }
});

/// Shared tail of the voice update: direct kick or slot-list walk.
unsafe fn common_tail(this: u32, voice: u32, slots: u32, mode: u32, flag: u32) -> u32 {
    let voice_b = voice as *mut u8;
    let w = *(voice_b.add(0x28) as *const u32);
    if w & 0x3C0 == 0x80 && *(voice_b.add(0x1304) as *const u32) == 2 {
        if w & 0x7C00 == 0xC00 {
            return 0x80;
        }
