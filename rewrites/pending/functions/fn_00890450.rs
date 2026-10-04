// original: 0x00890450 audio_find_slot_node
/// Find `key` in this object's two id slots and look up that slot's node.
///
/// Scans the dwords at `this+0x9c` and `this+0xa0`; when slot `i` matches,
/// behaves like [`rw_00890410`] for that slot. Returns null when the key is
/// absent or the matched slot is empty (0xff).
export!(thiscall, rw_00890450(this: u32, key: u32) -> u32 {
    unsafe {
        for i in 0..2u32 {
            if ((this + 0x9c + i * 4) as *const u32).read() == key {
                let slot = ((this + 0x48 + i) as *const u8).read();
                if slot == 0xff {
                    return 0;
                }
                let stride = *global::<u32>(0x0115D964);
                let variant = ((this + 0x40) as *const u8).read() as u32;
                let base = *global::<u32>(0x0115D988);
                let row = ((base
                    .wrapping_add(variant.wrapping_mul(0x6f40))
                    .wrapping_add(0x6f10)) as *const u32)
                    .read();
                return stride.wrapping_mul(slot as u32).wrapping_add(row);
            }
        }
        0
    }
});
