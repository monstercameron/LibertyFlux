// original: 0x009a42e0 EP1_INTRO_MUSIC_TRACK
/// Read a global dword at a file VA.
#[inline(always)]
unsafe fn g_dword(file_va: u32) -> u32 {
    *global::<u32>(file_va)
}

/// Read a global byte at a file VA.
#[inline(always)]
unsafe fn g_byte(file_va: u32) -> u8 {
    *global::<u8>(file_va)
}

// 0x009A42E0: intro-music-track picker (merged hint name
// "EP1_INTRO_MUSIC_TRACK", after the hashed track-name string).
//
// Chooses which music track the intro sequence should play. After a cached
// hash lookup that can bail out early with "none" (0xFF), it validates the
// candidate vehicle occupant, resolves the occupant's band preference
// through the station table, and falls back through the picker helper plus
// two scripted queries until a playable track id turns up.
//
// Convention: thiscall/1, returns the track id byte in al (0xFF = none).
// The upper 24 bits of eax are stale callee answers, so the contract
// compares al only.
// ---------------------------------------------------------------------------
export!(thiscall, rw_009a42e0(this: *mut u8, occupant: u32) -> u32 {
    unsafe {
        let init = g_dword(0x01284680);
        if init & 1 == 0 {
            let h = callee_cdecl!(1, u32, relocated(0x00E912A8), 0);
            *global::<u32>(0x01284680) = init | 1;
            *global::<u32>(0x0128467C) = h;
        }
        if g_dword(0x01288534) == g_dword(0x0128467C) {
            return 0xFF;
        }
        let mut track: u32 = 0xFF;
        if *this.add(0x70).cast::<u32>() == 2
            && callee_thiscall!(2, u32, occupant) != 0
        {
            let st = *this.add(0x74).cast::<u32>();
            if st != 0xFE {
                return callee_cdecl!(3, u32, st, occupant.wrapping_add(0xA20), 2);
            }
        }
        if occupant == 0 {
            return 0xFF;
        }
        if callee_cdecl!(4, u32,) == 0 {
            return 0xFF;
        }
        if callee_cdecl!(5, u32, 0) != 0 {
            return 0xFF;
        }
        if callee_thiscall!(2, u32, occupant) == 0 {
            if *((occupant as *const u8).add(0xD18) as *const u32) == 0 {
                return 0xFF;
            }
            if callee_cdecl!(6, u32, 0x3F000000) == 0 {
                return 0xFF;
            }
        }
        let band = *((occupant as *const u8).add(0xD11));
        if band != 0xFE {
            let want = band as u32;
            let count = callee_cdecl!(4, u32,);
            let mut probe = true;
            if want < count {
                let e = callee_cdecl!(10, u32, want);
                if *(((e as *const u8).add(0x1927)) as *const u8) != 0 {
                    probe = false;
                }
            }
            if probe {
                track = callee_cdecl!(3, u32, want, occupant.wrapping_add(0xA20), 2);
            }
        }
        if callee_thiscall!(2, u32, occupant) != 0 {
            return track & 0xFF;
        }
        if track != 0xFF {
            return track & 0xFF;
        }
        let mut alt: u32 = 0x0D;
        if *((occupant as *const u8).add(0xF50) as *const u32) != 0 {
            let mut out0: u32 = 0;
            let mut out1: u32 = 0;
            callee_stdcall!(7, u32,
                &mut out0 as *mut u32 as u32,
                &mut out1 as *mut u32 as u32);
            track = if out0 == 0xFFFFFFFF { 0x0D } else { out0 };
            if out1 != 0xFFFFFFFF {
                alt = out1;
            }
        } else {
            track = *((occupant as *const u8).add(0xD18) as *const u32);
        }
        // The candidate id selects the query, but the query's out-word
        // is what lands back in the track slot (both share one frame
        // slot in the original).
        let f = callee_thiscall!(8, u32, this as u32, track);
        let mut w: u32 = 0;
        callee_cdecl!(9, u32, &mut w as *mut u32 as u32, f);
        track = w;
        if track < callee_cdecl!(4, u32,) {
            track = callee_cdecl!(3, u32, track, occupant.wrapping_add(0xA20), 2);
        }
        if track != 0xFF {
            return track & 0xFF;
        }
        if alt == 0 {
            return track & 0xFF;
        }
        let f2 = callee_thiscall!(8, u32, this as u32, alt);
        let mut w2: u32 = 0;
        callee_cdecl!(9, u32, &mut w2 as *mut u32 as u32, f2);
        track = w2;
        if track != 0xFF {
            track = callee_cdecl!(3, u32, track, occupant.wrapping_add(0xA20), 2);
        }
        track & 0xFF
    }
});
