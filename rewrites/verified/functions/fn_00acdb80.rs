// original: 0x00acdb80 audio_update_voice
/// Voice update: probe the source, resolve its mix row, decay the level,
/// rebuild the orientation matrix, transform the emitted vector through the
/// parent matrix, accumulate it over the tick, and hand the finished slice
/// to the mixer objects.
///
/// `this` is the voice, `source` the sound source, `dt` the tick length.
/// Returns nothing; effects are the outgoing calls and the matrix/flag writes.
export!(thiscall, rw_acdb80(this_voice: u32, source: u32, dt_bits: u32) -> u32 {
    /// Per-tick decay applied to the voice level (5.0, read-only game data).
    const FADE_RATE: f32 = 5.0;
    /// File VA of the global row table indexed by the source id.
    const ROW_TABLE: u32 = 0x1295cd8;
    /// File VA of the direction vector shared with the offset block.
    const DIR_VEC: u32 = 0x110db00;
    /// File VA of the flag byte enabling the fade block.
    const FADE_FLAG: u32 = 0x103f35c;
    unsafe {
        let dt = f32::from_bits(dt_bits);
        let vtable = *(source as *const u32);
        let slot_a = *((vtable + 0xa0) as *const u32);
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot_a as usize);
        let gate = if probe(source) == 0 {
            *((source + 0x100) as *const u32)
        } else {
            let obj = probe(source);
            let inner_vt = *(obj as *const u32);
            let slot_b = *((inner_vt + 0xe0) as *const u32);
            let deep: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot_b as usize);
            deep(obj)
        };
        if gate == 0 {
            return 0;
        }
        let index = *((source + 0x2e) as *const i16) as i32;
        let col_index = *(this_voice as *const u32);
        let table = relocated(ROW_TABLE);
        let row_addr =
            ((table as i32).wrapping_add(index.wrapping_mul(4))) as u32;
        let row = *(row_addr as *const u32);
        let cols = *((row + 0xcc) as *const u32);
        let picked = *(cols.wrapping_add(col_index.wrapping_mul(4)) as *const u32);
        if (picked as i32) < 0 {
            return 0;
        }
        if *global::<u8>(FADE_FLAG) != 0 {
            let old = *((this_voice + 0x70) as *const f32);
            *((this_voice + 0x74) as *mut f32) = old;
            let next = old - dt * FADE_RATE;
            *((this_voice + 0x70) as *mut f32) =
                if 0.0f32 > next { 0.0 } else { next };
        }
        let handle = (this_voice + 0xd4) as *mut u32;
        if *handle != 0 {
            callee_thiscall!(4, u32, *handle, handle as u32);
        }
        *handle = 0;
        let matrix: u32 = callee_thiscall!(6, u32, source, picked);
        let mf = matrix as *mut f32;
        let mu = matrix as *mut u32;
        let tf = this_voice as *const f32;
        let tu = this_voice as *const u32;
        *mu.add(0) = 0x3f800000;
        *mu.add(1) = 0;
        *mu.add(2) = 0;
        *mu.add(4) = 0;
        *mu.add(5) = 0x3f800000;
        *mu.add(6) = 0;
        *mu.add(8) = 0;
        *mu.add(9) = 0;
        *mu.add(10) = 0x3f800000;
        *mf.add(9) = *tf.add(0x44 / 4);
        *mf.add(10) = *tf.add(0x48 / 4);
        *mu.add(8) = *tu.add(0x40 / 4);
        *mu.add(11) = *tu.add(0x4c / 4);
        *mf.add(2) = *tf.add(0x38 / 4);
        *mf.add(1) = *tf.add(0x34 / 4);
        *mu.add(0) = *tu.add(0x30 / 4);
        *mu.add(3) = *tu.add(0x3c / 4);
        *mf.add(4) = *mf.add(9) * *mf.add(2) - *mf.add(10) * *mf.add(1);
        *mf.add(5) = *mf.add(0) * *mf.add(10) - *mf.add(2) * *mf.add(8);
        *mf.add(6) = *mf.add(1) * *mf.add(8) - *mf.add(0) * *mf.add(9);
        let m0 = *mf.add(0);
        let m1 = *mf.add(1);
        let m2 = *mf.add(2);
        // The original tests the squared norm for exact zero (via lahf) and
        // substitutes 0 for the inverse length in that case only; NaN still
        // flows through the square root and division.
        let norm2 = (m0 * m0 + m1 * m1) + m2 * m2;
        let inv = if norm2 == 0.0 {
            0.0
        } else {
            1.0 / norm2.sqrt()
        };
        *mf.add(0) = m0 * inv;
        *mf.add(1) = m1 * inv;
        *mf.add(2) = m2 * inv;
        *mf.add(13) = *tf.add(0x54 / 4);
        *mu.add(12) = *tu.add(0x50 / 4);
        *mf.add(14) = *tf.add(0x58 / 4);
        *mu.add(15) = *tu.add(0x5c / 4);
        *mf.add(14) = *mf.add(14) + *tf.add(8 / 4);
        let mut vec = [0.0f32; 3];
        let slot_c = *((vtable + 0xec) as *const u32);
        let emit: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot_c as usize);
        emit(source, vec.as_mut_ptr() as u32);
        let parent = *((source + 0xdc4) as *const u32);
        let pf = parent as *const f32;
        let v0 = vec[0];
        let v1 = vec[1];
        let v2 = vec[2];
        vec[0] = *pf.add(5) * v1 + *pf.add(4) * v0 + *pf.add(6) * v2;
        vec[1] = *pf.add(9) * v1 + *pf.add(8) * v0 + *pf.add(10) * v2;
        vec[2] = *pf.add(13) * v1 + v0 * *pf.add(12) + *pf.add(14) * v2;
        let mut block = [0.0f32; 10];
        callee_thiscall!(5, u32, block.as_mut_ptr() as u32, matrix);
        // The original accumulates onto uninitialized stack slots here; the
        // harness defines those to zero, so the accumulators start at zero.
        let mut acc = [0.0f32; 3];
        acc[0] = vec[0] * dt + acc[0];
        acc[1] = vec[1] * dt + acc[1];
        acc[2] = vec[2] * dt + acc[2];
        let flags = *tu.add(0x164 / 4);
        if (flags >> 10) & 1 == 0 {
            let s = if (flags >> 11) & 1 == 1 {
                1.0f32
            } else {
                -1.0f32
            };
            let dir = global::<[f32; 3]>(DIR_VEC);
            let k = *tf.add(0x10 / 4) * s;
            acc[0] = (*dir)[0] * k + acc[0];
            acc[1] = (*dir)[1] * k + acc[1];
            acc[2] = (*dir)[2] * k + acc[2];
        }
        let _ = acc;
        let chain = *((parent + 4) as *const u32);
        let combo = *((chain + 0xc) as *const u32);
        let voice_idx = *((this_voice + 4) as *const i16) as i32 as u32;
        callee_thiscall!(7, u32, combo, voice_idx, block.as_mut_ptr() as u32);
        *mu.add(12) = *tu.add(0x60 / 4);
        *mf.add(13) = *tf.add(0x64 / 4);
        *mf.add(14) = *tf.add(0x68 / 4);
        *mu.add(15) = *tu.add(0x6c / 4);
        let mut s = *tf.add(0x70 / 4);
        let cap = *tf.add(0x18 / 4);
        if s > cap {
            s = cap;
        }
        s = s + *tf.add(8 / 4);
        let b8 = *mf.add(8);
        let b9 = *mf.add(9);
        let b10 = *mf.add(10);
        let q9 = b9 * s;
        let q10 = b10 * s;
        let q8 = b8 * s;
        *mf.add(13) = q9 + *tf.add(0x64 / 4);
        *mf.add(12) = *mf.add(12) + q8;
        *mf.add(14) = q10 + *tf.add(0x68 / 4);
        callee_thiscall!(8, u32, combo, voice_idx, matrix);
        let pc = *((parent + 0x64) as *const u32);
        if pc == 0 {
            return 0;
        }
        let base = *((pc + 0x1b8) as *const u32);
        if base == 0 {
            return 0;
        }
        let base2 = *((pc + 0x174) as *const u32);
        let mixer = *((pc + 0x17c) as *const u32);
        let ans: u32 = callee_thiscall!(9, u32, mixer, voice_idx);
        let entry =
            *((base2.wrapping_add(4).wrapping_add(ans.wrapping_mul(4))) as *const u32);
        callee_thiscall!(
            5,
            u32,
            block.as_mut_ptr() as u32,
            entry.wrapping_add(0x1a0)
        );
        block.swap(1, 4);
        block.swap(2, 8);
        block.swap(6, 9);
        let idx2 = *((this_voice + 4) as *const i16) as i32 as u32;
        let lane = base.wrapping_add(idx2.wrapping_shl(6));
        callee_thiscall!(10, u32, lane, matrix);
        callee_thiscall!(11, u32, lane, parent.wrapping_add(0x10));
        let src = mixer as *const f32;
        let dst = lane as *mut f32;
        *dst.add(12) = *dst.add(12) - *src.add(0x100 / 4);
        *dst.add(13) = *dst.add(13) - *src.add(0x104 / 4);
        *dst.add(14) = *dst.add(14) - *src.add(0x108 / 4);
        callee_thiscall!(12, u32, lane, block.as_mut_ptr() as u32);
        0
    }
});
