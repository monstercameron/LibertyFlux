// original: 0x00cca8e0 CTaskComplexInjuredOnGround::vf1
/// Clone/create helper, `CTaskComplexInjuredOnGround` vtable slot 1
/// (thiscall/0).
///
/// Allocates a fresh object through the global task pool (thiscall/0);
/// returns null when allocation fails. Otherwise initialises the fresh
/// object from the dwords at `this+0x1c` and `this+0x20` (thiscall/2) and
/// returns the initialiser's answer.
export!(thiscall, rw_rs01_cca8e0(this: *const u8) -> u32 {
    unsafe {
        let pool = *global::<u32>(TASK_POOL);
        let alloc: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let fresh = alloc(pool);
        if fresh == 0 {
            return 0;
        }
        let arg1 = *((this.add(0x1C)) as *const u32);
        let arg2 = *((this.add(0x20)) as *const u32);
        let init: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        init(fresh, arg1, arg2)
    }
});
