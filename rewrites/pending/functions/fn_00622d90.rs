// original: 0x00622d90 net_session_find_by_id_pair
/// Find the first session entry whose id pair matches the key.
///
/// Scans the entry-pointer table at `this+0x1d94` (up to `count` at
/// `this+0x1e14` entries) and returns the first entry whose words at +0x40
/// and +0x44 equal the two key words. Returns null when the count is not
/// positive or no entry matches. Reads the key only when the count is
/// positive, and reads each entry's second word only when the first matches.
export!(thiscall, rw_00622d90(this: u32, key: u32) -> u32 {
    unsafe {
        let base = this as *const u8;
        let count = (base.add(0x1e14) as *const i32).read_unaligned();
        if count <= 0 {
            return 0;
        }
        let want0 = (key as *const u32).read_unaligned();
        let want1 = (key as *const u32).add(1).read_unaligned();
        let table = base.add(0x1d94) as *const u32;
        let mut i: i32 = 0;
        while i < count {
            let e = table.add(i as usize).read_unaligned() as *const u8;
            if (e.add(0x40) as *const u32).read_unaligned() == want0
                && (e.add(0x44) as *const u32).read_unaligned() == want1
            {
                return e as u32;
            }
            i += 1;
        }
        0
    }
});
