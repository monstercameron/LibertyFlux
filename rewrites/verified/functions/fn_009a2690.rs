// original: 0x009a2690 keyed_audio_mode_selector
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

/// Read one dword from an in-image pointer table at a signed dword index,
/// exactly like `(an instruction of the original)`.
#[inline(always)]
unsafe fn table_entry(table_va: u32, idx: i32) -> u32 {
    let base = relocated(table_va);
    let addr = base.wrapping_add((idx as u32).wrapping_mul(4));
    *(addr as *const u32)
}

// ---------------------------------------------------------------------------
// 0x009A2690: keyed audio-mode selector (proposed name).
//
// Dispatches on the key in the first stack argument, comparing it against a
// chain of global selector values. Most arms probe one audio station slot
// through a shared helper call and answer one of two globals depending on
// the outcome; two arms answer from small tables instead.
//
// Convention: thiscall/2, ECX = owner object, returns the selected value.
// Quirk: on probe arms the original also zeroes the low byte of its own
// incoming key slot (caller stack above ESP). A Rust rewrite cannot address
// that slot, so the contract compares everything except the incoming stack
// (stack:false); return value, calls and all other state are fully checked.
// ---------------------------------------------------------------------------
export!(thiscall, rw_009a2690(this: *mut u8, key: u32, aux: u32) -> u32 {
    unsafe {
        // Arm 0: direct value, or a flag-table lookup on the owner's object.
        if key == g_dword(0x01284530) {
            let obj = *((this.add(8)) as *const u32);
            let settled = *((obj as *const u8).add(0x218));
            let armed = *((obj as *const u8).add(0x219));
            if settled == 0 && armed != 0 && callee_cdecl!(1, u32,) == 0 {
                return g_dword(0x01284394);
            }
            let idx = *(((obj as *const u8).add(0x2E)) as *const i16) as i32;
            let row = table_entry(0x01295CD8, idx);
            let flags = *(((row as *const u8).add(0x120)) as *const u32);
            if flags & 2 == 0 {
                return g_dword(0x01284434);
            }
            return g_dword(0x01284458);
        }
        // Arms 1-3: probe one station slot; same shape, different globals.
        if key == g_dword(0x01284438) {
            return probe_station_slot(aux, 0x0128445C, 0x01284444);
        }
        if key == g_dword(0x012844E8) {
            return probe_station_slot(aux, 0x01284588, 0x01284418);
        }
        if key == g_dword(0x0128449C) {
            return probe_station_slot(aux, 0x01284420, 0x0128459C);
        }
        // Arm 4: the probed slot is picked by a small mode switch.
        if key == g_dword(0x0128440C) {
            // Note: the jump table permutes the cases (0 and 3 swap
            // bodies with the default arm); the order below is read from
            // the table, not from the code layout.
            let pick = match g_dword(0x01295868) {
                1 => g_dword(0x01284580),
                2 => g_dword(0x012844A0),
                3 => g_dword(0x01284430),
                _ => g_dword(0x012844A4),
            };
            let mut slot_a: u32 = 0;
            let mut slot_b: u32 = 0;
            let r = callee_stdcall!(2, u32, pick, aux,
                &mut slot_a as *mut u32 as u32,
                &mut slot_b as *mut u32 as u32, 0) as i32;
            if r < 0 {
                return g_dword(0x012844A4);
            }
            return pick;
        }
        // Arm 5: two-way pick between neighbouring globals.
        if key == g_dword(0x01284400) {
            let v = g_dword(0x0129586C);
            if v == 0 {
                return g_dword(0x01284568);
            }
            if v == 1 {
                return g_dword(0x01284460);
            }
            return g_dword(0x01284568);
        }
        // No arm matched: the key passes through unchanged.
        key
    }
});

/// Shared body of arms 1-3: look at the station table through the global
/// index, and when the slot looks live (or the force flag is set) run the
/// shared probe call. Answers `ok_va` on probe success, `alt_va` otherwise.
unsafe fn probe_station_slot(aux: u32, alt_va: u32, ok_va: u32) -> u32 {
    let alt = g_dword(alt_va);
    let idx = g_dword(0x01036F14) as i32;
    let row = if idx == -1 {
        0
    } else {
        table_entry(0x011A8808, idx)
    };
    let stamp = *(((row as *const u8).add(0x568)) as *const u32);
    if stamp <= g_dword(0x011735B4) && g_byte(0x01284384) == 0 {
        return alt;
    }
    let mut slot_a: u32 = 0;
    let mut slot_b: u32 = 0;
    let r = callee_stdcall!(2, u32, g_dword(ok_va), aux,
        &mut slot_a as *mut u32 as u32,
        &mut slot_b as *mut u32 as u32, 0) as i32;
    if r < 0 {
        alt
    } else {
        g_dword(ok_va)
    }
}
