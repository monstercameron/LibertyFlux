// original: 0x00951080 quad_teardown_calls
/// Runs a fixed four-call teardown sequence and returns the last answer.
///
/// Two parameterless releases run first, then a flagged release on one global
/// object and an unflagged release on another. Only the final answer is kept.
export!(cdecl, rw_00951080() -> u32 {
    unsafe {
        const FIRST_OBJ: u32 = 0x0117_6888;
        const SECOND_OBJ: u32 = 0x0116_2630;
        callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32,);
        callee_thiscall!(3, u32, relocated(FIRST_OBJ), 0xFFFF_FFFF);
        callee_thiscall!(4, u32, relocated(SECOND_OBJ), 0)
    }
});
