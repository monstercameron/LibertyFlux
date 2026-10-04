// original: 0x00e5ea40 network_static_init_07
/// Initialise one static network object and register its callback.
///
/// Calls the object routine (stubbed, cdecl/0), then passes the callback
/// thunk `0x00e6f160` to the registrar helper (stubbed, cdecl/1). Takes no
/// arguments; entry registers are ignored. Returns the registrar's answer.
export!(cdecl, rw_00e5ea40() -> u32 {
    unsafe {
        let _: u32 = callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(0xE6F160))
    }
});
