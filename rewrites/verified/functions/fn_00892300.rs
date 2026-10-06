// original: 0x00892300 audsound_collect_voice_slots
/// Collects the voice into the slot array, then recurses into sub-slots.
///
/// When bit 7 of `this+0x39` is set and the word at `this+6` is 2, returns
/// at once (entry EAX, pinned 0 by the contract). When `group` (arg2) is
/// null, returns at once likewise. When the byte at `this+0x3b` equals `key`
/// (arg1), scans the first `n` (arg3, SIGNED: non-positive skips) dwords of
/// `group` for the first zero slot and stores `this` there. Unless `flags`
/// (arg4, low byte) is set or bit 2 of `this+0x3a` is clear, returns here.
/// Otherwise visits the eight bytes at `this+0x48`: a 0xff byte is skipped,
/// any other resolves through `stride * byte + table[idx * 0x6f40 + 0x6f10]`
/// and, when nonzero, recurses (thiscall: resolved object, key, group, n,
/// flags) with the recursion stubbed by the checker (the call sites and
/// arguments are verified, not the nested depth). Returns the last
/// recursion's answer, the scan result, or the slot byte as the paths leave
/// it (entry EAX, pinned 0, on the two early exits).
/// Original: 0x00892300 (thiscall, four stack words: key, group, n, flags).
export!(thiscall, rw_00892300(this: *mut u8, key: u32, group: u32, n: i32, flags: u32) -> u32 {
    unsafe {
        const SELF_CALL: u32 = 1;
        const SLOTS: usize = 0x48;
        const NSLOTS: u32 = 8;
        const INDEX: usize = 0x40;
        const ROW: u32 = 0x6f40;
        const COL: u32 = 0x6f10;
        const EMPTY: u8 = 0xff;
        const STRIDE_G: u32 = 0x115d964;
        const TABLE_G: u32 = 0x115d988;
        if *this.add(0x39) & 0x80 != 0 && *(this.add(6) as *const u16) == 2 {
            return 0;
        }
        if group == 0 {
            return 0;
        }
        let sb = *this.add(0x3b);
        let mut eax = sb as u32;
        if sb as u32 == key {
            eax = 0;
            if n > 0 {
                let mut i = 0i32;
                while i < n {
                    let slot = (group.wrapping_add((i as u32).wrapping_mul(4))) as *mut u32;
                    if slot.read_unaligned() == 0 {
                        slot.write_unaligned(this as u32);
                        eax = i as u32;
                        break;
                    }
                    i = i.wrapping_add(1);
                    eax = i as u32;
                }
            }
        }
        if (flags as u8) == 0 && *this.add(0x3a) & 4 != 0 {
            return eax;
        }
        let stride = *global::<u32>(STRIDE_G);
        let table = *global::<u32>(TABLE_G);
        let idx = *this.add(INDEX) as u32;
        let base = *((table.wrapping_add(idx.wrapping_mul(ROW)).wrapping_add(COL)) as *const u32);
        let mut s = 0u32;
        while (s as i32) < NSLOTS as i32 {
            let b = *this.add(SLOTS + s as usize);
            if b != EMPTY {
                let obj = stride.wrapping_mul(b as u32).wrapping_add(base);
                if obj != 0 {
                    eax = callee_thiscall!(SELF_CALL, u32, obj, key, group, n as u32, flags);
                }
            }
            s = s.wrapping_add(1);
        }
        eax
    }
});
