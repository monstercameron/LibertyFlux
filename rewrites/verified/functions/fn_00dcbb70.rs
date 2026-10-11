// original: 0x00DCBB70 CTaskComplexMoveAvoidOtherPedWhileWandering::~CTaskComplexMoveAvoidOtherPedWhileWandering
/// Restores the two destructor vtable slots, conditionally releases the
/// nested task handle through its field address, and forwards destruction to
/// the base task. The optional helper receives the nested object in ECX and
/// the address of the task's handle field as its sole stack argument.
lf_checker_rt::export!(thiscall, rw_00dcbb70(task: u32) -> () {
    const TASK_VTABLE: u32 = 0x00ef54bc;
    const DESTRUCTOR_VTABLE: u32 = 0x00ef5514;
    const HANDLE_OFFSET: usize = 0x20;
    const RELEASE_ID: u32 = 1;
    const BASE_DESTROY_ID: u32 = 2;
    let handle_slot = task.wrapping_add(HANDLE_OFFSET as u32);
    // SAFETY: the contract supplies a readable and writable task object.
    let handle = unsafe { (handle_slot as *const u32).read() };
    // SAFETY: the contract supplies a writable task object.
    unsafe {
        (task as *mut u32).write(lf_checker_rt::relocated(TASK_VTABLE));
        ((task as *mut u8).add(0x14) as *mut u32).write(lf_checker_rt::relocated(DESTRUCTOR_VTABLE));
    }
    if handle != 0 {
        let _ = lf_checker_rt::callee_thiscall!(RELEASE_ID, u32, handle, handle_slot);
    }
    let _ = lf_checker_rt::callee_thiscall!(BASE_DESTROY_ID, u32, task);
});
