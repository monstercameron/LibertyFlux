// original: 0x00B12D60 CObject::~CObject__deleting

/// Finish the common CObject teardown, optionally releasing the object from
/// the world-object pool, and return the original object pointer.
///
/// `this` is the object pointer and `deleting` is the destructor flag word.
/// The tracker helper always receives `this`; when bit zero is set, the pool
/// helper receives the tracker singleton in ECX and `this` as its one stack
/// argument. Both helpers are external effects represented by checker stubs.
/// The routine uses the Microsoft x86 thiscall convention and returns `this`.
lf_checker_rt::export!(thiscall, rw_00B12D60(this: u32, deleting: u32) -> u32 {
    const TRACKER_GLOBAL_VA: u32 = 0x0163_2c60;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    if deleting & 1 != 0 {
        let tracker = unsafe { lf_checker_rt::global::<u32>(TRACKER_GLOBAL_VA).read_unaligned() };
        let _ = lf_checker_rt::callee_thiscall!(2, u32, tracker, this);
    }
    this
});
