// original: 0x00DB7390 UIFontString::vf124

/// Store the given float bits into the field at `+1EC`.
///
/// `thiscall`, one stack word. Leaf: no calls, no globals. Void. Bits pass through SSE (movss) verbatim.
lf_checker_rt::export!(thiscall, rw_00db7390(this_ptr: u32, value_bits: u32) -> u32 {
    const FIELD: u32 = 0x1EC;
    unsafe { ((this_ptr + FIELD) as *mut u32).write_unaligned(value_bits) };
    0
});
