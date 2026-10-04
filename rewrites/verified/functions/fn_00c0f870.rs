// original: 0x00c0f870 UILayoutFrame::vf20
/// Forward field `0x1c` of this layout frame, the incoming argument and
/// the constant 0x80 to the engine helper (one intercepted call).
/// Returns the helper's answer.
export!(thiscall, rw_00c0f870(this: *mut u8, arg: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, this.add(0x1c) as u32, arg, 0x80) }
});
