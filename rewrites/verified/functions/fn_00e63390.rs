// original: 0x00e63390 init_callback_e713e0
/// Invokes the shared init helper (cdecl/1, stubbed) with a fixed address.
///
/// Passes the relocated address 0xE713E0 and returns the helper's answer.
export!(cdecl, rw_00e63390() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE713E0)) }
});
