// original: 0x005B4690 fixed_object_forward
/// Forwards one argument to the shared fixed-object handler.
export!(cdecl, rw_005B4690(arg: u32) -> u32 {
    callee_thiscall!(1, u32, relocated(0x019D2DC0), arg)
});
