// original: 0x00e637b0 notify_code_ptr_1
/// Pass the code pointer at 0xE71450 to the notify helper (cdecl/1).
///
/// Returns the helper's answer. Same shape covers the two sibling wrappers
/// below at neighbouring immediates.
export!(cdecl, rw_00e637b0() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE71450)) }
});
