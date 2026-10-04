// original: 0x00953110 get_entry_value
/// Read the value word of entry `index` from the entry table. Indexes past
/// 0x5db and entries whose flag byte is clear read as zero. When the
/// companion used-byte is clear the entry is first passed through a reset
/// helper. Only the low word of the index is used.
export!(cdecl, rw_00953110(index: u32) -> u32 {
    unsafe {
        let i = index & 0xffff;
        if i > 0x5db {
            return 0;
        }
        if *global::<u8>(0x011f7114 + i * 8) == 0 {
            return 0;
        }
        if *global::<u8>(0x011f6958 + i) == 0 {
            let slot = relocated(0x011f7110 + i * 8);
            let _: u32 = callee_thiscall!(1, u32, slot);
        }
        *global::<u32>(0x011f7110 + i * 8)
    }
});
