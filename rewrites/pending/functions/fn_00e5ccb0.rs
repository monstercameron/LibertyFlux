// original: 0x00E5CCB0 timer_init_h_register
/// Run one mainloop-timing initialiser, then register its callback.
///
/// Calls the initialiser (no stack arguments, callee id 1), discards its
/// answer, then registers the callback's code address with the registrar
/// (cdecl/1, callee id 2) and returns the registrar's answer. Takes no
/// arguments; entry registers are ignored.
export!(cdecl, rw_00e5ccb0() -> u32 {
    unsafe {
        let _init_answer: u32 = callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(0x00E6E640))
    }
});
