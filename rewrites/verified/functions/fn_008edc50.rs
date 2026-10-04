// original: 0x008edc50 id_cache_insert_lru
/// Insert `id` into a 32-slot cache with LRU eviction.
///
/// Only ids whose node type nibble is 1 or 2 take part (others return
/// `id` unchanged). A known id only refreshes its timestamp; a new id
/// takes the first free slot (low word `0xFFFF`) or evicts the slot
/// with the smallest timestamp. Returns the stamp, or `id` when idle.
export!(thiscall, rw_008edc50(this: *mut u8, id: u32) -> u32 {
    unsafe {
        let slot = |i: u32| {
            this.add((0x1AC4 + i * 4) as usize) as *mut u32
        };
        let stamp = |i: u32| {
            this.add((0x1B44 + i * 4) as usize) as *mut u32
        };
        let node = (*global::<u32>(0x1178284 + (id & 0xFFFF) * 4))
            .wrapping_add((id >> 16).wrapping_mul(32));
        let nib = *((node + 0x1c) as *const u8) >> 4;
        if nib != 1 && nib != 2 {
            return id;
        }
        let g = *global::<u32>(0x11735B4);
        let mut i = 0u32;
        while i < 32 {
            if *slot(i) == id {
                *stamp(i) = g;
                return g;
            }
            i += 1;
        }
        i = 0;
        while i < 32 {
            if *slot(i) & 0xFFFF == 0xFFFF {
                *slot(i) = id;
                *stamp(i) = g;
                return g;
            }
            i += 1;
        }
        let mut best = 0xFFFFFFFFu32;
        let mut bi = 0u32;
        i = 0;
        while i < 32 {
            let t = *stamp(i);
            if t < best {
                best = t;
                bi = i;
            }
            i += 1;
        }
        *slot(bi) = id;
        *stamp(bi) = g;
        g
    }
});
