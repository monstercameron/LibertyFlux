// original: 0x00c55890 CEventSeenCop::~CEventSeenCop__deleting
/// Delete a CEventSeenCop object. Replace its vtable with the most-derived table, call the acquaintance-event teardown, and call the global manager's deleting helper only when flag bit zero is set. Return the original object pointer; other flag bits do not affect the branch.
lf_checker_rt::export!(thiscall, rw_00c55890(this: u32, flags: u32) -> u32 {
unsafe {
    const EVENT_VTABLE_VA: u32 = 0x00ec_ace4;
    const MANAGER_GLOBAL_VA: u32 = 0x0166_69c8;
    core::ptr::write_unaligned(this as *mut u32, lf_checker_rt::relocated(EVENT_VTABLE_VA));
    let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
    if flags & 1 != 0 {
        let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, manager, this);
    }
    this
}
});
