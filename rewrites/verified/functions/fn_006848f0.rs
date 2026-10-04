// original: 0x006848f0 indexed_word_fetch
/// Indexed word fetch from a strided table.
///
/// Reads the flag word at +0x1c: when bit 1 is clear, stores the index
/// itself into the output slot. Otherwise bounds-checks the index against
/// the u16 limit at +0x14 (out of range returns 0 and writes nothing) and
/// copies the u16 at table[index * 0xE0 + 0x16] into the output slot,
/// returning 1. The full EAX value on each path matches the original.
export!(thiscall, rw_006848f0(this_: *const u8, index: u32, out: *mut u16) -> u32 {
    unsafe {
        const STRIDE: usize = 0xE0;
        const ENTRY_OFF: usize = 0x16;
        let idx = (index & 0xFFFF) as u16;
        let flags = *(this_.add(0x1C) as *const u32);
        let hi = (flags >> 1) & 0xFFFF_0000;
        if flags & 0x02 == 0 {
            *out = idx;
            return hi | ((idx as u32) & 0xFF00) | 1;
        }
        let bound = *(this_.add(0x14) as *const u16);
        if idx >= bound {
            return hi | ((idx as u32) & 0xFF00);
        }
        let table = *(this_ as *const u32) as *const u8;
        *out = *(table.add(idx as usize * STRIDE + ENTRY_OFF) as *const u16);
        // The original reloads EAX with the output pointer before returning,
        // so EAX carries the output address here, not the table address.
        (out as u32 & 0xFFFF_FF00) | 1
    }
});
