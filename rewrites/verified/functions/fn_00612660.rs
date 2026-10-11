// original: 0x00612660 physics_restore_archetype_cache

/// Restore twelve defined cached dwords from `source` into `this`, leaving
/// the three intervening gap words untouched. The machine return carries the
/// final copied dword. This is a 32-bit thiscall with one stack pointer.
lf_checker_rt::export!(thiscall, rw_00612660(this: u32, source: u32) -> u32 {
    unsafe {
        const SOURCE_OFFSETS: [u32; 12] = [0, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38];
        const DESTINATION: u32 = 0x80;
        let mut last = 0;
        for offset in SOURCE_OFFSETS {
            last = ((source + offset) as *const u32).read_unaligned();
            ((this + DESTINATION + offset) as *mut u32).write_unaligned(last);
        }
        last
    }
});
