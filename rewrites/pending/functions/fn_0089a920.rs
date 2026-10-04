// original: 0x0089a920 audio_voice_commit (proposed name)
/// Commit one audio voice's parameters and hand it to the mixer.
///
/// `obj` is the voice object, `arg1` an opaque parameter for the prepare
/// call. A bank record selected by the voice's bank/slot bytes refreshes
/// the cached levels; unless every cached level is at its rest value the
/// voice is marked dirty. After prepare, the three gain factors are
/// multiplied, remapped through a curve call, and pushed together with the
/// record into the level and route calls; the mixer tail call's answer is
/// the result. A missing record returns the prepare answer (or zero when
/// the record address itself is null).
export!(thiscall, rw_0089a920(obj: *mut u8, arg1: u32) -> u32 {
    unsafe {
        let gtable = *global::<u32>(0x0115d988);
        let stride14 = *global::<u32>(0x0115d968);
        let stride10 = *global::<u32>(0x0115d964);
        // Level-cache refresh from the 0x6f14 bank table.
        let bank = *obj.add(0x40) as u32;
        let f7 = *obj.add(0xf7) as u32;
        let row14 = *((gtable as *const u8)
            .add(bank.wrapping_mul(0x6f40).wrapping_add(0x6f14) as usize)
            as *const u32);
        let rec14 = stride14.wrapping_mul(f7).wrapping_add(row14);
        if *((rec14 as *const u8).add(0xa1)) != 0 {
            *(obj.add(0xf0) as *mut u16) = *((rec14 as *const u8).add(0x9c) as *const u16);
            *(obj.add(0xf2) as *mut u16) = *((rec14 as *const u8).add(0x9e) as *const u16);
            *obj.add(0xf4) = *((rec14 as *const u8).add(0xa0));
            *(obj.add(0xe8) as *mut u32) = *((rec14 as *const u8).add(0x90) as *const u32);
            *(obj.add(0xec) as *mut u32) = *((rec14 as *const u8).add(0x94) as *const u32);
            *obj.add(0xf8) |= 8;
        }
        let at_rest = *(obj.add(0xf0) as *const u16) == 0
            && *(obj.add(0xf2) as *const u16) == 0
            && *obj.add(0xf4) == 0x64
            && *(obj.add(0xe8) as *const u32) == 0xffffffff
            && (*(obj.add(0xec) as *const u32) as i32) <= 0
            && *(obj.add(0xd4) as *const u32) == 0
            && *(obj.add(0xd8) as *const u32) == 0
            && *(obj.add(0xdc) as *const u32) == 0
            && *(obj.add(0xe0) as *const u32) == 0
            && *(obj.add(0xe4) as *const u32) == 0;
        if !at_rest {
            *obj.add(0xf8) |= 1;
            *obj.add(0xf5) = 1;
        }
        callee_thiscall!(1, u32, obj as u32);
        let prepared = callee_thiscall!(2, u32, obj as u32, arg1);
        let slot = *obj.add(0x48) as u32;
        if slot == 0xff {
            return prepared;
        }
        // The original re-reads the bank bytes and the table base three
        // times; nothing between the reads can change them, so one lookup
        // serves all three uses.
        let bank2 = *obj.add(0x40) as u32;
        let row10 = *((gtable as *const u8)
            .add(bank2.wrapping_mul(0x6f40).wrapping_add(0x6f10) as usize)
            as *const u32);
        let rec10 = stride10.wrapping_mul(slot).wrapping_add(row10);
        if rec10 == 0 {
            return 0;
        }
        let gain = *(obj.add(0xb4) as *const f32)
            * *(obj.add(0xb0) as *const f32)
            * *(obj.add(0xb8) as *const f32);
        let curved: f32 = callee_cdecl!(3, f32, gain.to_bits());
        callee_thiscall!(4, u32, rec10, curved.to_bits());
        callee_thiscall!(5, u32, rec10, *(obj.add(0x54) as *const u32), 0);
        callee_thiscall!(6, u32, rec10)
    }
});
