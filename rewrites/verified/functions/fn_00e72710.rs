// original: 0x00e72710 ecx_thunk_00e72710
/// Entry thunk: select the shared object, then hand off.
///
/// Takes no arguments and reads no registers. Loads the shared object's
/// address and transfers control to the shared implementation with it,
/// returning whatever that call answers.
export!(cdecl, rw_00e72710() -> u32 {
    callee_thiscall!(1, u32, relocated(0x0166D9C0))
});
