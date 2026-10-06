// original: 0x008BEC30 menu_find_by_key_byte (proposed)

/// Select the menu entry whose kind byte matches, then confirm it.
///
/// Same shape as the neighbouring word-key scan: the first stack word
/// indexes the global handler table, a setup callee runs, and a counting
/// callee gives the entry count as a SIGNED dword (zero or negative returns
/// the count at once). The mode's entries are scanned while the SIGNED index
/// stays below the count, comparing the zero-extended kind byte at each
/// entry's first byte against the second stack word (full 32-bit compare, so
/// only keys below 256 can match). On a match the select callee runs with
/// (entry, index) and then the confirm callee with (entry, index, 1), whose
/// answer is returned. With no match the last compared byte is returned
/// (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_008BEC30(idx: u32, key: u32) -> u32 {
    unsafe {
        /// Global table of handler entry addresses.
        const HANDLER_TABLE: u32 = 0x01160C0C;
        /// Global mode dword selecting the entry array.
        const MODE: u32 = 0x01160C40;
        /// Table of entry-array pointers, one 24-byte slot per mode.
        const ENTRY_TABLE: u32 = 0x019D33A0;
        /// Bytes per entry; the kind byte is the entry's first byte.
        const ENTRY_STRIDE: u32 = 0x16;
        /// Callee ids in the contract: setup, count, select, confirm.
        const SETUP: u32 = 1;
        const COUNT: u32 = 2;
        const SELECT: u32 = 3;
        const CONFIRM: u32 = 4;
        let entry = (lf_checker_rt::relocated(
            HANDLER_TABLE.wrapping_add(idx.wrapping_mul(4)),
        ) as *const u32)
            .read_unaligned();
        lf_checker_rt::callee_cdecl!(SETUP, u32, entry);
        // The count is compared SIGNED (jle/jl in the original).
        let count = lf_checker_rt::callee_cdecl!(COUNT, u32, entry) as i32;
        if count <= 0 {
            return count as u32;
        }
        let mode = lf_checker_rt::global::<u32>(MODE).read_unaligned();
        let base = (lf_checker_rt::relocated(
            ENTRY_TABLE.wrapping_add(mode.wrapping_mul(24)),
        ) as *const u32)
            .read_unaligned();
        let mut index = 0i32;
        let mut last = 0u32;
        loop {
            let b = ((base + (index as u32) * ENTRY_STRIDE) as *const u8).read() as u32;
            last = b;
            if b == key {
                lf_checker_rt::callee_cdecl!(SELECT, u32, entry, index as u32);
                return lf_checker_rt::callee_cdecl!(CONFIRM, u32, entry, index as u32, 1u32);
            }
            index += 1;
            if !(index < count) {
                break;
            }
        }
        last
    }
});
