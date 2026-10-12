// original: 0x005F3D00 resolved_bit_test (proposed)

/// Test a key-selected bit after resolving the entry table.
///
/// Resolves the table owner from the link at `this + 0x5c`: a null link
/// selects the fallback at `this + 0x6c`, otherwise the link target's word
/// at `+ 8` must carry both ready bits (0x30000000) and the target's first
/// word is the owner (any other flag pattern makes the original fault on a
/// null base, so only ready links are exercised). Reads the entry pointer
/// from the owner's table at `+ 0xd4` indexed by the stack argument, takes
/// the key byte at entry `+ 0xc`, and tests bit `key & 31` of word
/// `key >> 5` of the shared bit array reached like `bitarray_test` through
/// `this + 0x64`. A null inner object reads as clear. The original is
/// thiscall with one stack argument, returning the result in AL.
lf_checker_rt::export!(thiscall, rw_005f3d00(this: u32, index: u32) -> u32 {
    unsafe {
        const LINK_OFFSET: u32 = 0x5c;
        const FALLBACK_OFFSET: u32 = 0x6c;
        const FLAGS_OFFSET: u32 = 0x08;
        const READY_MASK: u32 = 0x30000000;
        const TABLE_OFFSET: u32 = 0xd4;
        const KEY_OFFSET: u32 = 0x0c;
        const INNER_OFFSET: u32 = 0x64;
        const DESC_OFFSET: u32 = 0x10c;
        let link =
            (this.wrapping_add(LINK_OFFSET) as *const u32).read_unaligned();
        let base = if link == 0 {
            (this.wrapping_add(FALLBACK_OFFSET) as *const u32).read_unaligned()
        } else {
            let flags = (link.wrapping_add(FLAGS_OFFSET) as *const u32)
                .read_unaligned();
            if flags & READY_MASK != READY_MASK {
                // The original zeroes the base and faults on the next load.
                (0xd4u32 as *const u32).read_unaligned();
                return 0;
            }
            (link as *const u32).read_unaligned()
        };
        let table =
            (base.wrapping_add(TABLE_OFFSET) as *const u32).read_unaligned();
        let entry = (table.wrapping_add(index.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let key =
            (entry.wrapping_add(KEY_OFFSET) as *const u8).read_unaligned()
                as u32;
        let inner =
            (this.wrapping_add(INNER_OFFSET) as *const u32).read_unaligned();
        if inner == 0 {
            return 0;
        }
        let desc =
            (inner.wrapping_add(DESC_OFFSET) as *const u32).read_unaligned();
        let words = (desc as *const u32).read_unaligned();
        let bit = key & 31;
        let word = (key as i32 >> 5) as u32;
        let w = (words.wrapping_add(word.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        u32::from(w & (1u32 << bit) != 0)
    }
});
