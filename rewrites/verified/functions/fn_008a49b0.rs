// original: 0x008a49b0 audio_table_key_search
/// Search two audio key tables for an entry id, return its payload pointer.
///
/// The object holds two entry counts (at +0xB0 and +0xB4) and three selector
/// bytes (at +0x40, +0xB8, +0xB9). Each phase indexes a global pointer table
/// by `selector * 0x6F40`, adds a scaled offset, then linearly scans that
/// many 8-byte entries comparing the first dword with `key`. On a hit it
/// returns a pointer to the entry's second dword; when neither phase hits it
/// returns null.
export!(thiscall, rw_008a49b0(this: *const u8, key: u32) -> u32 {
    unsafe {
        let base = *global::<u32>(0x0115D988);
        let search = |count: u32, index: u8, scale: u8, step: u32, bias: u32| -> Option<u32> {
            unsafe {
                if count == 0 {
                    return None;
                }
                let slot = base
                    .wrapping_add((index as u32).wrapping_mul(0x6F40))
                    .wrapping_add(bias);
                let entries = (*(slot as *const u32))
                    .wrapping_add((scale as u32).wrapping_mul(step));
                let mut i = 0u32;
                while i < count {
                    let at = entries.wrapping_add(i.wrapping_mul(8));
                    if *(at as *const u32) == key {
                        return Some(at.wrapping_add(4));
                    }
                    i += 1;
                }
                None
            }
        };
        if let Some(hit) = search(
            *(this.add(0xB0) as *const u32),
            *this.add(0x40),
            *this.add(0xB8),
            *global::<u32>(0x0115D964),
            0x6F10,
        ) {
            return hit;
        }
        if let Some(hit) = search(
            *(this.add(0xB4) as *const u32),
            *this.add(0x40),
            *this.add(0xB9),
            *global::<u32>(0x0115D968),
            0x6F14,
        ) {
            return hit;
        }
        0
    }
});

