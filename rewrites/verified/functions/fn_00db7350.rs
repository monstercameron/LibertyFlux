// original: 0x00DB7350 UIRowLayout::vf134

/// Store the given word into the field at `+1F4`.
///
/// `thiscall`, one stack word. Leaf: no calls, no globals. Void.
lf_checker_rt::export!(thiscall, rw_00db7350(this_ptr: u32, value: u32) -> u32 {
    const FIELD: u32 = 0x1F4;
    unsafe { ((this_ptr + FIELD) as *mut u32).write_unaligned(value) };
    0
});
