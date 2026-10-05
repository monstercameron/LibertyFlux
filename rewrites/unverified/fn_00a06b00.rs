// original: 0x00a06b00 NativeImpl_SET_OBJECT_INITIAL_VELOCITY (native)
/// Give an object its initial velocity vector.
///
/// Resolves `handle` through the object pool with the vector [x, y, z] and
/// applies it as the initial velocity. Returns the apply answer. Cdecl.
lf_checker_rt::export!(cdecl, rw_00a06b00(handle: u32, x: u32, y: u32, z: u32) -> u32 {
    unsafe {
        const OBJ_POOL: u32 = 0x01632c60;
        const LOOKUP: u32 = 0;
        const APPLY: u32 = 1;
        let pool = (lf_checker_rt::global::<u32>(OBJ_POOL) as *const u32).read_unaligned();
        let vel = [x, y, z];
        let obj = lf_checker_rt::callee_thiscall!(LOOKUP, u32, pool, handle, vel.as_ptr() as u32);
        lf_checker_rt::callee_thiscall!(APPLY, u32, obj)
    }
});
