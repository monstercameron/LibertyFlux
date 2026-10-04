// original: 0x00ca4f10 CTaskSimpleHitWall::vf1
/// Builds a hit-wall task object: fetches the global task factory, asks it
/// for a fresh object, initialises it with constant parameters (two float
/// bit patterns, a kind code, a name pointer) and installs its vtable.
/// Returns the new object, or null when the factory has none.
lf_rs75_rt::export!(cdecl, rw_00ca4f10() -> u32 {
    unsafe {
        let mgr = *lf_rs75_rt::global::<u32>(0x0167E2A0);
        let obj: u32 = lf_rs75_rt::callee_thiscall!(1, u32, mgr);
        if obj == 0 {
            return 0;
        }
        const KIND: u32 = 0x19B;
        const SPEED: u32 = 0x3F800000; // 1.0f bits
        const RANGE: u32 = 0x40800000; // 4.0f bits
        let _: u32 = lf_rs75_rt::callee_thiscall!(
            2,
            u32,
            obj,
            0,
            1,
            RANGE,
            KIND,
            lf_rs75_rt::relocated(0x00ED7B48),
            0,
            SPEED,
            0
        );
        *(obj as *mut u32) = lf_rs75_rt::relocated(0x00ED7AF4);
        obj
    }
});
