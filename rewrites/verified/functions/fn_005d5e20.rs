// original: 0x005d5e20 html_string_after_colon
/// Search the node's string (or the shared default when the presence flag
/// at `+0xe4` is clear) for `':'`; return the address just past it, or the
/// string itself when there is none.
export!(thiscall, rw_005d5e20(this_ptr: u32) -> u32 {
    let pick = |this_ptr: u32| unsafe {
        if ((this_ptr + 0xE4) as *const u16).read() != 0 {
            ((this_ptr + 0xE0) as *const u32).read()
        } else {
            relocated(0x00FC9C85)
        }
    };
    let p = pick(this_ptr);
    let q: u32 = callee_cdecl!(0, u32, p, 0x3A);
    if q != 0 {
        q.wrapping_add(1)
    } else {
        pick(this_ptr)
    }
});
