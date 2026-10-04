// original: 0x00950ae0 range_keyed_accumulate
/// Accumulate scripted span values over key-ordered table entries.
///
/// Takes one unsigned argument and subtracts a global base to form an
/// adjusted bound. Two scripted helpers supply a range bound and a range
/// floor; when the adjusted bound is below the floor (signed), their
/// difference seeds the accumulator and the floor becomes the cursor. A
/// scripted table helper answers a descriptor whose words at +0x28/+0x2c
/// are an entry-pointer array and a 16-bit count. A first scan remembers
/// the last entry whose key at +0x14 lies in [floor, adjusted] (signed)
/// and resolves that entry's tag byte at +1 through a scripted rater.
/// A second scan walks the entries after that hit: each key inside
/// [cursor, bound] (signed) with a positive span over the cursor adds a
/// scripted combine of (span, rate) to the accumulator, moves the cursor
/// to the key and re-rates. A final tail covers [cursor, bound] the same
/// way when the direct span is positive. All comparisons are signed and
/// all arithmetic wraps. Returns the accumulator.
export!(cdecl, rw_00950ae0(a0: u32) -> u32 {
    unsafe {
        const G_SUB: u32 = 0x11F7028;
        const G_LOOKUP_ARG: u32 = 0x11F6F34;
        const G_LOOKUP_THIS: u32 = 0x11F6954;
        const ID_BOUND: u32 = 0;
        const ID_FLOOR: u32 = 1;
        const ID_LOOKUP: u32 = 2;
        const ID_RATE: u32 = 3;
        const ID_COMBINE: u32 = 4;
        const DESC_ARRAY_OFF: u32 = 0x28;
        const DESC_COUNT_OFF: u32 = 0x2C;
        const ENTRY_KEY_OFF: u32 = 0x14;
        const ENTRY_TAG_OFF: u32 = 1;

        let hi = a0.wrapping_sub(*(global::<u32>(G_SUB)));
        let bound: u32 = callee_cdecl!(ID_BOUND, u32,);
        let floor: u32 = callee_cdecl!(ID_FLOOR, u32,);
        let mut acc = 0u32;
        let mut cursor = hi;
        if (hi as i32) < (floor as i32) {
            acc = floor.wrapping_sub(hi);
            cursor = floor;
        }
        let desc: u32 = callee_thiscall!(
            ID_LOOKUP,
            u32,
            *(global::<u32>(G_LOOKUP_THIS)),
            *(global::<u32>(G_LOOKUP_ARG))
        );
        let array = *((desc.wrapping_add(DESC_ARRAY_OFF)) as *const u32);
        let count =
            core::ptr::read_unaligned((desc.wrapping_add(DESC_COUNT_OFF)) as *const u16) as u32;
        let end = array.wrapping_add(count.wrapping_mul(4));
        let mut last_hit = 0u32;
        let mut resume = 0u32;
        let mut p = array;
        while p != end {
            let entry = *(p as *const u32);
            let key = *((entry.wrapping_add(ENTRY_KEY_OFF)) as *const u32);
            if (floor as i32) <= (key as i32) && (key as i32) <= (hi as i32) {
                last_hit = p;
                resume = p.wrapping_add(4);
            }
            p = p.wrapping_add(4);
        }
        let mut rate = 0u32;
        if last_hit != 0 {
            let entry = *(last_hit as *const u32);
            let tag = *((entry.wrapping_add(ENTRY_TAG_OFF)) as *const u8);
            rate = callee_cdecl!(ID_RATE, u32, tag as u32);
        }
        // The no-hit path skips the bound/cursor pre-check the other
        // paths take and decides on the direct span alone.
        if resume == 0 {
            let span = bound.wrapping_sub(cursor);
            if (span as i32) <= 0 {
                return acc;
            }
            let got: u32 = callee_cdecl!(ID_COMBINE, u32, span, rate);
            return acc.wrapping_add(got);
        }
        if resume != end {
            let mut q = resume;
            loop {
                let entry = *(q as *const u32);
                let key = *((entry.wrapping_add(ENTRY_KEY_OFF)) as *const u32);
                if (cursor as i32) <= (key as i32) && (key as i32) <= (bound as i32) {
                    let span = key.wrapping_sub(cursor);
                    if (span as i32) > 0 {
                        let got: u32 = callee_cdecl!(ID_COMBINE, u32, span, rate);
                        acc = acc.wrapping_add(got);
                    }
                    cursor = key;
                    let tag = *((entry.wrapping_add(ENTRY_TAG_OFF)) as *const u8);
                    rate = callee_cdecl!(ID_RATE, u32, tag as u32);
                }
                q = q.wrapping_add(4);
                // The original re-reads the descriptor bounds each pass.
                let array_now = *((desc.wrapping_add(DESC_ARRAY_OFF)) as *const u32);
                let count_now = core::ptr::read_unaligned(
                    (desc.wrapping_add(DESC_COUNT_OFF)) as *const u16,
                ) as u32;
                if q == array_now.wrapping_add(count_now.wrapping_mul(4)) {
                    break;
                }
            }
        }
        if (bound as i32) <= (cursor as i32) {
            return acc;
        }
        let span = bound.wrapping_sub(cursor);
        if (span as i32) <= 0 {
            return acc;
        }
        let got: u32 = callee_cdecl!(ID_COMBINE, u32, span, rate);
        acc.wrapping_add(got)
    }
});
