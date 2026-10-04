// original: 0x00ca6650 CEventHandler::vf69
/// Event reaction 69: when the owner carries task kind 0x41e, sample the
/// task's tuning block, blend a facing offset from it and commit the
/// behaviour through the manager.
///
/// The tuning block arrives through seven out-pointers; its key word must
/// read 3 or the reaction ends. A flag byte picks the base angle, whose
/// sine and cosine rotate the staged offset before a final scale and shift.
/// The committed record (or zero when no manager answers) lands at
/// owner+0xc.
export!(thiscall, rw_rb02_vf69(this_ptr: u32, _a1: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        let p1 = *((this_ptr.wrapping_add(4)) as *const u32);
        let g: u32 = callee_thiscall!(1, u32, p1);
        if g == 0 {
            return 0;
        }
        let esi: u32 = callee_thiscall!(2, u32, g.wrapping_add(8));
        if esi == 0 {
            return 0;
        }
        let heapc = *((esi.wrapping_add(0x224)) as *const u32);
        let mut node = *((heapc.wrapping_add(0x2e0)) as *const u32);
        if node == 0 {
            return esi;
        }
        // Walk the kind chain while its tags do not rise.
        let mut tag0 = (*((node.wrapping_add(8)) as *const u32) >> 1) & 7;
        let mut last_tag = tag0;
        loop {
            let tag = (*((node.wrapping_add(8)) as *const u32) >> 1) & 7;
            last_tag = tag;
            if tag0 < tag {
                return tag;
            }
            tag0 = tag;
            if *((node.wrapping_add(4)) as *const u32) == 0x41e {
                break;
            }
            node = *((node.wrapping_add(12)) as *const u32);
            if node == 0 {
                return last_tag;
            }
        }
        let slot = *((p1.wrapping_add(0x224)) as *const u32);
        let chk: u32 = callee_thiscall!(3, u32, slot.wrapping_add(0x2e0), 0x76cu32, 0u32);
        if (chk & 0xff) != 0 {
            return chk;
        }
        // Sample the tuning block through seven out-pointers.
        let mut w7 = 0u32;
        let mut w6 = 0u32;
        let mut pad = [0u32; 5];
        let samp: u32 = callee_thiscall!(4, u32, heapc.wrapping_add(0x2e0),
            (&mut w7 as *mut u32) as u32, (&mut w6 as *mut u32) as u32,
            (&mut pad[4] as *mut u32) as u32, (&mut pad[3] as *mut u32) as u32,
            (&mut pad[2] as *mut u32) as u32, (&mut pad[1] as *mut u32) as u32,
            (&mut pad[0] as *mut u32) as u32);
        if w7 != 3 {
            return samp;
        }
        // Stage the facing pair and run it through the shaper.
        let based = *((esi.wrapping_add(0x20)) as *const u32);
        let mut tri = [
            *((based.wrapping_add(0x10)) as *const f32),
            *((based.wrapping_add(0x14)) as *const f32),
            0.0f32,
        ];
        let _: u32 = callee_thiscall!(5, u32, (&mut tri as *mut f32) as u32);
        let base_ang = if (w6 & 0xff) != 0 {
            *global::<f32>(0xfe8da0)
        } else {
            *global::<f32>(0xfe8978)
        };
        let sin_b: u32 = callee_cdecl!(6, u32, base_ang.to_bits());
        let sin = f32::from_bits(sin_b);
        let cos_b: u32 = callee_cdecl!(7, u32, base_ang.to_bits());
        let cos = f32::from_bits(cos_b);
        // Rotate the staged pair, scale by ten and shift to the anchor.
        let t0 = tri[0];
        let t1 = tri[1];
        let t2 = tri[2];
        let x1 = t1 * sin;
        let mut x5 = t0;
        let mut x2 = t0;
        x2 *= sin;
        x5 *= cos;
        let mut x4 = t1;
        x4 *= cos;
        let ten = *global::<f32>(0xfe8b08);
        x5 -= x1;
        let mut y1 = t2;
        x4 += x2;
        let mut y2 = *((based.wrapping_add(0x34)) as *const f32);
        y1 *= ten;
        x5 *= ten;
        x4 *= ten;
        let mut y0 = *((based.wrapping_add(0x38)) as *const f32);
        x5 += *((based.wrapping_add(0x30)) as *const f32);
        y2 += x4;
        y0 += y1;
        let blend = [x5, y2, y0];
        let bus = *global::<u32>(0x167e2a0);
        let mgr: u32 = callee_thiscall!(8, u32, bus);
        // Note: a null manager here faults below on the original too
        // (unguarded flag write through a null record); both sides agree.
        let edi: u32 = if mgr == 0 {
            0
        } else {
            let ecxval = based.wrapping_add(0x30);
            callee_thiscall!(11, u32, mgr, 0u32,
                (&blend as *const f32) as u32, 0u32, 1u32, ecxval,
                0x40400000u32, 2u32)
        };
        *((edi.wrapping_add(0x94)) as *mut u8) &= 0xf7;
        let mgr2: u32 = callee_thiscall!(9, u32, bus);
        let esi2: u32 = if mgr2 == 0 {
            0
        } else {
            callee_thiscall!(12, u32, mgr2)
        };
        let _: u32 = callee_thiscall!(13, u32, esi2, edi);
        let mgr3: u32 = callee_thiscall!(10, u32, bus);
        if mgr3 == 0 {
            let ans: u32 = callee_thiscall!(13, u32, esi2, 0u32);
            *((this_ptr.wrapping_add(0xc)) as *mut u32) = esi2;
            return ans;
        }
        let ans14: u32 = callee_thiscall!(14, u32, mgr3, 0u32,
            (&blend as *const f32) as u32, 0xbf800000u32, 0u32);
        let ans: u32 = callee_thiscall!(13, u32, esi2, ans14);
        *((this_ptr.wrapping_add(0xc)) as *mut u32) = esi2;
        ans
    }
});
