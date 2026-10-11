// original: 0x00A34860 count_active_world_entries

/// Counts nonzero first dwords in the 90-record world table. The original
/// visits 15 groups of six consecutive records, with each record 0x2c bytes
/// apart; only the first dword of each record affects the result. Any nonzero
/// value counts, including values with the high bit set. This is a no-argument
/// cdecl function returning the count in EAX.
lf_checker_rt::export!(cdecl, rw_00a34860() -> u32 {
    const TABLE_VA: u32 = 0x012d_e150;
    const RECORD_STRIDE: u32 = 0x2c;
    const RECORD_COUNT: u32 = 90;

    let table = lf_checker_rt::global::<u8>(TABLE_VA) as u32;
    let mut count = 0u32;
    let mut index = 0u32;
    while index < RECORD_COUNT {
        let slot = table.wrapping_add(index.wrapping_mul(RECORD_STRIDE));
        let value = unsafe { (slot as *const u32).read_unaligned() };
        count += u32::from(value != 0);
        index += 1;
    }
    count
});
