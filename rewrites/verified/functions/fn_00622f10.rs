// original: 0x00622f10 net_entry_find_by_slot0
/// Find the first table entry whose first word equals the id.
///
/// Scans the entry-pointer table at `this+0x2e24` (up to `count` at
/// `this+0x2ea4` entries) and returns the first entry whose word at +0
/// equals `id`. Returns null when `id` is negative, when the count is not
/// positive, or when no entry matches.
export!(thiscall, rw_00622f10(this: u32, id: i32) -> u32 {
    unsafe {
        if id < 0 {
            return 0;
        }
        let base = this as *const u8;
        let count = (base.add(0x2ea4) as *const i32).read_unaligned();
        if count <= 0 {
            return 0;
        }
        let table = base.add(0x2e24) as *const u32;
        let mut i: i32 = 0;
        while i < count {
            let e = table.add(i as usize).read_unaligned() as *const u32;
            if e.read_unaligned() == id as u32 {
                return e as u32;
            }
            i += 1;
        }
        0
    }
});
