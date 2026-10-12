// original: 0x00b46470 CEventCommunicateEvent::vf0

/// Call the event object's destructor helper. If bit zero of the deleting flags is
/// set, call the release helper with the global allocator context and this object.
/// Return the original object pointer.
lf_checker_rt::export!(thiscall, rw_00b46470(this_ptr: u32, deleting_flags: u32) -> u32 {
    const DELETE_FLAG: u32 = 1;
    // The destructor and optional release are the only delegated effects.
    const ALLOCATOR_CONTEXT_VA: u32 = 0x016669c8;
    unsafe {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, this_ptr);
        if (deleting_flags & DELETE_FLAG) != 0 {
            let allocator_context = core::ptr::read_unaligned(
                lf_checker_rt::global::<u32>(ALLOCATOR_CONTEXT_VA),
            );
            let _ = lf_checker_rt::callee_thiscall!(2, u32, allocator_context, this_ptr);
        }
    }
    this_ptr

});
