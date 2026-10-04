// original: 0x00952fb0 tagged_entry_scan_match
/// Scan a tag-driven entry table for a two-key match.
///
/// Takes an index, two key words, an out-pointer and an extra word;
/// returns 1 on match, 0 otherwise (low byte only). A null out-pointer
/// or an index above a global maximum fails fast. A scripted lookup
/// helper answers a cursor start and a table selector byte; the
/// out-struct's first three words are zeroed and its float fourth word
/// is 0.0 (the original reads that word from an uninitialised frame
/// slot, which the contract defines as zero). Scanning starts at the
/// selected table base plus the cursor: tag 5 fails; tag 0 either
/// advances (when a guard byte at cursor plus a global base reads 2)
/// or re-indexes through an unsigned divide of the selector-plus-one
/// by a global divisor byte, restarting at the remainder table with a
/// zero cursor; tags 0x1b/0x1c run the key matcher, any other tag
/// advances by a scripted step. The matcher works in two phases driven
/// by sticky flags (the first set when the second key is -1 or the
/// extra word is 0): with the second flag clear and the dword at entry
/// +0xc equal to the first key, a scripted match helper runs with the
/// out-pointer, the second flag sets, and a set first flag accepts;
/// otherwise, with the first flag clear and the dword equal to the
/// second key, the helper runs with the extra word, the first flag
/// sets, and a set second flag accepts. Anything else advances.
export!(cdecl, rw_00952fb0(idx: u32, key1: u32, out: u32, key2: u32, extra: u32) -> u32 {
    unsafe {
        const G_MAX: u32 = 0x11F707C;
        const G_TABLE: u32 = 0x11F6F7C;
        const G_GUARD: u32 = 0x11F6FF0;
        const G_DIV: u32 = 0x11F6FFB;
        const ID_LOOKUP: u32 = 0;
        const ID_MATCH: u32 = 1;
        const ID_STEP: u32 = 2;
        const TAG_FAIL: u8 = 5;
        const TAG_GUARD: u8 = 0;
        const TAG_MATCH_LO: u8 = 0x1B;
        const TAG_MATCH_HI: u8 = 0x1C;

        if out == 0 {
            return 0;
        }
        if idx > *(global::<u32>(G_MAX)) {
            return 0;
        }
        let ans: u32 = callee_cdecl!(ID_LOOKUP, u32, idx);
        let mut cursor = *(ans as *const u32);
        let mut sel = *((ans.wrapping_add(8)) as *const u8) as u32;
        *(out as *mut u32) = 0;
        *((out.wrapping_add(4)) as *mut u32) = 0;
        *((out.wrapping_add(8)) as *mut u32) = 0;
        // The original loads this float from its own uninitialised
        // frame; the contract's zero stack fill defines it as 0.0.
        *((out.wrapping_add(12)) as *mut u32) = 0;
        let table = relocated(G_TABLE);
        let mut base = *((table.wrapping_add(sel.wrapping_mul(4))) as *const u32);
        let mut cl = if key2 == 0xFFFFFFFF || extra == 0 { 1u32 } else { 0 };
        let mut dl = 0u32;
        loop {
            let e = base.wrapping_add(cursor);
            let tag = *(e as *const u8);
            if tag == TAG_FAIL {
                return 0;
            }
            if tag == TAG_GUARD {
                let g = *((relocated(G_GUARD).wrapping_add(cursor)) as *const u8);
                if g != 2 {
                    let div = *(relocated(G_DIV) as *const u8) as u32;
                    let n = sel.wrapping_add(1);
                    let r = n % div;
                    cursor = 0;
                    sel = r & 0xFF;
                    base = *((table.wrapping_add(sel.wrapping_mul(4))) as *const u32);
                    let _ = n / div;
                    continue;
                }
            } else if tag == TAG_MATCH_LO || tag == TAG_MATCH_HI {
                let k = *((e.wrapping_add(0xC)) as *const u32);
                let mut matched = false;
                if dl == 0 && k == key1 {
                    let _: u32 = callee_thiscall!(ID_MATCH, u32, e, out);
                    dl = 1;
                    if cl != 0 {
                        return 1;
                    }
                    matched = true;
                }
                if !matched && cl == 0 && k == key2 {
                    let _: u32 = callee_thiscall!(ID_MATCH, u32, e, extra);
                    cl = 1;
                    if dl != 0 {
                        return 1;
                    }
                }
            }
            let step: u32 = callee_cdecl!(ID_STEP, u32, tag as u32);
            cursor = cursor.wrapping_add(step);
        }
    }
});
