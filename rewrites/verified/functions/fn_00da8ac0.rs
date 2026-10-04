// original: 0x00da8ac0 CTaskComplexStuckInAir::vf0
/// Tear the task down, freeing its storage when asked.
///
/// Runs the base teardown unconditionally, then releases the storage through
/// the shared allocator when the low bit of the flag argument is set.
/// Returns the task pointer in both cases.
export!(thiscall, rw_00da8ac0(this: u32, flags: u32) -> u32 {
    unsafe {
        /// Global slot holding the shared allocator.
        const ALLOC: u32 = 0x0167E2A0;
        /// Flag bit requesting storage release.
        const FREE: u32 = 1;
        let _: u32 = callee_thiscall!(1, u32, this);
        if flags & 0xFF & FREE != 0 {
            let alloc = global::<u32>(ALLOC).read();
            let _: u32 = callee_thiscall!(2, u32, alloc, this);
        }
        this
    }
});
