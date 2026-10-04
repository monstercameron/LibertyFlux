// original: 0x00ca61c0 CEventHandler::vf74
/// Event reaction 74: pick a spoken line for the event, then derive a blend
/// target from the event record and commit it through the behaviour manager.
///
/// `ev` carries several staged float triples plus a mode word at +0x40:
/// mode 3 requests a scripted action directly, mode 0 re-aims at the
/// owner's position, mode 1 rotates the staged offset by a scripted angle,
/// and any other mode keeps the staged offset. The chosen offset triple
/// is handed to the behaviour call; the resulting handle (or zero when no
/// manager answers) is stored at owner+0xc and returned.
export!(thiscall, rw_rb02_vf74(this_ptr: u32, ev: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        let dir_x = *((ev.wrapping_add(0x10)) as *const f32);
        let dir_y = *((ev.wrapping_add(0x14)) as *const f32);
        let dir_z = *((ev.wrapping_add(0x18)) as *const f32);
        let pos_x = *((ev.wrapping_add(0x30)) as *const f32);
        let pos_y = *((ev.wrapping_add(0x34)) as *const f32);
        let pos_z = *((ev.wrapping_add(0x38)) as *const f32);
        let mode = *((ev.wrapping_add(0x40)) as *const u32);
        let p1 = *((this_ptr.wrapping_add(4)) as *const u32);

        // Gate call; a large answer skips the speech request below.
        let gate: u32 = callee_thiscall!(1, u32, this_ptr);
        let skipped_speech = (gate as i32) >= 0x3fff;
        let mut speech_ecx = this_ptr;
        if !skipped_speech {
            let inner = *((p1.wrapping_add(0x21c)) as *const u32);
            let kind = *((inner.wrapping_add(0x12c)) as *const u32);
            let in_range = kind == 2 || kind.wrapping_sub(3) <= 0xb;
            let flag_clear = *((ev.wrapping_add(0x4c)) as *const u8) == 0;
            let tag: u32 = if in_range && flag_clear { relocated(0x00ed7850) } else { relocated(0x00ed7868) };
            speech_ecx = p1.wrapping_add(0x570);
            let _speech: u32 = callee_thiscall!(2, u32, speech_ecx,
                tag, 0u32, 0u32, 0u32, 0xffffffffu32,
                0u32, 0u32, 0x3f800000u32, 0u32, 0u32);
        }

        // Mode 3: scripted action straight away.
        if mode == 3 {
            let strength = *((ev.wrapping_add(0x20)) as *const f32);
            let bus = *global::<u32>(0x167e2a0);
            let target = *((ev.wrapping_add(0x44)) as *const u32);
            let mgr: u32 = callee_thiscall!(3, u32, bus);
            if mgr == 0 {
                *((this_ptr.wrapping_add(0xc)) as *mut u32) = 0;
                return 0;
            }
            let dir = [dir_x, dir_y, dir_z];
            let ans: u32 = callee_thiscall!(4, u32, mgr,
                0x40400000u32, (&dir as *const f32) as u32, strength.to_bits(),
                target, 0x2710u32);
            *((this_ptr.wrapping_add(0xc)) as *mut u32) = ans;
            return ans;
        }

        // Blend target: staged position, refined by mode.
        let mut ox = pos_x;
        let mut oy = pos_y;
        let mut oz = pos_z;
        if mode == 0 {
            let base = *((p1.wrapping_add(0x20)) as *const u32);
            let dx = *((base.wrapping_add(0x30)) as *const f32) - dir_x;
            let dy = *((base.wrapping_add(0x34)) as *const f32) - dir_y;
            let dz = *((base.wrapping_add(0x38)) as *const f32) - dir_z;
            let lensq = dy * dy + dx * dx + dz * dz;
            // Inverse length with a zero guard: the original tests the
            // compare flags' parity, which skips only an exactly-zero
            // length (NaN and infinity both take the divide path).
            let inv = if lensq == 0.0 { 0.0 } else { 1.0f32 / lensq.sqrt() };
            ox = dx * inv;
            oy = dy * inv;
            oz = dz * inv;
        } else if mode == 1 {
            // ECX here is post-call residue in the original (whatever the
            // speech call left behind), not a real input; the contract
            // skips it. Pass through the same register value for shape.
            let ecx_b = if skipped_speech { this_ptr } else { speech_ecx };
            let n: u32 = callee_thiscall!(1, u32, ecx_b);
            let mut t = (n as i32) as f32;
            t *= *global::<f32>(0xfe8684);
            t *= *global::<f32>(0xfe8aa0);
            t -= *global::<f32>(0xfe8978);
            let s28 = t;
            let sin_bits: u32 = callee_cdecl!(5, u32, t.to_bits());
            let sin = f32::from_bits(sin_bits);
            let cos_bits: u32 = callee_cdecl!(6, u32, s28.to_bits());
            let cos = f32::from_bits(cos_bits);
            let x1 = oy * sin;
            let mut x3 = ox;
            x3 *= sin;
            let mut x4 = ox;
            x4 *= cos;
            let mut x2 = oy;
            x2 *= cos;
            x4 -= x1;
            x2 += x3;
            ox = x4;
            oy = x2;
        }

        // Commit the blend target through the behaviour manager.
        let bus2 = *global::<u32>(0x167e2a0);
        let mgr2: u32 = callee_thiscall!(3, u32, bus2);
        if mgr2 == 0 {
            *((this_ptr.wrapping_add(0xc)) as *mut u32) = 0;
            return 0;
        }
        let out = [ox, oy, oz];
        let ans7: u32 = callee_thiscall!(7, u32, mgr2,
            0u32, (&out as *const f32) as u32, 0u32, 1u32);
        *((this_ptr.wrapping_add(0xc)) as *mut u32) = ans7;
        ans7
    }
});
