// original: 0x00a05d50 NativeImpl_PLACE_OBJECT_RELATIVE_TO_CAR (native)
/// Place an object at a car-relative offset.
///
/// Resolves `handle` through the object pool with the offset vector
/// [x, y, z], resolves that against `car` through pool A, and commits the
/// placement. Returns the commit answer. Cdecl, five words.
lf_checker_rt::export!(cdecl, rw_00a05d50(handle: u32, car: u32, x: u32, y: u32, z: u32) -> u32 {
    unsafe {
        const OBJ_POOL: u32 = 0x01632c60;
        const POOL_A: u32 = 0x012e22a4;
        const LOOKUP_VEC: u32 = 0;
        const LOOKUP_PAIR: u32 = 1;
        const COMMIT: u32 = 2;
        let pool0 = (lf_checker_rt::global::<u32>(OBJ_POOL) as *const u32).read_unaligned();
        let pool1 = (lf_checker_rt::global::<u32>(POOL_A) as *const u32).read_unaligned();
        let off = [x, y, z];
        let o1 = lf_checker_rt::callee_thiscall!(LOOKUP_VEC, u32, pool0, handle, off.as_ptr() as u32);
        let o2 = lf_checker_rt::callee_thiscall!(LOOKUP_PAIR, u32, pool1, car, o1);
        lf_checker_rt::callee_cdecl!(COMMIT, u32, o2)
    }
});
