// original: 0x0094eb40 store_link_and_set_flag
/// Store a link and latch the owner-present flag.
///
/// Always stores `link` at offset `0x4C`. When the link is nonzero, bit 30
/// of the flags word is set (a zero link leaves the flags untouched).
/// Returns the original's exit EAX residue: the flag bit when it changed
/// from clear to set, otherwise zero.
export!(thiscall, rw_0094eb40(obj: *mut u32, link: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x4000_0000;
        const FLAGS: usize = 0x24 / 4;
        const LINK: usize = 0x4C / 4;
        *obj.add(LINK) = link;
        if link != 0 {
            let was_set = *obj.add(FLAGS) & FLAG != 0;
            *obj.add(FLAGS) |= FLAG;
            if was_set {
                0
            } else {
                FLAG
            }
        } else {
            0
        }
    }
});
