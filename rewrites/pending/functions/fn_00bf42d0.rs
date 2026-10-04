// original: 0x00bf42d0 attach_entity_record (proposed name)
/// Attach an entity's record row to its live session object.
///
/// `this_` carries the session key (dword at +8, weight float at +0x10,
/// control flags at +0x14), `entity` the entity. A session lookup runs
/// first; a null answer returns zero. Otherwise the entity's live record
/// is fetched through two virtual calls (falling back to its stored
/// record when the first answers null), an index helper maps the record
/// to a slot, and the row address is the slot scaled plus the record's
/// base. A null row returns zero; otherwise the row is pushed into the
/// session, an optional weight call runs, the session is flushed, and a
/// generation counter is stored back (bumped when it agrees with a
/// global). Returns the stored counter, zero when the session lookup
/// fails, or the record pointer when its row is null.
export!(thiscall, rw_00bf42d0(this_: *mut u8, entity: *mut u8) -> u32 {
    unsafe {
        // These immediates have no reloc entries, yet the worker's log
        // shows the original passing relocated addresses; match that.
        let session_root = relocated(0x01394D60);
        let weight_tag = relocated(0x00EBB934);
        let key = *(this_.add(8) as *const u32);
        let bx = callee_thiscall!(1, u32, session_root, key, 0, 0);
        if bx == 0 {
            return 0;
        }
        let flags = *(this_.add(0x14) as *const u8);
        let vt = *(entity as *const u32);
        let slot1 = ((vt + 0xa0) as *const u32).read();
        let vcall1: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot1 as usize);
        let fetch = |vcall1: extern "thiscall" fn(u32) -> u32| -> u32 {
            let a = vcall1(entity as u32);
            if a == 0 {
                *(entity.add(0x100) as *const u32)
            } else {
                let b = vcall1(entity as u32);
                let vt2 = *(b as *const u32);
                let slot2 = ((vt2 + 0xe0) as *const u32).read();
                let vcall2: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot2 as usize);
                vcall2(b)
            }
        };
        let e1 = fetch(vcall1);
        let kind = if (flags & 2) != 0 { 0x1a5u32 } else { 0x4b0u32 };
        let inp = *((e1 + 4) as *const u32);
        let edi = callee_cdecl!(2, u32, inp, kind);
        let e2 = fetch(vcall1);
        let row = (edi << 6).wrapping_add(*((e2 + 0x14) as *const u32));
        if row == 0 {
            // The zero test is on the row, but eax still holds e2.
            return e2;
        }
        callee_thiscall!(3, u32, bx, row);
        if (flags & 1) == 0 {
            let f = *(this_.add(0x10) as *const f32);
            callee_thiscall!(4, u32, bx, weight_tag, f.to_bits());
        }
        callee_thiscall!(5, u32, bx);
        let g = callee_cdecl!(6, u32,);
        let g1 = *global::<u32>(0x011F702C);
        let mut g2 = *global::<u32>(0x011F70C4);
        if g1 == g {
            g2 = g2.wrapping_add(1);
        }
        *((bx + 0x1d4) as *mut u32) = g2;
        g2
    }
});
