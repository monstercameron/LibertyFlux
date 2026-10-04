// original: 0x00cca9f0 CTaskSimpleDie::vf1
/// Clone/create helper, `CTaskSimpleDie` vtable slot 1 (thiscall/0).
///
/// Allocates a fresh object through the global task pool (thiscall/0);
/// returns null when allocation fails. Otherwise initialises the fresh
/// object (thiscall/7) from the dwords at `this+0x14` and `this+0x18`, the
/// four float words at `this+0x20..0x30`, and the constant `0xBDCCCCCD`
/// (about -0.1 as a float), and returns the initialiser's answer. The
/// original moves the float words through SSE registers without arithmetic,
/// so plain dword copies are bit-exact.
export!(thiscall, rw_rs01_cca9f0(this: *const u8) -> u32 {
    unsafe {
        let pool = *global::<u32>(TASK_POOL);
        let alloc: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let fresh = alloc(pool);
        if fresh == 0 {
            return 0;
        }
        let w = |off: usize| *((this.add(off)) as *const u32);
        let init: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        init(
            fresh,
            w(0x14),
            w(0x18),
            w(0x20),
            w(0x24),
            w(0x2C),
            w(0x30),
            0xBDCCCCCD,
        )
    }
});
