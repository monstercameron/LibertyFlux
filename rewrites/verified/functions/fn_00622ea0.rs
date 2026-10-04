// original: 0x00622ea0 net_entry_find_embedded_or_by_id_pair
/// Find the embedded slot or a table entry matching an id pair.
///
/// When `a0`/`a1` equal the local id pair at `this+0xbf0`/`this+0xbf4`,
/// returns the embedded slot at `this+0xbb0`. Otherwise scans the
/// entry-pointer table at `this+0x2e24` (up to `count` at `this+0x2ea4`
/// entries) for the first entry whose words at +0x40/+0x44 equal `a0`/`a1`,
/// returning null when the count is not positive or nothing matches.
export!(thiscall, rw_00622ea0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        let base = this as *const u8;
        if a0 == (base.add(0xbf0) as *const u32).read_unaligned()
            && a1 == (base.add(0xbf4) as *const u32).read_unaligned()
        {
            return base.add(0xbb0) as u32;
        }
        let count = (base.add(0x2ea4) as *const i32).read_unaligned();
        if count <= 0 {
            return 0;
        }
        let table = base.add(0x2e24) as *const u32;
        let mut i: i32 = 0;
        while i < count {
            let e = table.add(i as usize).read_unaligned() as *const u8;
            if (e.add(0x40) as *const u32).read_unaligned() == a0
                && (e.add(0x44) as *const u32).read_unaligned() == a1
            {
                return e as u32;
            }
            i += 1;
        }
        0
    }
});
