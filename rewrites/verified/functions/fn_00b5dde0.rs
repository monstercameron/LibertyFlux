// original: 0x00b5dde0 audio_event_gate
/// Decide whether an audio event on an entity plays, and at what level.
///
/// `this` carries the emitter key at +0x18 used for both state lookups;
/// `ent` is the entity the event is attached to. Returns nothing meaningful
/// (the original leaves whatever the last call left in EAX).
export!(thiscall, rw_b5dde0(this: *mut u8, ent: *mut u8, _a1: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        let gate = *((ent.add(0x6c)) as *mut u32);
        if gate != 0 && *(((gate as *mut u8).add(0xe)) as *mut u8) != 0 {
            return 0;
        }
        let key = *((this.add(0x18)) as *mut u32);
        let first = callee_cdecl!(1, u32, key);
        let second = callee_cdecl!(1, u32, key);
        let flags = *(((second as *mut u8).add(0x20)) as *mut u32);
        if (flags >> 5) & 1 == 1 {
            callee_cdecl!(2, u32, 0x11f, 1.0f32.to_bits());
            callee_cdecl!(3, u32,);
            if first != 0
                && *((ent.add(0x218)) as *mut u8) == 0
                && *((ent.add(0x219)) as *mut u8) != 0
            {
                let slot = ((first as *mut u8).add(0x100)) as *mut u32;
                *slot = (*slot).wrapping_add(1);
            }
        } else if first != 0 {
            let mode = *(((first as *mut u8).add(0xc)) as *mut u32);
            let alt = *(((first as *mut u8).add(8)) as *mut u32);
            if mode == 3 || (alt == 3 && mode == 4) {
                callee_cdecl!(2, u32, 0x1bf, 1.0f32.to_bits());
                callee_cdecl!(3, u32,);
            }
        }
        let count = *(((first as *mut u8).add(0x70)) as *mut i32);
        if count > 0 {
            let mut suppressed = false;
            if *((first as *mut u8).add(0x23)) & 1 != 0
                && (*((ent.add(0x28)) as *mut u32) & 0x3c0) == 0xc0
            {
                let holder = *((ent.add(0x224)) as *mut u32);
                let mut node = *(((holder as *mut u8).add(0x2e0)) as *mut u32);
                if node != 0 {
                    loop {
                        if *(((node as *mut u8).add(4)) as *mut u32) == 0x2e4 {
                            suppressed = true;
                            break;
                        }
                        node = *(((node as *mut u8).add(0xc)) as *mut u32);
                        if node == 0 {
                            break;
                        }
                    }
                }
            }
            if !suppressed {
                let level = *(((first as *mut u8).add(0x74)) as *mut u32);
                callee_cdecl!(4, u32, count as u32, level, 0);
            }
        }
        0
    }
});