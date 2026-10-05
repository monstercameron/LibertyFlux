// original: 0x008F88A0 NativeImpl_PRINT_WITH_NUMBER_BIG
/// Register one print request in the big-print table.
///
/// The table holds 0x40-byte entries indexed `slot + group * 4`. The
/// mode byte is forced to 1 when the group is 1. Unless the force
/// flag is set, the four slots of the group are scanned for a free
/// (zero head) one; a full group returns the address past it. The
/// chosen slot is then cleared through the clear routine (whose
/// answer is discarded) and filled with the three header words, a
/// fallback global and the six trailing arguments, ending with the
/// valid flag. Cdecl, eleven stack arguments (three header words,
/// group, flag byte, six trailing words). Returns the last trailing
/// word on the store path.
export!(cdecl, rw_008f88a0(h0: u32, h1: u32, h2: u32, group: u32, flag: u32,
        s0: u32, s1: u32, s2: u32, s3: u32, s4: u32, s5: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x118e7d0;
        const TABLE: u32 = 0x118dec0;
        const FALLBACK: u32 = 0x11735b4;
        const ENTRY: u32 = 0x40;
        let mut mode = *global::<u8>(MODE);
        if group == 1 {
            mode = 1;
        }
        *global::<u8>(MODE) = mode;
        let mut slot: u32 = 0;
        if (flag as u8) == 0 {
            let base = relocated(TABLE);
            let mut p = base.wrapping_add(group.wrapping_shl(8));
            let mut found = false;
            while slot < 4 {
                if (p as *const u32).read_unaligned() == 0 {
                    found = true;
                    break;
                }
                slot += 1;
                p = p.wrapping_add(ENTRY);
            }
            if !found {
                return p;
            }
        }
        let _: u32 = callee_cdecl!(1, u32, group, slot);
        let e = relocated(TABLE)
            .wrapping_add(slot.wrapping_add(group.wrapping_mul(4)).wrapping_mul(ENTRY));
        ((e) as *mut u32).write_unaligned(h0);
        ((e + 0x04) as *mut u32).write_unaligned(h1);
        ((e + 0x10) as *mut u32).write_unaligned(h2);
        ((e + 0x14) as *mut u32).write_unaligned(*global::<u32>(FALLBACK));
        ((e + 0x18) as *mut u32).write_unaligned(s0);
        ((e + 0x1c) as *mut u32).write_unaligned(s1);
        ((e + 0x20) as *mut u32).write_unaligned(s2);
        ((e + 0x24) as *mut u32).write_unaligned(s3);
        ((e + 0x28) as *mut u32).write_unaligned(s4);
        ((e + 0x2c) as *mut u32).write_unaligned(s5);
        ((e + 0x3b) as *mut u8).write(1);
        s5
    }
});
