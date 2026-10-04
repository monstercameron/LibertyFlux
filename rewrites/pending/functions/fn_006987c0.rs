// original: 0x006987c0 rage::crCreatureComponentSkeleton::vf6
/// Forward a skeleton notification with the joint-set handle.
///
/// Passes the dword at offset 4 of the object at offset 0xc, three zero
/// words and the second argument to the shared handler, with ECX set to the
/// first argument. Returns its answer.
export!(thiscall, rw_006987c0(this: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        let inner = *((this as *const u32).add(3));
        let handle = *((inner as *const u32).add(1));
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        f(arg1, handle, 0, 0, 0, arg2)
    }
});
