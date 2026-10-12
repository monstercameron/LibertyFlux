// original: 0x00cca8b0 CTaskComplexHitResponse::vf1
/// Clone/create helper, `CTaskComplexHitResponse` vtable slot 1 (thiscall/0).
///
/// Allocates a fresh object through the global task pool (thiscall/0);
/// returns null when allocation fails. Otherwise initialises the fresh
/// object from the dword at `this+0x14` (thiscall/1) and returns the
/// initialiser's answer. No merged field name covers `+0x14` as a dword.
export!(thiscall, rw_rs01_cca8b0(this: *const u8) -> u32 {
    unsafe {
        let pool = *global::<u32>(TASK_POOL);
        let alloc: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(__lf_callee(1) as usize);
        let fresh = alloc(pool);
        if fresh == 0 {
            return 0;
        }
        let arg = *((this.add(0x14)) as *const u32);
        let init: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(__lf_callee(2) as usize);
        init(fresh, arg)
    }
});

const TASK_POOL: u32 = 0x167E2A0;
