// original: 0x00e63870 notify_code_ptr_3
/// Pass the code pointer at 0xE71470 to the notify helper (cdecl/1).
export!(cdecl, rw_00e63870() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE71470)) }
});
