// original: 0x008CCCD0 stream_bank_store_advance (proposed)

/// Stores a slot byte and advances the bank cursor: writes the low byte of
/// `val` to `bank + off + FLAG_BASE`, where `bank` is `TABLE[index]` (the
/// index `INVALID` (-90) and a null entry end the call at once). Then, unless
/// the signed cursor dword (`+CURSOR`) starts negative, repeatedly advances
/// it past unblocked slots: a slot is blocked when its flag byte is non-zero
/// and its 16-bit word at `bank + cursor * SLOT_STRIDE + WORD_OFF` is also
/// non-zero; the scan stops at a blocked slot or when the signed cursor
/// reaches the signed limit dword (`+LIMIT`). When the scan ends at or past
/// the limit, the cursor wraps to 0 and the scan runs once more; reaching the
/// limit again resets the cursor to 0.
///
/// The inventory size cuts the last store mid-instruction; the true size is
/// 250 bytes. Three stack arguments (cdecl); no return value.
lf_checker_rt::export!(cdecl, rw_008CCCD0(index: u32, off: u32, val: u32) -> u32 {
    unsafe {
        /// Base of the bank pointer table (one dword per index).
        const TABLE: u32 = 0x1173560;
        /// Rejected index.
        const INVALID: u32 = 0xFFFF_FFA6;
        /// Base offset of the slot flag bytes in a bank object.
        const FLAG_BASE: u32 = 0x3416;
        /// Offset of the cursor dword in a bank object.
        const CURSOR: u32 = 0x361C;
        /// Offset of the limit dword in a bank object.
        const LIMIT: u32 = 0x34F4;
        /// Stride between slot words.
        const SLOT_STRIDE: u32 = 120;
        /// Offset of the checked word inside a slot.
        const WORD_OFF: u32 = 4;
        /// One cursor scan: advances past unblocked slots, stopping at a
        /// blocked slot or at the signed limit. The table entry is re-read
        /// every iteration as the original does.
        unsafe fn scan(index: u32) {
            unsafe {
                const TABLE: u32 = 0x1173560;
                const FLAG_BASE: u32 = 0x3416;
                const CURSOR: u32 = 0x361C;
                const LIMIT: u32 = 0x34F4;
                const SLOT_STRIDE: u32 = 120;
                const WORD_OFF: u32 = 4;
                loop {
                    let bank = (lf_checker_rt::relocated(TABLE)
                        .wrapping_add(index.wrapping_mul(4)) as *const u32)
                        .read();
                    let cur = (bank.wrapping_add(CURSOR) as *const u32).read_unaligned();
                    let flag =
                        (bank.wrapping_add(cur).wrapping_add(FLAG_BASE) as *const u8).read();
                    if flag != 0 {
                        let w = (bank
                            .wrapping_add(cur.wrapping_mul(SLOT_STRIDE))
                            .wrapping_add(WORD_OFF) as *const u16)
                            .read_unaligned();
                        if w != 0 {
                            break;
                        }
                    }
                    let lim = (bank.wrapping_add(LIMIT) as *const u32).read_unaligned();
                    if (cur as i32) >= (lim as i32) {
                        break;
                    }
                    (bank.wrapping_add(CURSOR) as *mut u32).write_unaligned(cur.wrapping_add(1));
                }
            }
        }
        if index == INVALID {
            return 0;
        }
        let bank = (lf_checker_rt::relocated(TABLE).wrapping_add(index.wrapping_mul(4))
            as *const u32)
            .read();
        if bank == 0 {
            return 0;
        }
        (bank.wrapping_add(off).wrapping_add(FLAG_BASE) as *mut u8).write((val & 0xFF) as u8);
        let cur = (bank.wrapping_add(CURSOR) as *const u32).read_unaligned();
        if (cur as i32) >= 0 {
            scan(index);
        }
        let bank = (lf_checker_rt::relocated(TABLE).wrapping_add(index.wrapping_mul(4))
            as *const u32)
            .read();
        let cur = (bank.wrapping_add(CURSOR) as *const u32).read_unaligned();
        let lim = (bank.wrapping_add(LIMIT) as *const u32).read_unaligned();
        if (cur as i32) < (lim as i32) {
            return 0;
        }
        (bank.wrapping_add(CURSOR) as *mut u32).write_unaligned(0);
        scan(index);
        let bank = (lf_checker_rt::relocated(TABLE).wrapping_add(index.wrapping_mul(4))
            as *const u32)
            .read();
        let cur = (bank.wrapping_add(CURSOR) as *const u32).read_unaligned();
        let lim = (bank.wrapping_add(LIMIT) as *const u32).read_unaligned();
        if (cur as i32) < (lim as i32) {
            return 0;
        }
        (bank.wrapping_add(CURSOR) as *mut u32).write_unaligned(0);
        0
    }
});
