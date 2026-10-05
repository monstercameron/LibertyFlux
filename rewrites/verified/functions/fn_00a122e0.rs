// original: 0x00a122e0 follow_cam_construct (proposed)
/// Construct a follow-camera object in place.
///
/// Runs the base constructor on `this`, installs the follow-camera virtual
/// table, clears the words at `+0x320` and `+0x324`, then runs the second
/// initialiser on `this`. Returns `this`. Thiscall, no stack arguments.
export!(thiscall, rw_00a122e0(this: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const INIT: u32 = 2;
        const VTABLE: u32 = 0x00e9af3c;
        callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(relocated(VTABLE));
        ((this + 0x320) as *mut u32).write_unaligned(0);
        ((this + 0x324) as *mut u32).write_unaligned(0);
        callee_thiscall!(INIT, u32, this);
        this
    }
});
