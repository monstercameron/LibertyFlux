// original: 0x00B464D0 CEventDraggedOutCar::~CEventDraggedOutCar__deleting

/// Implements CEventDraggedOutCar::vf0: returns the event object's address after the base cleanup boundary. When the low bit of the deleting flag is set, forwards the object to the class allocator boundary.
lf_checker_rt::export!(thiscall, rw_00b464d0(this_ptr: u32, deleting_flags: u32) -> u32 {
    const DELETE_FLAG: u32 = 1;
    const TASKSET_VTABLE_VA: u32 = 0x00eae7d4;
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
