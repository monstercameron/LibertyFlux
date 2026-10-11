// original: 0x00C4A8E0 CCamCinematicHeliChase::~CCamCinematicHeliChase

/// Replace the object's virtual table, detach its task state, and run the
/// base camera destructor.
///
/// The receiver is in ECX. The function stores its class table pointer at
/// offset zero, calls the task teardown helper with the receiver, then
/// forwards the receiver to the base destructor. Both helper calls are
/// intercepted by the contract. EAX is not a defined result of this
/// destructor and is omitted from comparison.
lf_checker_rt::export!(thiscall, rw_c4a8e0(this: u32) -> u32 {
    const VTABLE_POINTER: u32 = 0x00ec_98a4;
    const TASK_TEARDOWN: u32 = 1;
    const BASE_DESTRUCTOR: u32 = 2;

    unsafe { (this as *mut u32).write_unaligned(VTABLE_POINTER) };
    let _ = lf_checker_rt::callee_thiscall!(TASK_TEARDOWN, u32, this);
    let _ = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this);
    0
});
