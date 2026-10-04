// original: 0x005d5a60 html_text_format_is_empty
/// Return whether the word at `+0x238` is zero.
export!(thiscall, rw_005d5a60(this_ptr: u32) -> u32 {
    (unsafe { ((this_ptr + 0x238) as *const u32).read() } == 0) as u32
});
