// original: 0x005C09E0 active_record_forward

/// Stop rumble on the active record when the global record index selects one.
/// The index is an unsigned word; `u32::MAX` denotes no active record. The
/// table address is computed with 32-bit wrapping arithmetic and a 0xBC-byte
/// record stride. A null computed address is also ignored. Otherwise the
/// helper calls the record's rumble operation with the caller's word argument.
/// The helper has a cdecl stack argument and does not expose a return value.
lf_checker_rt::export!(cdecl, rw_005c09e0(rumble_arg: u32) -> () {
    const ACTIVE_RECORD_INDEX_VA: u32 = 0x010330F8;
    const ACTIVE_RECORD_TABLE_VA: u32 = 0x0118D470;
    const ACTIVE_RECORD_STRIDE: u32 = 0xBC;
    const NO_ACTIVE_RECORD: u32 = u32::MAX;
    const STOP_RUMBLE_CALLEE: u32 = 1;

    let index = unsafe { lf_checker_rt::global::<u32>(ACTIVE_RECORD_INDEX_VA).read_unaligned() };
    if index == NO_ACTIVE_RECORD {
        return;
    }

    let record = lf_checker_rt::relocated(ACTIVE_RECORD_TABLE_VA)
        .wrapping_add(index.wrapping_mul(ACTIVE_RECORD_STRIDE));
    if record != 0 {
        let _: u32 = lf_checker_rt::callee_thiscall!(STOP_RUMBLE_CALLEE, u32, record, rumble_arg);
    }
});
