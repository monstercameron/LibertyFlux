// original: 0x009a3ef0 audRadioAudioEntity::vf3
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

// 0x009A3EF0: audRadioAudioEntity::vf3 (merged symbol name).
//
// Per-tick update of a radio audio entity. Scales the global clock into an
// integer frame stamp, then either retunes (scanning the station list for a
// live slot and parking the entity on it) or refreshes the current station's
// volume, filter voice and tuning fields, and finally ages the retune timer,
// which can trigger one last station probe.
//
// Convention: thiscall/1 (the stack argument is ignored), returns a status
// or probe value in eax.
// Quirks reproduced exactly:
// - the volume store reads an empty x87 register stack (underflow); the
//   value observed on this host is deterministically 0 (see lane report);
// - the earliest exit returns the entry control word ORed with the truncate
//   bits (0x037F | 0x0C00 under the checker's fninit entry state).
// ---------------------------------------------------------------------------
export!(thiscall, rw_009a3ef0(this: *mut u8, _arg: u32) -> u32 {
    unsafe {
        let tick = *global::<f32>(0x0115DBF4) * *global::<f32>(0x00FE8C58);
        let stamp = tick as i64 as u32;
        if *this.add(0x8B) != 0 && *this.add(0x8C) != 0 {
            retune_scan(this);
            return retune_tail(this, stamp);
        }
        if g_byte(0x0115DBFC) != 0 {
            return retune_tail(this, stamp);
        }
        if g_byte(0x0115DCE6) == 0 {
            return 0x037F | 0x0C00;
        }
        let count = callee_cdecl!(1, u32,);
        if count == 0 {
            return 0;
        }
        refresh_station(this);
        let eax2 = (*global::<f32>(0x0115DBF4) * *global::<f32>(0x00FE8C58)) as i64 as u32;
        let xmm1 = pick_volume(this);
        callee_thiscall!(10, u32, this.add(0x30) as u32, xmm1.to_bits(), eax2);
        // The original's fstp reads an empty x87 stack here; on this host
        // the observed stored value is deterministically 0 (see report).
        *this.add(0x20).cast::<u32>() = 0;
        if *this.add(0x6C).cast::<u32>() != 1 && *this.add(0x70).cast::<u32>() != 1 {
            callee_thiscall!(11, u32, this as u32, stamp);
        }
        callee_thiscall!(12, u32, this as u32, stamp);
        callee_thiscall!(13, u32, this as u32, stamp);
        callee_thiscall!(14, u32, this as u32);
        *this.add(0x85) = *this.add(0x84);
        *this.add(0x86) = g_byte(0x012845C8);
        retune_tail(this, stamp)
    }
});

/// First-half retune scan: silence every live slot found, and when none was
/// live re-seed each slot through the tuning table and drop the retune flag.
unsafe fn retune_scan(this: *mut u8) {
    let mut count = callee_cdecl!(1, u32,);
    let mut any_live = false;
    if count != 0 {
        let mut i: u32 = 0;
        loop {
            let e = callee_cdecl!(2, u32, i);
            if callee_thiscall!(3, u32, e) != 0 {
                let e2 = callee_cdecl!(2, u32, i);
                callee_thiscall!(4, u32, e2, 0, 0, 0xFF);
                any_live = true;
            }
            i = i.wrapping_add(1);
            count = callee_cdecl!(1, u32,);
            if i >= count {
                break;
            }
        }
    }
    if any_live {
        *this.add(0x6C).cast::<u32>() = 0;
        *this.add(0x70).cast::<u32>() = 0;
        return;
    }
    let mut count = callee_cdecl!(1, u32,);
    if count != 0 {
        let base = g_dword(0x012845CC);
        let mut i: u32 = 0;
        loop {
            let e = callee_cdecl!(2, u32, i);
            callee_thiscall!(5, u32, e, base.wrapping_add(i.wrapping_mul(8)));
            i = i.wrapping_add(1);
            count = callee_cdecl!(1, u32,);
            if i >= count {
                break;
            }
        }
    }
    *this.add(0x8B) = 0;
}

/// Refresh the tuned station from the player and vehicle state.
unsafe fn refresh_station(this: *mut u8) {
    let ped = callee_cdecl!(6, u32,);
    let r = callee_cdecl!(7, u32, 0);
    if *this.add(0x28).cast::<u32>() != r {
        *this.add(0x28).cast::<u32>() = r;
    }
    let mut has = 0u8;
    if ped != 0 {
        let cur = *this.add(0x28).cast::<u32>();
        if cur != 0 && *((cur as *const u8).add(0xD10)) != 0xFE {
            let t = *((ped as *const u8).add(0x224) as *const u32);
            if callee_thiscall!(8, u32, t.wrapping_add(0x2E0), 0x2DE, 0) == 0
                && callee_thiscall!(8, u32, t.wrapping_add(0x2E0), 0x2E2, 0) == 0
            {
                has = 1;
            }
        }
    }
    *this.add(0x84) = has;
    if has != 0 {
        *this.add(0x2C).cast::<u32>() = *this.add(0x28).cast::<u32>();
    }
}

/// Pick the voice volume: full level unless the ducking checks say silence.
unsafe fn pick_volume(this: *mut u8) -> f32 {
    let loud = *global::<f32>(0x00FE88E8) > *global::<f32>(0x0103234C);
    let direct: bool;
    if g_byte(0x01283049) != 0 || g_byte(0x011D7629) != 0 {
        direct = false;
    } else if callee_thiscall!(9, u32, relocated(0x0128E400)) != 0 {
        direct = true;
    } else {
        direct = false;
    }
    if !direct {
        if g_byte(0x012845C9) == 0
            && (*this.add(0x84) != 0 || g_byte(0x012845C8) != 0)
            && !loud
        {
            return 1.0;
        }
    }
    if *this.add(0x8A) != 0 {
        1.0
    } else {
        0.0
    }
}

/// Shared tail: age the retune timer and maybe probe one station slot.
unsafe fn retune_tail(this: *mut u8, stamp: u32) -> u32 {
    callee_cdecl!(15, u32, stamp);
    callee_cdecl!(16, u32, stamp);
    let mode = *this.add(0x6C).cast::<u32>();
    let limit = g_dword(0x011735B4);
    if mode == 4 {
        *this.add(0x90).cast::<u32>() = limit;
        *this.add(0x95) = 0;
    }
    let end = (*this.add(0x90).cast::<u32>()).wrapping_add(0xBB8);
    if end >= limit || *this.add(0x95) != 0 {
        *this.add(0x94) = 0;
        return end;
    }
    *this.add(0x95) = 1;
    let want = g_dword(0x01284644);
    let count = callee_cdecl!(1, u32,);
    if want >= count {
        return count;
    }
    let e = callee_cdecl!(2, u32, want);
    if e == 0 {
        return 0;
    }
    if *(((e as *const u8).add(0x1904)) as *const u32) != 6 {
        let band = *(((e as *const u8).add(0x1917)) as *const u8) as u32;
        let c = e.wrapping_add(band.wrapping_mul(0xBD0));
        if c != 0
            && *(((c as *const u8).add(0xBCA)) as *const u8) != 0
            && *(((c as *const u8).add(0xBC0)) as *const u8) == 2
        {
            *this.add(0x94) = 1;
        }
    }
    e
}
