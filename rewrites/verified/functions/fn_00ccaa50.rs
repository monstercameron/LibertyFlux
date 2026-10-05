// original: 0x00ccaa50 CTaskSimpleHitFromBack::vf1
/// Clone/create helper, `CTaskSimpleHitFromBack` vtable slot 1 (thiscall/0).
///
/// Note: the original never reads its own `this` pointer; it only uses the
/// global task pool. Allocates a fresh object (thiscall/0) and returns null
/// when allocation fails. Otherwise initialises the fresh object
/// (thiscall/8) with fixed constants (including a relocated table address
/// and the floats 1.0 and 4.0), installs the class vtable on the fresh
/// object, and returns it. The `(an instruction of the original)` / overwrite pair in the original
/// is a compiler stack-slot quirk: the pushed register value is overwritten
/// before the call and never observed.
export!(thiscall, rw_rs01_ccaa50(_this: *const u8) -> u32 {
    unsafe {
        let pool = *global::<u32>(TASK_POOL);
        let alloc: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let fresh = alloc(pool) as *mut u8;
        if fresh.is_null() {
            return 0;
        }
        let init: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        init(
            fresh as u32,
            0x2D,
            0xC2,
            0x40800000,
            0x193,
            relocated(0xED9D38),
            0,
            0x3F800000,
            0,
        );
        *(fresh as *mut u32) = relocated(0xED9CE4);
        fresh as u32
    }
});
