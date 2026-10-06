// original: 0x008BEBC0 menu_find_by_key_word (proposed)

/// Select the menu entry whose 16-bit key matches, if any.
///
/// The first stack word indexes the global handler table; the entry address
/// is passed to every callee. A setup callee runs first (its answer is
/// ignored), then a counting callee whose answer is the entry count as a
/// SIGNED dword: zero or negative returns the count at once. Otherwise the
/// entries of the current mode (global mode dword times 24 into the entry
/// table, each entry 0x16 bytes, key as a sign-extended word at +0x12) are
/// scanned while the SIGNED index stays below the count; the first entry
/// whose key equals the second stack word (full 32-bit compare) is chosen
/// through the select callee with (entry, index) and its answer is returned.
/// With no match the last compared key is returned (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_008BEBC0(idx: u32, key: u32) -> u32 {
    unsafe {
        /// Global table of handler entry addresses.
        const HANDLER_TABLE: u32 = 0x01160C0C;
        /// Global mode dword selecting the entry array.
        const MODE: u32 = 0x01160C40;
        /// Table of entry-array pointers, one 24-byte slot per mode.
        const ENTRY_TABLE: u32 = 0x019D33A0;
        /// Bytes per entry and offset of the key word inside an entry.
        const ENTRY_STRIDE: u32 = 0x16;
        const KEY_OFF: u32 = 0x12;
        /// Callee ids in the contract: setup, count, select.
        const SETUP: u32 = 1;
        const COUNT: u32 = 2;
        const SELECT: u32 = 3;
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
            let w = ((base + KEY_OFF + (index as u32) * ENTRY_STRIDE)
                as *const i16)
                .read_unaligned() as i32 as u32;
            last = w;
            if w == key {
                return lf_checker_rt::callee_cdecl!(SELECT, u32, entry, index as u32);
            }
            index += 1;
            if !(index < count) {
                break;
            }
        }
        last
    }
});
