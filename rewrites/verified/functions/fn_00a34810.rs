// original: 0x00A34810 clear_world_entry_by_index

/// Computes the address of the indexed record in the first world table and
/// forwards it in ECX to the record-reset helper. The table uses 0x2c-byte
/// records, and the helper clears the record's first dword plus the dword at
/// +0x24 when needed. The original tail call preserves incoming EAX; the proof
/// pins that register to zero because a Rust cdecl export cannot read it. This
/// is a cdecl function with one stack index.
lf_checker_rt::export!(cdecl, rw_00a34810(record_index: u32) -> u32 {
    const TABLE_VA: u32 = 0x012d_e150;
    const RECORD_STRIDE: u32 = 0x2c;

    let table = lf_checker_rt::global::<u8>(TABLE_VA) as u32;
    let entry = table.wrapping_add(record_index.wrapping_mul(RECORD_STRIDE));
    lf_checker_rt::callee_thiscall!(1, u32, entry);
    0
});
