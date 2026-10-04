// original: 0x00da8b90 CTaskComplexStuckInAir::vf1
/// Allocate storage and default-construct a task in it.
///
/// Fetches fresh storage through the shared allocator and tail-constructs a
/// task there, returning the new task; returns 0 when no storage is
/// available. The original reaches the constructor with a tail jump; the
/// rewrite calls it and forwards the result, which is behaviourally identical.
export!(cdecl, rw_00da8b90() -> u32 {
    unsafe {
        /// Global slot holding the shared allocator.
        const ALLOC: u32 = 0x0167E2A0;
        let storage = callee_thiscall!(1, u32, global::<u32>(ALLOC).read());
        if storage == 0 {
            0
        } else {
            callee_thiscall!(2, u32, storage)
        }
    }
});
