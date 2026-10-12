// original: 0x00515F50 leaderboard_row_exists
///
/// Report whether the table holds a row matching `key`.
///
/// `this` points to the table: a byte count at +5 and an entry array at
/// +0x14, each entry 0x10 bytes. `key` points to the sought row. An entry
/// matches when its type byte at +8 equals the key's and is exactly 3, both
/// the entry's and the key's first two words OR to nonzero, the sub-type
/// bytes at +9 agree, and words at +0 and +4 agree. The comparisons on bytes
/// are unsigned. The first match wins.
///
/// The returned full word follows the original's register: the accumulator
/// starts at the count, each visited entry's type byte is written into its
/// low byte, a type match replaces it with the entry words' OR, then with
/// the key words' OR, then with that OR's high bytes plus the entry sub-type
/// byte; a match returns that with the low byte forced to 1, while running
/// out of entries returns it with the low byte forced to 0.
///
/// thiscall: the table pointer arrives in ECX, the key pointer on the stack.
lf_checker_rt::export!(thiscall, rw_00515f50(this: u32, key: u32) -> u32 {
    const COUNT_AT: u32 = 5;
    const ENTRIES_AT: u32 = 0x14;
    const ENTRY_STRIDE: u32 = 0x10;
    const TYPE_AT: u32 = 8;
    const SUB_AT: u32 = 9;
    const WANTED_TYPE: u8 = 3;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let count = read_u8(this.wrapping_add(COUNT_AT)) as u32;
        let mut eax = count;
        let entries = read_u32(this.wrapping_add(ENTRIES_AT));
        let key_w0 = read_u32(key);
        let key_w4 = read_u32(key.wrapping_add(4));
        let key_type = read_u8(key.wrapping_add(TYPE_AT));
        let key_sub = read_u8(key.wrapping_add(SUB_AT));
        let mut i = 0u32;
        while i < count {
            let entry = entries.wrapping_add(i.wrapping_mul(ENTRY_STRIDE));
            let entry_type = read_u8(entry.wrapping_add(TYPE_AT));
            eax = (eax & 0xFFFF_FF00) | entry_type as u32;
            i += 1;
            if entry_type != key_type {
                continue;
            }
            if entry_type < 1 {
                continue;
            }
            if entry_type != WANTED_TYPE {
                continue;
            }
            eax = read_u32(entry) | read_u32(entry.wrapping_add(4));
            if eax == 0 {
                continue;
            }
            if key_type < 1 {
                continue;
            }
            if key_type != WANTED_TYPE {
                continue;
            }
            eax = key_w0 | key_w4;
            if eax == 0 {
                continue;
            }
            let entry_sub = read_u8(entry.wrapping_add(SUB_AT));
            eax = (eax & 0xFFFF_FF00) | entry_sub as u32;
            if entry_sub != key_sub {
                continue;
            }
            if read_u32(entry) != key_w0 {
                continue;
            }
            if read_u32(entry.wrapping_add(4)) != key_w4 {
                continue;
            }
            return (eax & 0xFFFF_FF00) | 1;
        }
        eax & 0xFFFF_FF00
    }
});
