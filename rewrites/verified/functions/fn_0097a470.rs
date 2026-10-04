// original: 0x0097a470 audio_update_voice_pair
/// Updates two voice slots of a sound record.
///
/// Skips entirely unless the audio system is live (three global gates). Per
/// slot: fetches a gain from a helper, scales it, resolves the voice handle
/// through a table keyed by the slot's type bytes, and, when the slot's level
/// exceeds its floor, programs the voice through a scratch descriptor.
/// Returns nothing.
export!(thiscall, rw_0097a470(this_ptr: u32, rec: u32) -> () {
    const GATE_RUNNING: u32 = 0x011F7060;
    const GATE_STATE: u32 = 0x012088B4;
    const GATE_STATE_REF: u32 = 0x00F1C040;
    const GATE_MODE: u32 = 0x01037720;
    const TABLE_STRIDE: u32 = 0x0115D968;
    const TABLE_BASE: u32 = 0x0115D988;
    const ROW_BYTES: u32 = 0x6F40;
    const ROW_HEAD: u32 = 0x6F14;

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
        for iter in 0..2u32 {
            // ebx runs 1, 2; the and/jns sequence in the original is (ebx & 1).
            let ebx = 1 + iter;
            let esi = rec.wrapping_add(0x28 + iter * 4);
            let obj = *(esi as *const u32);
            if obj == 0 {
                continue;
            }
            let key = *((rec + 0x58 + (ebx & 1) * 4) as *const u32);
            let aux = *((esi + 0x30) as *const u32);
            // The callee reads only this one word (verified in its body) and
            // pops one; the original's three extra pushes are dead stack that
            // a later callee cleans up. Reproducing the push/pop imbalance in
            // Rust would break the compiler's stack model, so the dead words
            // travel with the later call instead (same net stack effect, same
            // logged words; the dead addresses are skipped in the contract).
            let gain: f32 = callee_thiscall!(1, f32, this_ptr, key);
            let scale = *((esi + 0x18) as *const f32);
            let dead = [0u32; 2];
            let dead0 = &dead[0] as *const u32 as u32;
            let dead1 = &dead[1] as *const u32 as u32;
            callee_thiscall!(
                2,
                u32,
                this_ptr,
                (gain * scale).to_bits(),
                aux,
                dead0,
                dead1
            );
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
            let voice = resolve(obj);
            let floor: f32 = callee_thiscall!(3, f32, voice);
            // The original compares a never-written stack slot (defined fill
            // 0.0) against the floor and proceeds only when above it.
            let level: f32 = 0.0;
            if level > floor {
                let obj2 = *(esi as *const u32);
                let voice2 = resolve(obj2);
                callee_thiscall!(4, u32, voice2, level.to_bits());
                // Scratch descriptor, filled by the stubbed initializer.
                let mut desc = [0u32; 12];
                let dp = desc.as_mut_ptr() as u32;
                callee_thiscall!(5, u32, dp, 0xD);
                *((dp + 0x29) as *mut u8) |= 2;
                *((dp + 0x0C) as *mut u32) = level.to_bits();
                let tail = *((obj2 + 0xA4) as *const u32);
                // Push order is the descriptor first, the tail word second,
                // so the tail word is argument 0 on top of the stack.
                callee_cdecl!(6, u32, tail, dp);
            }
        }
    }
});
