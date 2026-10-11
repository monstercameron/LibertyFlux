// original: 0x00A82CF0 CSimpleCarDriveTaskInfo::vf18

/// Forward one task kind to the shared sequence-progress query.
///
/// The `this` pointer is not read. The one stack argument is forwarded
/// unchanged to a cdecl helper, and its 32-bit return value is returned.
lf_checker_rt::export!(thiscall, rw_00a82cf0(this: u32, task_kind: u32) -> u32 {
    const SEQUENCE_QUERY: u32 = 1;
    let _ = this;
    lf_checker_rt::callee_cdecl!(SEQUENCE_QUERY, u32, task_kind)
});
