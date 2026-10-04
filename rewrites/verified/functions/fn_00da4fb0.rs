// original: 0x00DA4FB0 CTaskComplexShockingEventHurryAway::vf0

/// Scalar deleting destructor: run the object destructor, then free the
/// object through the game memory manager when the caller's flag asks.
///
/// `this` is the task object. `delete_flag` bit 0 selects the freeing
/// path: when set, the manager pointer is loaded from its global slot and
/// the block is released with the object address as its argument. The
/// return value is always `this`, on both paths.
///
/// The destructor itself is an intercepted callee (answered by script), so
/// this rewrite covers the flag test, the global load and the conditional
/// release only. Original: thiscall, one stack word, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da4fb0(this: u32, delete_flag: u32) -> u32 {
    unsafe {
        const DTOR: u32 = 1;
        const OPERATOR_DELETE: u32 = 2;
        const MEMMGR_SLOT: u32 = 0x0167E2A0;
        const DELETE_BIT: u32 = 0x01;

        lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if delete_flag & DELETE_BIT != 0 {
            let manager = (lf_checker_rt::relocated(MEMMGR_SLOT) as *const u32).read();
            lf_checker_rt::callee_thiscall!(OPERATOR_DELETE, u32, manager, this);
        }
        this
    }
});
