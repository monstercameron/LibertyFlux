// original: 0x00a94fa0 stream_alloc_entry_indexed (proposed)

/// Allocate and initialise a streaming entry selected by a table lookup.
///
/// The third argument picks a word from a global word table (byte offset
/// `a2*100`); adding the second argument's low half gives the entry index.
/// The entry is `table[index*3*8]` where `table` is at `this+0x00`. When bit
/// 11 of its flag word is set the index is returned as-is (already live).
/// Otherwise the entry must be fully zero in its key dword (`+0x08` masked
/// with 0xfffffffc) and flag bit 11; anything set means no free entry and
/// 0xffff is returned. A free entry is zeroed in its flag word, ORed with
/// 0x0800, packed through callee 1 (thiscall/2 with 0, 0), stamped with the
/// third argument's low byte at `+0x16`/`+0x17`, with 0xff at `+0x04` and
/// 0xffff at `+0x14`, and its index is returned.
///
/// The first argument is unread. Thiscall: object in ecx, three stack
/// words, callee pops 12.
lf_checker_rt::export!(thiscall, rw_00a94fa0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x00;
        const KEY_OFF: u32 = 0x08;
        const KEY_MASK: u32 = 0xfffffffc;
        const FLAGS_OFF: u32 = 0x0e;
        const LIVE_BIT: u16 = 11;
        const LIVE_OR: u16 = 0x0800;
        const WORD_TABLE: u32 = 0x013053a8;
        const TABLE_SCALE: u32 = 100;
        const NO_ENTRY: u32 = 0xffff;
        const STAMP_BYTE: u8 = 0xff;
        const PACK_CALLEE: u32 = 1;
        let _ = a0;
        let pick = ((lf_checker_rt::relocated(WORD_TABLE) + a2.wrapping_mul(TABLE_SCALE))
            as *const u16)
            .read_unaligned();
        let index = pick.wrapping_add(a1 as u16) as u32;
        let elem = ((this + TABLE) as *const u32).read_unaligned()
            + index.wrapping_mul(3).wrapping_mul(8);
        if ((((elem + FLAGS_OFF) as *const u16).read_unaligned() >> LIVE_BIT) & 1) != 0 {
            return index;
        }
        if ((elem + KEY_OFF) as *const u32).read_unaligned() & KEY_MASK != 0 {
            return NO_ENTRY;
        }
        if ((((elem + FLAGS_OFF) as *const u16).read_unaligned() >> LIVE_BIT) & 1) != 0 {
            return NO_ENTRY;
        }
        ((elem + FLAGS_OFF) as *mut u16).write_unaligned(0);
        let flags = (elem + FLAGS_OFF) as *mut u16;
        flags.write_unaligned(flags.read_unaligned() | LIVE_OR);
        lf_checker_rt::callee_thiscall!(PACK_CALLEE, u32, elem, 0, 0);
        let stamp = a2 as u8;
        ((elem + 0x17) as *mut u8).write(stamp);
        ((elem + 0x16) as *mut u8).write(stamp);
        ((elem + 0x04) as *mut u8).write(STAMP_BYTE);
        ((elem + 0x14) as *mut u16).write_unaligned(NO_ENTRY as u16);
        index
    }
});
