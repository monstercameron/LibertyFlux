// original: 0x00a12410 CCamFollowVehicle::vf0
/// Virtual destructor prologue: rebuild the object, then optionally free it.
///
/// Runs the follow-camera initialiser on `this`. When the low bit of `flag`
/// is set, also passes `this` to the freeing routine with the global heap.
/// Returns `this`. Thiscall, one stack argument.
export!(thiscall, rw_00a12410(this: u32, flag: u32) -> u32 {
    unsafe {
        const INIT: u32 = 1;
        const FREE: u32 = 2;
        const HEAP_GLOBAL: u32 = 0x012fb1a0;
        callee_thiscall!(INIT, u32, this);
        if (flag & 1) != 0 {
            let heap = *global::<u32>(HEAP_GLOBAL);
            callee_thiscall!(FREE, u32, heap, this);
        }
        this
    }
});
