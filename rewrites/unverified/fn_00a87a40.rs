// original: 0x00A87A40 init_release_table
/// Initializes four indexed entries by calling the pair of table writers
/// with each slot index and a zero flag. The last helper result remains in EAX.
/// The table object is passed in ECX; both writers are intercepted thiscall
/// callees with two stack arguments.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00a87a40(table: u32) -> u32 {
    const FIRST_SLOT: u32 = 0;
    const SLOT_LIMIT: u32 = 4;
    const WRITE_FLAGS: u32 = 0;
    let mut last_write = 0;
    for slot_index in FIRST_SLOT..SLOT_LIMIT {
        lf_checker_rt::callee_thiscall!(1, u32, table, slot_index, WRITE_FLAGS);
        last_write = lf_checker_rt::callee_thiscall!(2, u32, table, slot_index, WRITE_FLAGS);
    }
    last_write
}
