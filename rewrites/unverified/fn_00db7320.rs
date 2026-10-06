// original: 0x00DB7320 UIFontString::vf130

/// Store the caller's 32-bit value into the style word at `+0x1DC`.
///
/// `thiscall`: object in ECX, source pointer as the single stack word; the
/// pointed-to word is copied. Returns the stored value (left in EAX). Leaf.
lf_checker_rt::export!(thiscall, rw_00db7320(this_ptr: u32, src_ptr: u32) -> u32 {
    const STYLE_WORD: u32 = 0x1DC;
    let v = unsafe { (src_ptr as *const u32).read_unaligned() };
    unsafe { ((this_ptr + STYLE_WORD) as *mut u32).write_unaligned(v) };
    v
});
