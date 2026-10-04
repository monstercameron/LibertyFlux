// original: 0x00cd5730 euphoria_channel_update
/// Gated animation-channel update.
///
/// Picks a source object through two query layers, blends two polled floats
/// selected by a mode word, clamps the result, stores a weighted value into
/// the channel object, updates its flag word, runs two gated sub-queries
/// with stack out-cells on the set-flag path, and finishes with a
/// callback-taking call whose result is returned.
///
/// Float compares mirror the original comiss+branch pairs exactly: plain
/// Rust `>`/`>=` already match the ja/jae/jbe/jb NaN behaviour as used here
/// (unordered keeps the current value in every case). Early exit returns a
/// fixed value: the original returns entry-EAX residue there, which is
/// caller state rather than function behaviour, so contract B checks
/// everything except the return channel on that path.
export!(thiscall, rw_cd5730(obj: *mut u8, arg0: *mut u8, arg1: u32) -> u32 {
    unsafe {
        // Field shorthands: the object layouts are game-defined; offsets are
        // the original's.
        let r32 = |p: *const u8, off: usize| *(p.add(off) as *const u32);
        let rf = |p: *const u8, off: usize| f32::from_bits(*(p.add(off) as *const u32));
        if *obj.add(0x21) == 0 {
            // Early exit: the original returns entry-EAX residue here, which
            // is caller state, not function behaviour. No fixed rewrite value
            // can match it, so contract B checks everything except the return
            // channel on this path. Return a fixed value.
            return 0;
        }
        let ebx = r32(arg0, 0x78);
        // Scratch float pair. The original keeps these in its own incoming
        // arg slot and save area (clobbered, hence stack:false); only the
        // values matter.
        let mut slot_a: f32 = 0.0;
        let mut slot_b: f32 = 0.0;
        let edi = callee_thiscall!(1, u32, ebx, r32(obj, 0x34), r32(obj, 0x38)) as *mut u8;
        let mut al: u8 = 0;
        if !edi.is_null() {
            if ((r32(edi, 4) >> 15) & 1) == 0 {
                al = 1;
            } else {
                let f = rf(edi, 0x4c);
                let c = f32::from_bits(*global::<u32>(0xFE88DC));
                // comiss f,c + jae: taken (al stays 0) iff f >= c ordered.
                if f >= c {
                    al = 0;
                } else {
                    al = 1;
                }
            }
        }
        if al != 0 {
            slot_a = callee_thiscall!(2, f32, edi as u32);
            slot_b = callee_thiscall!(3, f32, edi as u32);
            *(obj.add(0x24) as *mut u32) = edi as u32;
            *(edi.add(4) as *mut u32) = r32(edi, 4) | 0x10;
            let f = rf(obj, 0x4c);
            callee_thiscall!(4, u32, r32(obj, 0x24), f.to_bits());
        } else {
            let edi2 = callee_thiscall!(5, u32, ebx, 3, 0) as *mut u8;
            if !edi2.is_null() {
                slot_a = callee_thiscall!(2, f32, edi2 as u32);
                slot_b = callee_thiscall!(3, f32, edi2 as u32);
            }
            let f = rf(obj, 0x4c);
            let r = callee_thiscall!(
                6, u32, ebx, r32(obj, 0x34), r32(obj, 0x38), f.to_bits(), 0xFFFF_FFFF
            );
            *(obj.add(0x24) as *mut u32) = r;
        }
        match r32(obj, 0x3c) {
            0 => slot_a = rf(obj, 0x40),
            1 => slot_a = rf(obj, 0x40) + slot_a,
            2 => slot_a = (slot_b - slot_a) + rf(obj, 0x40),
            _ => {}
        }
        let u24 = r32(obj, 0x24) as *mut u8;
        slot_b = callee_thiscall!(3, f32, u24 as u32);
        // Clamp: if 0 > a (ordered) take 0, else if a > b take b.
        // NaN on either side keeps a, matching ja/jbe through unordered.
        let mut x0 = slot_a;
        if 0.0f32 > x0 {
            x0 = 0.0;
        } else if x0 > slot_b {
            x0 = slot_b;
        }
        callee_thiscall!(7, u32, u24 as u32, x0.to_bits());
        let mut x1 = rf(obj, 0x44);
        if *arg0.add(0x219) != 0 {
            x1 = rf(obj, 0x48) * x1;
            if (arg1 & 0xFF) as u8 != 0 {
                x1 = f32::from_bits(*global::<u32>(0x1051998)) * x1;
            }
        }
        *(u24.add(0x54) as *mut u32) = x1.to_bits();
        if *obj.add(0x5c) & 1 == 0 {
            if ((r32(u24, 4) >> 6) & 1) != 0 {
                *(u24.add(4) as *mut u32) = r32(u24, 4) & 0xFFFF_FFBF;
            }
            if ((r32(u24, 4) >> 7) & 1) != 0 {
                *(u24.add(4) as *mut u32) = r32(u24, 4) & 0xFFFF_FF7F;
            }
            *(u24.add(4) as *mut u32) = r32(u24, 4) | 0x8000;
            *(u24.add(4) as *mut u32) = r32(u24, 4) | 0x4000;
        } else {
            *(u24.add(4) as *mut u32) = r32(u24, 4) | 0x40;
            if ((r32(u24, 4) >> 7) & 1) != 0 {
                *(u24.add(4) as *mut u32) = r32(u24, 4) & 0xFFFF_FF7F;
            }
            if ((r32(u24, 4) >> 15) & 1) != 0 {
                *(u24.add(4) as *mut u32) = r32(u24, 4) & 0xFFFF_7FFF;
            }
            if ((r32(u24, 4) >> 14) & 1) != 0 {
                *(u24.add(4) as *mut u32) = r32(u24, 4) & 0xFFFF_BFFF;
            }
            // Set-flag path falls through into the sub-query pair; the clear
            // path jumps over it to the tail call. Each sub-query takes a
            // mode word plus a pointer to a stack out-cell (address skipped,
            // contents snapped), a zero word and a 1.0 word.
            slot_b = rf(u24, 0x4c);
            let mut cell_a: u32 = 0;
            let bl = callee_thiscall!(
                8, u32, u24 as u32, 1, &mut cell_a as *mut u32 as u32, 0, 0x3F80_0000
            ) & 0xFF;
            let mut cell_b: u32 = 0;
            let al2 = callee_thiscall!(
                8, u32, u24 as u32, 2, &mut cell_b as *mut u32 as u32, 0, 0x3F80_0000
            ) & 0xFF;
            if bl != 0 && al2 != 0 {
                // comiss b,cell + jb skips unless b >= cell ordered.
                let b = slot_b;
                let c = f32::from_bits(cell_b);
                if b >= c {
                    let a = f32::from_bits(cell_a);
                    callee_thiscall!(9, u32, u24 as u32, a.to_bits());
                }
            }
        }
        callee_thiscall!(10, u32, u24 as u32, 1, relocated(0xCD1630), obj as u32)
    }
});
