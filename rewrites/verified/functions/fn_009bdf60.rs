// original: 0x009BDF60 ped_event_query_status
/// Returns zero when `owner` is null or its object pointer at byte offset
/// 0x6c is null. Otherwise calls the object's zero-argument thiscall status
/// method and returns its low 16 bits; upper EAX bytes are not defined on
/// the null-child path.
lf_checker_rt::export!(cdecl, rw_009bdf60(owner: u32) -> u16 {
    const STATUS_OBJECT_OFFSET: u32 = 0x6C;

    if owner == 0 {
        return 0;
    }
    let child = unsafe { ((owner + STATUS_OBJECT_OFFSET) as *const u32).read() };
    if child == 0 {
        return 0;
    }
    lf_checker_rt::callee_thiscall!(1, u32, child) as u16
});
