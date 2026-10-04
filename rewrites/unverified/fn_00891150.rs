// original: 0x00891150 
// 00891150 audSound retune: detach the old pool owner, then reprogram the
// slot entry with the new tone and tag.
export!(thiscall, rw_00891150(this: *mut u8, tag: u32, opts: u32, delta: u32) -> () {
    unsafe {
        let base = *global::<u32>(0x115D988);
        let stride = *global::<u32>(0x115D968);
        if (*this.add(0x39) & 1) == 0 {
            let sel = *this.add(4);
            let pool = if sel == 0xFF {
                0u32
            } else {
                let row = (*this.add(0x40) as u32).wrapping_mul(0x6F40);
                let entry =
                    *((row.wrapping_add(base).wrapping_add(0x6F14)) as *const u32);
                stride.wrapping_mul(sel as u32).wrapping_add(entry)
            };
            let owner = *(this.add(0x74) as *const u32);
            if owner != 0 {
                let vtable = *(owner as *const u32);
                let target = *((vtable as *const u8).add(4) as *const u32);
                let drop_old: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                let prev = drop_old(owner, 0);
                callee_thiscall!(2, u32, pool, prev);
            }
            callee_thiscall!(3, u32, pool, 1);
        }
        if (*this.add(0x38) & 0x80) != 0 {
            return;
        }
        let tone = callee_cdecl!(4, u32, tag) as u16;
        let mut label = 0xFFFF_FFFFu32;
        if delta != 0xFFFF_FFFF {
            label = callee_thiscall!(5, u32, relocated(0x115D9A0)).wrapping_add(delta);
        }
        let sel = *this.add(4);
        let pool = if sel == 0xFF {
            0u32
        } else {
            let row = (*this.add(0x40) as u32).wrapping_mul(0x6F40);
            let entry = *((row.wrapping_add(base).wrapping_add(0x6F14)) as *const u32);
            stride.wrapping_mul(sel as u32).wrapping_add(entry)
        };
        let mut mix = (opts as u8) << 2;
        mix ^= *(pool as *const u8).add(0xE8);
        *(pool as *mut u16).add(0xE4 / 2) = tone;
        mix &= 4;
        *(pool as *mut u8).add(0xE8) ^= mix;
        *(pool as *mut u8).add(0xE7) |= 0x40;
        *(pool as *mut u32).add(0xDC / 4) = label;
    }
});
