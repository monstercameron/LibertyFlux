// original: 0x00b5dc70 audio_directional_gain
/// Mix a directional gain for an entity into the audio graph.
///
/// `this` carries the emitter key at +0x18; `ent` selects the target entity
/// (directly or through its redirect link depending on its mode bits).
/// Returns nothing meaningful. Note: the original reads three uninitialized
/// frame slots as the base position; the contract defines them as zero
/// (`stack_fill`), which this rewrite uses directly.
export!(thiscall, rw_b5dc70(this: *mut u8, ent: *mut u8, _a1: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        let key = *((this.add(0x18)) as *mut u32);
        let s = callee_cdecl!(1, u32, key);
        if ((*(((s as *mut u8).add(0x20)) as *mut u32) >> 5) & 1) == 0 {
            return 0;
        }
        if ent.is_null() {
            return 0;
        }
        let mode = (*((ent.add(0x28)) as *mut u32) >> 6) & 0xf;
        let tgt: *mut u8 = if mode == 4 {
            ent
        } else if mode == 3 {
            *((ent.add(0x2c4)) as *mut u32) as *mut u8
        } else {
            return 0;
        };
        if tgt.is_null() || *((tgt.add(0x24)) as *mut u8) & 1 == 0 {
            return 0;
        }
        let pos_key = *((tgt.add(0x20)) as *mut u32);
        let mut basis = [0u32; 3];
        callee_thiscall!(2, u32, this as u32, pos_key, basis.as_mut_ptr() as u32, 3, 0);
        let s2 = callee_cdecl!(1, u32, key);
        let raw = ((s2 as *mut u8).add(0x90)) as *mut f32;
        let scaled = *raw * *global::<f32>(0x00fe876c);
        let cap = *global::<f32>(0x00fe8ad8);
        let level = if scaled > cap { cap } else { scaled };
        let p = callee_thiscall!(4, u32, tgt as u32);
        let vt = *(p as *mut u32) as *mut u32;
        let slot = *(vt.add(0x24 / 4)) as usize;
        let f: extern "thiscall" fn(u32) -> f32 = core::mem::transmute(slot);
        let rate = f(p);
        let gain = rate * level;
        let base: *mut u8 = if pos_key == 0 {
            tgt.add(0x10)
        } else {
            (pos_key as *mut u8).add(0x30)
        };
        // The original subtracts the anchor position from three uninitialized
        // frame slots; under the contract's zero stack fill those read +0.0.
        // black_box keeps the subtraction from folding into a negation, which
        // differs from subss on +0.0 and NaN inputs.
        let fill = black_box(0.0f32);
        let dx = fill - *((base.add(0)) as *mut f32);
        let dy = fill - *((base.add(4)) as *mut f32);
        let dz = fill - *((base.add(8)) as *mut f32);
        let dir = [dx.to_bits(), dy.to_bits(), dz.to_bits()];
        let b0 = -f32::from_bits(basis[0]) * gain;
        let b1 = -f32::from_bits(basis[1]) * gain;
        let b2 = -f32::from_bits(basis[2]) * gain;
        let out = [b0.to_bits(), b1.to_bits(), b2.to_bits()];
        callee_thiscall!(5, u32, tgt as u32, out.as_ptr() as u32, dir.as_ptr() as u32, 0);
        0
    }
});