// original: 0x00cca540 CTaskComplexDie::~CTaskComplexDie__deleting
/// Scalar deleting destructor of `CTaskComplexDie` (thiscall/1).
///
/// Runs the plain destructor (thiscall/0), then frees the object through the
/// global task pool when the low flag bit is set (thiscall/1: pool in ECX,
/// object pushed). Returns `this`.
export!(thiscall, rw_rs01_cca540(this: *mut u8, flags: u32) -> u32 {
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
