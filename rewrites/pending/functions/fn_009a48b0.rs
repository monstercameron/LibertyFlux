// original: 0x009a48b0 station_mode_stepper
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

// 0x009A48B0: station-mode stepper (proposed name).
//
// Advances a radio entity's station mode by one step: after refreshing the
// voice snapshot and (when unmuted) the intro gate, it dispatches on the
// current mode word. Modes tune up/down through the station list with
// wraparound, park on "none", or reseed from the stored position, always
// leaving the entity's slot fields consistent.
//
// Convention: thiscall/1, returns the last helper answer in eax.
// ---------------------------------------------------------------------------
export!(thiscall, rw_009a48b0(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        let mut snap: u32 = 0;
        callee_thiscall!(1, u32, &mut snap as *mut u32 as u32);
        if *this.add(0x8E) == 0
            && g_byte(0x012845C8) != 0
            && callee_cdecl!(2, u32,) == 0
            && g_byte(0x011618FA) == 0
            && g_byte(0x011D7629) == 0
            && g_byte(0x01283049) == 0
        {
            callee_thiscall!(3, u32, this as u32, 0);
        }
        if g_byte(0x012845C8) == 0 && *this.add(0x86) == 0 && *this.add(0x6C).cast::<u32>() != 0
        {
            *this.add(0x70).cast::<u32>() = 0;
        }
        // Each exit returns whatever eax holds there: usually the last
        // helper answer, sometimes a reloaded field or the argument.
        let voice = this.add(8) as u32;
        match *this.add(0x70).cast::<u32>() {
            1 => {
                let st = *this.add(0x74).cast::<u32>();
                let r = callee_cdecl!(4, u32, st, voice, 7);
                let e = callee_cdecl!(5, u32, st);
                if r == st && e != 0 && callee_thiscall!(6, u32, e) != 0 {
                    *this.add(0x70).cast::<u32>() = 2;
                } else {
                    callee_thiscall!(7, u32, this as u32);
                }
                *this.add(0x78).cast::<u32>() = st;
                st
            }
            2 => {
                callee_cdecl!(9, u32,);
                if *this.add(0x74).cast::<u32>() >= callee_cdecl!(8, u32,) {
                    *this.add(0x74).cast::<u32>() = 0;
                }
                let st = *this.add(0x74).cast::<u32>();
                *this.add(0x78).cast::<u32>() = st;
                *this.add(0x7C).cast::<u32>() = st;
                *this.add(0x1C).cast::<u32>() = arg;
                if g_dword(0x012845C4) > 0 {
                    let count = callee_cdecl!(8, u32,);
                    *this.add(0x70).cast::<u32>() = 4;
                    *this.add(0x78).cast::<u32>() =
                        st.wrapping_add(g_dword(0x012845C4)) % count;
                    return count;
                }
                if *this.add(0x80).cast::<u32>() != 0xFE {
                    let count = callee_cdecl!(8, u32,);
                    let prev = *this.add(0x80).cast::<u32>();
                    *global::<u32>(0x012845C4) =
                        count.wrapping_sub(st).wrapping_add(prev) % count;
                    *this.add(0x78).cast::<u32>() = prev;
                    *this.add(0x70).cast::<u32>() = 4;
                    *this.add(0x80).cast::<u32>() = 0xFE;
                    return prev;
                }
                arg
            }
            3 => {
                let mut slot: u32 = 0;
                callee_thiscall!(10, u32, this as u32,
                    g_dword(0x01038E5C), &mut slot as *mut u32 as u32,
                    0xFFFFFFFF, 0, 0);
                let r = callee_cdecl!(11, u32, voice);
                *this.add(0x74).cast::<u32>() = 0xFF;
                *this.add(0x78).cast::<u32>() = 0xFF;
                *this.add(0x70).cast::<u32>() = 0;
                r
            }
            4 => {
                callee_thiscall!(7, u32, this as u32);
                if g_dword(0x012845C4) > 0 {
                    let count = callee_cdecl!(8, u32,);
                    *this.add(0x78).cast::<u32>() = (*this.add(0x74).cast::<u32>())
                        .wrapping_add(g_dword(0x012845C4))
                        % count;
                    return callee_cdecl!(12, u32, voice);
                }
                let r = callee_cdecl!(13, u32, voice);
                if r == 0 {
                    *this.add(0x70).cast::<u32>() = 2;
                }
                r
            }
            _ => {
                if g_byte(0x012845C8) == 0 || *this.add(0x6C).cast::<u32>() != 0 {
                    return callee_cdecl!(11, u32, voice);
                }
                let v = *this.add(0x7C).cast::<u32>();
                *this.add(0x74).cast::<u32>() = v;
                if v == 0xFE || v == 0xFF {
                    callee_thiscall!(15, u32, this as u32);
                    *this.add(0x70).cast::<u32>() = 1;
                    if *this.add(0x68).cast::<u32>() != 0xFE {
                        let b = (*this.add(0x68)) as u32;
                        *this.add(0x68).cast::<u32>() = 0xFE;
                        *this.add(0x74).cast::<u32>() = b;
                        return b;
                    }
                    let r = callee_cdecl!(16, u32, 0,
                        callee_cdecl!(8, u32,).wrapping_sub(1))
                        & 0xFF;
                    *this.add(0x74).cast::<u32>() = r;
                    return r;
                }
                let r = callee_cdecl!(14, u32, v);
                if r == 0 && *this.add(0x8A) == 0 && g_byte(0x011D7629) == 0 {
                    let s = callee_thiscall!(15, u32, this as u32);
                    *this.add(0x70).cast::<u32>() = 1;
                    return s;
                }
                *this.add(0x70).cast::<u32>() = 1;
                r
            }
        }
    }
});
