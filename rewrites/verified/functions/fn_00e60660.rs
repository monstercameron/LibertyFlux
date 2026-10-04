// original: 0x00e60660 init_lock_and_forward_e6f980
/// Initialises a global critical section, then forwards one fixed
/// callback address to the shared mainloop helper.
///
/// The lock lives at `0x019f3368`; the initialisation call is made
/// through the import slot (stdcall/1, callee cleanup) and its answer
/// is ignored. The helper call (cdecl/1) carries the relocated address
/// `0x00e6f980` and its answer is returned unchanged.
export!(cdecl, rw_00e60660() -> u32 {
    unsafe {
        callee_stdcall!(2, u32, relocated(0x019F3368));
        callee_cdecl!(1, u32, relocated(0x00E6F980))
    }
});
