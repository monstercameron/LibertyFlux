// original: 0x00892B70 audsound_fixup_parity_entries
/// Rebases three words of one of two entries per index, by index parity.
///
/// Runs `i` from 0 while `i` is below the unsigned count at `this+0x50`. For
/// each `i` it picks the entry pointer at `this+0x80` or `this+0x84` by bit 0
/// of `base + i` (`base` is the dword at `this+0x58`), then adds the pointer
/// to itself into the three dwords at offsets 0, 8 and 0x10 past it (wrapping).
/// Returns the count.
/// Original: 0x00892B70 (thiscall, no stack arguments).
export!(thiscall, rw_00892B70(this: *mut u8) -> u32 {
    unsafe {
        const COUNT: usize = 0x50;
        const BASE: usize = 0x58;
        const SLOTS: usize = 0x80;
        let count = *(this.add(COUNT) as *const u32);
        let base = *(this.add(BASE) as *const u32);
        let mut i = 0u32;
        while count > i {
            let parity = base.wrapping_add(i) & 1;
            let p = *(this.add(SLOTS + (parity as usize) * 4) as *const u32);
            for off in [0u32, 8, 0x10] {
                let at = p.wrapping_add(off);
                let v = (at as *const u32).read_unaligned();
                (at as *mut u32).write_unaligned(v.wrapping_add(p));
            }
            i = i.wrapping_add(1);
        }
        i
    }
});
