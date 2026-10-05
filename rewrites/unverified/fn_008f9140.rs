// original: 0x008F9140 stream_mgr_startup
/// Start up the streaming manager.
///
/// Initializes the global manager object, runs the table setup, then
/// tail-calls the pool startup. Cdecl, no arguments; returns the tail
/// call's result.
export!(cdecl, rw_008f9140() -> u32 {
    unsafe {
        const MGR: u32 = 0x1033130;
        callee_thiscall!(1, u32, relocated(MGR));
        callee_cdecl!(2, u32,);
        callee_cdecl!(3, u32,)
    }
});
