// original: 0x00E6CFB0 initialize_vehicle_index_table_sentinels
/// Fills the 512-word vehicle index table with the 0xFFFF_FFFF sentinel and leaves that value in EAX. The table is a global image range; there are no arguments.
const TABLE_VA: u32 = 0x0179D128;
const TABLE_WORDS: u32 = 512;
const SENTINEL: u32 = 0xFFFF_FFFF;
lf_checker_rt::export!(cdecl, rw_00e6cfb0() -> u32 {
    let table = lf_checker_rt::global::<u32>(TABLE_VA);
    for index in 0..TABLE_WORDS {
        unsafe { table.add(index as usize).write(SENTINEL) };
    }
    SENTINEL
});
