// original: 0x00DB7360 UIFontString::vf121

/// Store the given float bits into the field at `+1E0`.
///
/// `thiscall`, one stack word. Leaf: no calls, no globals. Void. Bits pass through SSE (movss) verbatim.
lf_checker_rt::export!(thiscall, rw_00db7360(this_ptr: u32, value_bits: u32) -> u32 {
    const FIELD: u32 = 0x1E0;
    unsafe { ((this_ptr + FIELD) as *mut u32).write_unaligned(value_bits) };
    0
});
