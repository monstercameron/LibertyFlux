// original: 0x00e6c510 veh_obj_forward_1
/// Forward to the object routine with a static object (tail jump).
///
/// Sets ECX to the static object at `OBJ` (0x0171D004) and tail-jumps to
/// the object routine (thiscall/0), whose result becomes this
/// function's result. The rewrite expresses the tail jump as a call
/// that forwards the argument and result. Takes no arguments.
///
/// Original: 0x00E6C510, cdecl, no arguments.
export!(cdecl, rw_00e6c510() -> u32 {
    unsafe {
        const OBJ: u32 = 0x171D004;
        callee_thiscall!(1, u32, relocated(OBJ))
    }
});
