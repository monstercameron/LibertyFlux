// original: 0x00695290 anim_registry_lookup
/// Look up an animation object by id.
///
/// Ids below 0x80 index the direct table. Larger ids scan the overflow
/// entries (slots 14 and up) for a matching leading id byte. Returns the
/// object pointer, or null when absent.
export!(thiscall, rs80_695290(id: u32) -> u32 {
    unsafe {
        const DIRECT_MAX: u8 = 0x80;
        const OVERFLOW_FROM: u32 = 14;
        let b = id as u8;
        if b < DIRECT_MAX {
            let table = *global::<u32>(0x019F2380) as *const u32;
            *table.add(b as usize)
        } else {
            let count = *global::<u16>(0x019F2384) as u32;
            if count <= OVERFLOW_FROM {
                return 0;
            }
            let table = *global::<u32>(0x019F2380) as *const u32;
            let mut i = OVERFLOW_FROM;
            while i < count {
                let e = *table.add(i as usize);
                if e != 0 && *(e as *const u8) == b {
                    return e;
                }
                i += 1;
            }
            0
        }
    }
});
