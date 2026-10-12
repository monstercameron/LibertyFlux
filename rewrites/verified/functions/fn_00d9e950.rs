// original: 0x00D9E950 stream_global_record_value
/// Looks up a requested key in the global stream table, then returns the selected record's value field.
lf_checker_rt::export!(stdcall, rw_00d9e950(requested_key: u32) -> u32 {
    const STREAM_TABLE_FILE_VA: u32 = 0x171C968;
    let stream_table = lf_checker_rt::relocated(STREAM_TABLE_FILE_VA);
    let record: u32 = lf_checker_rt::callee_thiscall!(1, u32, stream_table, requested_key);
    lf_checker_rt::callee_thiscall!(2, u32, record)
});
