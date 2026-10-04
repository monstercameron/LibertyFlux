// original: 0x00B50D10 id_list_contains

/// Linear search of an inline id list for `wanted`.
///
/// `this + 0x08` holds a signed count; the entries start at `this + 0x0C`.
/// A count of zero or less matches nothing. Returns 1 in the low byte when an
/// entry equals `wanted`, else 0 (upper result bits are caller leftovers, so
/// only the low byte is compared).
///
/// Original: 0x00B50D10 (thiscall, `this` in ecx, one stack word).
export!(thiscall, rw_00b50d10(this: u32, wanted: u32) -> u32 {
    const COUNT_OFF: u32 = 0x08;
    const ITEMS_OFF: u32 = 0x0C;
    unsafe {
        let count = ((this + COUNT_OFF) as *const i32).read_unaligned();
        if count > 0 {
            let mut i = 0i32;
            while i < count {
                let v = ((this + ITEMS_OFF + (i as u32) * 4) as *const u32)
                    .read_unaligned();
                if v == wanted {
                    return 1;
                }
                i += 1;
            }
        }
        0
    }
});
