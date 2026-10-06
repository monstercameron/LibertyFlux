// original: 0x00DB7330 UIFontString::vf123

/// Store the given float bits into the field at `+1E8`.
///
/// `thiscall`, one stack word. Leaf: no calls, no globals. Void. Bits pass through SSE (movss) verbatim.
lf_checker_rt::export!(thiscall, rw_00db7330(this_ptr: u32, value_bits: u32) -> u32 {
    const FIELD: u32 = 0x1E8;
    unsafe { ((this_ptr + FIELD) as *mut u32).write_unaligned(value_bits) };
    0
});
