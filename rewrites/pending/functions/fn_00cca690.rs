// original: 0x00cca690 CTaskSimpleDie::~CTaskSimpleDie__deleting
/// Scalar deleting destructor of `CTaskSimpleDie` (thiscall/1).
///
/// Runs the plain destructor (thiscall/0), then frees the object through the
/// global task pool when the low flag bit is set. Returns `this`.
export!(thiscall, rw_rs01_cca690(this: *mut u8, flags: u32) -> u32 {
    unsafe {
        let base_dtor: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        base_dtor(this as u32);
        if flags & 1 != 0 {
            let pool = *global::<u32>(TASK_POOL);
            let free_task: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(callee_addr(2) as usize);
            free_task(pool, this as u32);
        }
        this as u32
    }
});
