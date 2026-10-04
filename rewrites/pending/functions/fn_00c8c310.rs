// original: 0x00C8C310 audio_emitter_poll
export!(cdecl, rw_00c8c310(obj: u32) -> () {
    unsafe {
        let vt = *(obj as *const u32);
        // Position query through vtable slot 0xEC; takes a frame scratch
        // slot whose address (not contents) is observable by the callee.
        let query_pos: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vt + 0xEC) as *const u32));
        let mut scratch: u32 = 0;
        let pos = query_pos(obj, &mut scratch as *mut u32 as u32);
        let x = *(pos as *const f32);
        let y = *((pos.wrapping_add(4)) as *const f32);
        let z = *((pos.wrapping_add(8)) as *const f32);
        let dist_sq = x * x + y * y + z * z;
        // Near gate: continue only while dist_sq is ordered-below 4.0.
        if !(*global::<f32>(0x00FE8AB8) > dist_sq) {
            return;
        }
        // Engine readiness probe.
        let ready = callee_thiscall!(1, u32, obj);
        if (ready as u8) == 0 {
            if *(obj.wrapping_add(0x288) as *const u32) == 0 {
                return;
            }
            callee_cdecl!(4, u32, obj);
            *(obj.wrapping_add(0x288) as *mut u32) = 0;
            return;
        }
        // Audibility probe on the emitter (or its parent) position block.
        let anchor = *(obj.wrapping_add(0x20) as *const u32);
        let probe_at = if anchor == 0 {
            obj.wrapping_add(0x10)
        } else {
            anchor.wrapping_add(0x30)
        };
        let audible = callee_cdecl!(2, u32, probe_at, 1);
        if (audible as u8) == 0 {
            *(obj.wrapping_add(0x288) as *mut u32) = 0;
            return;
        }
        let mut voice: u32 = callee_cdecl!(3, u32, obj);
        let pick_a: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt + 0x64) as *const u32));
        let pick_b: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt + 0x60) as *const u32));
        let sign_mask = *global::<u32>(0x00FE8FA0);
        let pick_tol = *global::<f32>(0x00EAB200);
        if voice == 3 {
            let a = pick_a(obj);
            let b = pick_b(obj);
            let mut d = *(a as *const f32) - *(b as *const f32);
            if 0.0f32 > d {
                d = f32::from_bits(d.to_bits() ^ sign_mask);
            }
            if pick_tol > d {
                voice = 0;
            }
        } else if voice == 2 {
            let a = pick_a(obj);
            let b = pick_b(obj);
            let mut d = *((a.wrapping_add(4)) as *const f32)
                - *((b.wrapping_add(4)) as *const f32);
            if 0.0f32 > d {
                d = f32::from_bits(d.to_bits() ^ sign_mask);
            }
            if pick_tol > d {
                voice = 0;
            }
        }
        if voice != *(obj.wrapping_add(0x288) as *const u32) {
            callee_cdecl!(4, u32, obj);
            *(obj.wrapping_add(0x288) as *mut u32) = voice;
        }
        if voice == 0 {
            return;
        }
        callee_cdecl!(5, u32, obj);
    }
});
