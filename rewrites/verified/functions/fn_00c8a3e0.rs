// original: 0x00c8a3e0 audio_voice_gate_retry
/// Audio voice/listener gating check with directional retry grid.
///
/// Samples a distance-like value for the voice, rejects voices past the
/// audible range, then asks the voice object (through its slot-0xEC hook)
/// for a direction triple, normalizes it, and dots it against the
/// listener's orientation. Depending on the sign it runs one of two retry
/// grids over the per-slot helper, sweeping slot indexes up or down until
/// the helper accepts.
///
/// Original: thiscall/2 (this = owner, arg0 = voice object, arg1 = opaque
/// parameter forwarded to the helper). Returns the last helper answer, or
/// the sample bits on the early-out path.
export!(thiscall, rw_00c8a3e0(this: u32, obj: *mut u8, param: u32) -> u32 {
    unsafe {
        // Distance sample for the voice's position block.
        let pos = *(obj.add(0x20) as *const u32);
        let posb = pos as *const u8;
        let sample_arg = pos.wrapping_add(0x30);
        let dist: f32 = callee_cdecl!(1, f32, sample_arg);
        if dist > *global::<f32>(0xFE8CB0) {
            return dist.to_bits();
        }
        // Direction triple from the voice hook (slot 0xEC), filled into a
        // small frame buffer; then its squared length, accumulated in the
        // original's component order.
        let vtable = *(obj as *const u32);
        let target = *((vtable as *const u8).add(0xEC) as *const u32);
        let hook: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let mut triple = [0u32; 3];
        hook(obj as u32, triple.as_mut_ptr() as u32);
        let cx = f32::from_bits(triple[0]);
        let cy = f32::from_bits(triple[1]);
        let cz = f32::from_bits(triple[2]);
        let len2 = (cy * cy + cx * cx) + cz * cz;
        // Zero-length stays zero (avoids a divide by zero); otherwise the
        // reciprocal length. NaN flows through the sqrt/divide path.
        let one = *global::<f32>(0xFE88E8);
        let k = if len2 == 0.0 { 0.0 } else { one / len2.sqrt() };
        let _nx = cx * k;
        let _ny = cy * k;
        let _nz = cz * k;
        // Second hook call returns the triple to dot against the listener
        // orientation instead of using the frame buffer.
        let target2 = *((vtable as *const u8).add(0xEC) as *const u32);
        let hook2: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target2 as usize);
        let mut triple2 = [0u32; 3];
        let rp = hook2(obj as u32, triple2.as_mut_ptr() as u32);
        let r0 = f32::from_bits(*(rp as *const u32));
        let r1 = f32::from_bits(*((rp as *const u32).add(1)));
        let r2 = f32::from_bits(*((rp as *const u32).add(2)));
        let lx = f32::from_bits(*(posb.add(0x10) as *const u32));
        let ly = f32::from_bits(*(posb.add(0x14) as *const u32));
        let lz = f32::from_bits(*(posb.add(0x18) as *const u32));
        let dot = (ly * r1 + lx * r0) + lz * r2;
        if dot < 0.0 || dot.is_nan() {
            // Facing away: sweep slots 0..=4 with mode (0,1,0), then
            // slots 4..=0 with mode (1,0,0); stop at the first accept.
            let mut ans = 0u32;
            let mut slot = 0u32;
            loop {
                ans = callee_thiscall!(3, u32, this, obj as u32, param, slot, 0, 1, 0);
                if ans & 0xFF != 0 {
                    break;
                }
                slot += 1;
                if slot > 4 {
                    break;
                }
            }
            let mut slot = 4i32;
            loop {
                ans = callee_thiscall!(
                    3, u32, this, obj as u32, param, slot as u32, 0, 0, 1
                );
                if ans & 0xFF != 0 {
                    break;
                }
                slot -= 1;
                if slot < 0 {
                    break;
                }
            }
            ans
        } else {
            // Facing the listener: pre-pass slots 4..=0 with mode
            // (0,1,1), then sweep 0..=4 with mode (1,0,1) until accept.
            let mut ans = 0u32;
            let mut slot = 4i32;
            loop {
                ans = callee_thiscall!(
                    3, u32, this, obj as u32, param, slot as u32, 1, 1, 0
                );
                if ans & 0xFF != 0 {
                    break;
                }
                slot -= 1;
                if slot < 0 {
                    break;
                }
            }
            let mut slot = 0u32;
            loop {
                ans = callee_thiscall!(3, u32, this, obj as u32, param, slot, 1, 0, 1);
                if ans & 0xFF != 0 {
                    return ans;
                }
                slot += 1;
                if slot > 4 {
                    break;
                }
            }
            ans
        }
    }
});
