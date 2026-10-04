// original: 0x00891800 aud_global_update_thunk
/// Tail-jumps to the shared update routine with the global sound object.
///
/// The listed 117-byte range is wrong: the function is these 10 bytes plus
/// padding (the inventory's own Ghidra evidence agrees), and the following
/// bytes belong to the next function. Verified through the checker's tail-jump
/// patching with the callee's answer returned.
export!(thiscall, rw_00891800(_this: *mut u8) -> u32 {
    unsafe { callee_thiscall!(1, u32, relocated(0x115d8a0)) }
});
