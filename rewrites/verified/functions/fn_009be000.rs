// original: 0x009BE000 CEventImminentDanger::vf0
/// Runs the deleting destructor for an imminent-danger event. It installs the
/// event vtable, releases a nonnull child through the slot at byte offset
/// 0x44, calls the base destructor, and calls the manager deletion helper when
/// bit 0 of `deleting_flags` is set. It returns the original object pointer.
/// The pointer and flag arguments use the 32-bit MSVC thiscall convention.
lf_checker_rt::export!(thiscall, rw_009be000(this: u32, deleting_flags: u32) -> u32 {
    const EVENT_VTABLE_VA: u32 = 0x00E9_4CDC;
    const CHILD_SLOT_OFFSET: u32 = 0x44;
    const DELETE_FLAG: u32 = 1;
    const MANAGER_GLOBAL_VA: u32 = 0x0166_69C8;

    let child_slot = this.wrapping_add(CHILD_SLOT_OFFSET);
    let child = unsafe { (child_slot as *const u32).read() };

    unsafe {
        (this as *mut u32).write(lf_checker_rt::relocated(EVENT_VTABLE_VA));
    }
    if child != 0 {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, child, child_slot);
    }
    let _ = lf_checker_rt::callee_thiscall!(2, u32, this);

    if deleting_flags & DELETE_FLAG != 0 {
        let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read() };
        let _ = lf_checker_rt::callee_thiscall!(3, u32, manager, this);
    }

    this
});
