// original: 0x00DB7050 UIFontString::vf133

/// Copy the 32-bit style word at `+0x1DC` into the caller's out slot.
///
/// `thiscall`: object in ECX, out pointer as the single stack word. Returns
/// the out pointer (left in EAX by the load). Leaf: no calls, no globals.
lf_checker_rt::export!(thiscall, rw_00db7050(this_ptr: u32, out_ptr: u32) -> u32 {
    const STYLE_WORD: u32 = 0x1DC;
    let v = unsafe { ((this_ptr + STYLE_WORD) as *const u32).read_unaligned() };
    unsafe { (out_ptr as *mut u32).write_unaligned(v) };
    out_ptr
});
