// original: 0x005C09C0 active_record_lookup

/// Return the active record's address in the indexed-record table. The global
/// record index is an unsigned 32-bit value; `u32::MAX` is the absent-record
/// sentinel and returns null. Every other index is multiplied by the
/// 0xBC-byte record stride with 32-bit wrapping arithmetic and added to the
/// table base. This is a zero-argument cdecl helper.
lf_checker_rt::export!(cdecl, rw_005c09c0() -> u32 {
    const ACTIVE_RECORD_INDEX_VA: u32 = 0x010330F8;
    const ACTIVE_RECORD_TABLE_VA: u32 = 0x0118D470;
    const ACTIVE_RECORD_STRIDE: u32 = 0xBC;
    const NO_ACTIVE_RECORD: u32 = u32::MAX;

    let index = unsafe { lf_checker_rt::global::<u32>(ACTIVE_RECORD_INDEX_VA).read_unaligned() };
    if index == NO_ACTIVE_RECORD {
        0
    } else {
        lf_checker_rt::relocated(ACTIVE_RECORD_TABLE_VA)
            .wrapping_add(index.wrapping_mul(ACTIVE_RECORD_STRIDE))
    }
});
