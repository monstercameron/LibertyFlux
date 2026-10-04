// original: 0x006212b0 net_count_adjusted_slots
/// Compute an adjusted slot count, minus reserver matches.
///
/// Forms a base value from header words (`+0x124`, `+0x2ea8`, and, when
/// `id` is zero, `+0x1e14` and `+0x120`), then subtracts the number of
/// stride-0x20 slots in the table at `this+0x2ec0` (up to `count` at
/// `this+0x32b0` slots) whose first word equals `id`. All arithmetic wraps.
export!(thiscall, rw_006212b0(this: u32, id: u32) -> u32 {
    unsafe {
        let base = this as *const u8;
        let r = |off: usize| (base.add(off) as *const u32).read_unaligned();
        let delta = r(0x124).wrapping_sub(r(0x2ea8));
        let mut eax = if id == 0 {
            r(0x124).wrapping_sub(r(0x1e14)).wrapping_add(r(0x120)).wrapping_sub(delta)
        } else {
            delta
        };
        let count = r(0x32b0) as i32;
        if count > 0 {
            let mut hits: u32 = 0;
            let mut i: i32 = 0;
            while i < count {
                let slot = base.add(0x2ec0) as *const u8;
                if (slot.add((i as usize) * 0x20) as *const u32).read_unaligned() == id {
                    hits += 1;
                }
                i += 1;
            }
            eax = eax.wrapping_sub(hits);
        }
        eax
    }
});
