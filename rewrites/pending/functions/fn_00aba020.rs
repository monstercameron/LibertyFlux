// original: 0x00aba020 clamped_table_lookup

/// Clamped index into a six-entry global pointer table.
///
/// Negative indexes and indexes past the fifth entry read slot zero; the
/// table itself lives in the game's data section and is resolved through the
/// relocated image base.
export!(cdecl, rs64_aba020(idx: i32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0103_EEB8;
        const ENTRIES: i32 = 6;
        let i = if idx < 0 || idx >= ENTRIES {
            0usize
        } else {
            idx as usize
        };
        *global::<u32>(TABLE).add(i)
    }
});
