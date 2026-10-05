// original: 0x00e6c570 veh_obj_forward_2
/// Forward to the object routine with a static object (tail jump).
///
/// Sets ECX to the static object at `OBJ` (0x0171CF88) and tail-jumps to
/// the object routine (thiscall/0), whose result becomes this
/// function's result. The rewrite expresses the tail jump as a call
/// that forwards the argument and result. Takes no arguments.
///
/// Original: 0x00E6C570, cdecl, no arguments.
export!(cdecl, rw_00e6c570() -> u32 {
    unsafe {
        const OBJ: u32 = 0x171CF88;
        callee_thiscall!(1, u32, relocated(OBJ))
    }
});
