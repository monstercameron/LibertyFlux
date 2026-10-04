// original: 0x00891070 
// 00891070 audSound start: gate on the state flags, resolve the tag, then
// program the slot pool entry. Returns the tag, or 1/2 for the early exits.
export!(thiscall, rw_00891070(this: *mut u8, tag: u32, opts: u32) -> u32 {
    unsafe {
        if (*this.add(0x39) & 1) != 0 {
            return 2;
        }
        if (*this.add(0x38) & 0x80) != 0 {
            return 1;
        }
        let mut live = tag;
        if live == 0 {
            live = callee_cdecl!(1, u32, *(this.add(0x3C) as *const i16) as u32);
        }
        if callee_thiscall!(2, u32, this as u32) & 0xFF != 0 {
            live = 1;
        } else {
            let sel = *this.add(0x3B);
            let target = *global::<u32>(0x115D654 + (sel as u32) * 4);
            let resolve: extern "cdecl" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            live = resolve(this as u32, live, 1);
            if live == 2 {
                return 2;
            }
        }
        let tone = callee_cdecl!(4, u32, tag) as u16;
        let sel = *this.add(4);
        let pool = if sel == 0xFF {
            0u32
        } else {
            let row = (*this.add(0x40) as u32).wrapping_mul(0x6F40);
            let base = *global::<u32>(0x115D988);
            let entry = *((row.wrapping_add(base).wrapping_add(0x6F14)) as *const u32);
            (*global::<u32>(0x115D968)).wrapping_mul(sel as u32).wrapping_add(entry)
        };
        let mut mix = (opts as u8) << 2;
        mix ^= *(pool as *const u8).add(0xE8);
        *(pool as *mut u16).add(0xE4 / 2) = tone;
        mix &= 4;
        *(pool as *mut u8).add(0xE8) ^= mix;
        *(pool as *mut u8).add(0xE7) |= 0x40;
        *(pool as *mut u32).add(0xDC / 4) = 0xFFFF_FFFF;
        live
    }
});
