// original: 0x005d5990 html_string_or_default
/// Return the string pointer at `+0xe0` when the presence flag at `+0xe4`
/// is set, otherwise the shared default string.
export!(thiscall, rw_005d5990(this_ptr: u32) -> u32 {
    let present = unsafe { ((this_ptr + 0xE4) as *const u16).read() };
    if present != 0 {
        unsafe { ((this_ptr + 0xE0) as *const u32).read() }
    } else {
        relocated(0x00FC9C85)
    }
});
