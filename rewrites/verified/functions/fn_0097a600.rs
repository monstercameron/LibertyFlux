// original: 0x0097a600 audio_update_voice_field
/// Updates two field-voice slots of a sound record.
///
/// Same live-gates as its sibling, then a per-record scale factor and an
/// empty-record hook, then per slot: gain fetch and scale, voice-handle
/// resolution, a level store through the resolved handle, a distance-squared
/// gate, and programming through a scratch descriptor. Returns nothing.
export!(thiscall, rw_0097a600(this_ptr: u32, rec: u32) -> () {
    const GATE_RUNNING: u32 = 0x011F7060;
    const GATE_STATE: u32 = 0x012088B4;
    const GATE_STATE_REF: u32 = 0x00F1C040;
    const GATE_MODE: u32 = 0x01037720;
    const TABLE_STRIDE: u32 = 0x0115D968;
    const TABLE_BASE: u32 = 0x0115D988;
    const ROW_BYTES: u32 = 0x6F40;
    const ROW_HEAD: u32 = 0x6F14;
    const LEVEL_OFF: u32 = 0xCC;

    unsafe {
        if *(global::<u32>(GATE_RUNNING)) == 1 {
            return;
        }
        if *(global::<u32>(GATE_STATE)) != *(global::<u32>(GATE_STATE_REF)) {
            return;
        }
        if *(global::<u32>(GATE_MODE)) == 0x12 {
            return;
        }
        let scale: f32 = callee_thiscall!(1, f32, this_ptr, rec);
        if *(rec as *const u32).add(0x38 / 4) == 0 && *(rec as *const u32).add(0x3C / 4) == 0 {
            callee_thiscall!(2, u32, this_ptr, rec);
        }
        let resolve = |o: u32| {
            let kind = *((o + 4) as *const u8);
            if kind == 0xFF {
                return 0;
            }
            let row = *((o + 0x40) as *const u8) as u32;
            let stride = *(global::<u32>(TABLE_STRIDE));
            let tbase = *(global::<u32>(TABLE_BASE));
            stride
                .wrapping_mul(kind as u32)
                .wrapping_add(*((tbase + row * ROW_BYTES + ROW_HEAD) as *const u32))
        };
        for iter in 0..2u32 {
            let ebp = 1 + iter;
            let esi = rec.wrapping_add(0x38 + iter * 4);
            let vec = rec.wrapping_add(iter * 0x10);
            let obj = *(esi as *const u32);
            if obj == 0 {
                continue;
            }
            let key = *((rec + 0x58 + (ebp & 1) * 4) as *const u32);
            let aux = *((esi + 0x20) as *const u32);
            let gain: f32 = callee_thiscall!(3, f32, this_ptr, key);
            // Dead-stack balancing travels with this call (see sibling
            // 0x0097a470): same net stack effect and same logged words.
            let dead = [0u32; 2];
            let dead0 = &dead[0] as *const u32 as u32;
            let dead1 = &dead[1] as *const u32 as u32;
            callee_thiscall!(
                4,
                u32,
                this_ptr,
                (gain * scale).to_bits(),
                aux,
                dead0,
                dead1
            );
            let voice = resolve(obj);
            // Argument is a never-written stack slot (defined fill 0).
            callee_thiscall!(5, u32, voice, 0);
            let voice2 = resolve(*(esi as *const u32));
            // Stored value is a never-written stack slot (defined fill 0.0).
            *((voice2 + LEVEL_OFF) as *mut u32) = 0.0f32.to_bits();
            let ax = *((vec + 0) as *const f32);
            let bx = *((vec + 4) as *const f32);
            let cx = *((vec + 8) as *const f32);
            let dist2 = (ax * ax + bx * bx) + cx * cx;
            if (dist2.to_bits() & 0x7F800000) != 0x7F800000 {
                let voice3 = resolve(*(esi as *const u32));
                callee_thiscall!(6, u32, voice3, vec);
            }
            let mut desc = [0u32; 12];
            let dp = desc.as_mut_ptr() as u32;
            callee_thiscall!(7, u32, dp, 0x10);
            *((dp + 0x29) as *mut u8) |= 0x0C;
            *((dp + 0x10) as *mut u32) = *((vec + 0) as *const u32);
            *((dp + 0x14) as *mut u32) = *((vec + 4) as *const u32);
            *((dp + 0x18) as *mut u32) = *((vec + 8) as *const u32);
            // A never-written stack slot (defined fill 0).
            *((dp + 0x1C) as *mut u32) = 0;
            let tail = *((*(esi as *const u32) + 0xA4) as *const u32);
            callee_cdecl!(8, u32, tail, dp);
        }
    }
});
