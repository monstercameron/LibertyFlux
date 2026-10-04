// original: 0x00d1da30 cover_task_update
// Advances a cover task against a pedestrian: bails out when the ped has
// no cover slot, is busy, or the reachability probe fails; otherwise
// scores the slot, keeps the score when the validator accepts it (else
// records -1), and on acceptance stores the scaled fallback distance.
// Returns nothing meaningful.
export!(thiscall, rw_00d1da30(this_ptr: u32, ped: u32, fbits: u32, extra: u32) -> u32 {
    unsafe {
        let slot = *((ped.wrapping_add(0xD68)) as *const u32);
        if slot == 0 {
            return 0;
        }
        if *((ped.wrapping_add(0x211)) as *const u8) != 0 {
            return 0;
        }
        let mut buf = [0u32; 3];
        callee_thiscall!(1, u32, slot, buf.as_mut_ptr() as u32, 0);
        let ok = callee_cdecl!(
            2, u32, slot,
            this_ptr.wrapping_add(0x20), buf.as_mut_ptr() as u32
        );
        if (ok & 0xFF) == 0 {
            return 0;
        }
        let kind = *(slot as *const u32);
        let sel = ((((kind >> 3) & 3) != 3) as u32);
        let score = callee_cdecl!(3, u32, ped, fbits, extra, sel);
        *((this_ptr.wrapping_add(0xB4)) as *mut u32) = score;
        let chk = callee_cdecl!(4, u32, 0x2A, score);
        if chk == 0 {
            *((this_ptr.wrapping_add(0xB4)) as *mut u32) = 0xFFFF_FFFF;
            return 0;
        }
        let back = callee_cdecl!(
            5, u32, buf.as_mut_ptr() as u32, 0x3F80_0000, 0x2A,
            *((this_ptr.wrapping_add(0xB4)) as *const u32)
        );
        let v = f32::from_bits(*((back.wrapping_add(4)) as *const u32));
        let g = f32::from_bits(*(global::<u32>(0x010545A4)));
        *((this_ptr.wrapping_add(0xB8)) as *mut u32) = (v * g).to_bits();
        0
    }
});
