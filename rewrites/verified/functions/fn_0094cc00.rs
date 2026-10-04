// original: 0x0094cc00 release_global_entry
/// Mark one global table entry as free.
///
/// Stores the invalid marker into both dwords of entry `index & 0xFFFF` of
/// the global pair table. Returns the masked index.
export!(cdecl, rw_0094cc00(index: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x011D95A0;
        let idx = index & 0xFFFF;
        let slot = global::<u32>(TABLE + idx * 8);
        *slot = 0xFFFF_FFFF;
        *slot.add(1) = 0xFFFF_FFFF;
        idx
    }
});
