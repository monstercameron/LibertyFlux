// original: 0x00d21130 slot_classifier_single_arg
// Forwards to the tick-slot classifier with a zero addend and returns its
// answer unchanged.
export!(cdecl, rw_00d21130(flag: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, flag, 0) }
});
