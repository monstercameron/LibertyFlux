// original: 0x00e63820 notify_code_ptr_2
/// Pass the code pointer at 0xE71460 to the notify helper (cdecl/1).
export!(cdecl, rw_00e63820() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE71460)) }
});
