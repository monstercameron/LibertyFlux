// original: 0x00cad390 CTaskComplexGetOutOfWater::vf0
/// Scalar deleting destructor (MSVC `vf0`): destroy the task, then free it.
///
/// `this` (ECX) is the task object; `flags` bit 0 selects the deleting form.
/// Runs the class destructor through ECX (intercepted callee 1, thiscall/0),
/// and when bit 0 of `flags` is set releases the object through the game
/// allocator global at `0x0167E2A0` (intercepted callee 2, thiscall/1 with
/// the allocator in ECX and `this` pushed). Other flag bits are ignored.
/// Returns `this` unchanged. Original is thiscall(`this`, `flags`).
lf_checker_rt::export!(thiscall, rw_00cad390(this: u32, flags: u32) -> u32 {
    unsafe {
        const FREE_FLAG: u32 = 0x0000_0001;
        const ALLOCATOR_GLOBAL: u32 = 0x0167_E2A0;
        const DTOR: u32 = 1;
        const RELEASE: u32 = 2;
        let _: u32 = lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if flags & FREE_FLAG != 0 {
            let mgr = (lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL) as *const u32).read();
            let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE, u32, mgr, this);
        }
        this
    }
});
