// original: 0x005d5e70 html_string_copy_token
/// Copy the node's string (or the shared default) into the caller buffer
/// up to the first `':'`, clamping the length at 63 bytes (signed
/// comparison, as in the original) and NUL-terminating. When there is no
/// colon the buffer is left empty. The second argument is unused.
/// Returns the buffer on the empty path, else the copier answer.
export!(thiscall, rw_005d5e70(this_ptr: u32, buf: u32, _unused: u32) -> u32 {
    let pick = |this_ptr: u32| unsafe {
        if ((this_ptr + 0xE4) as *const u16).read() != 0 {
            ((this_ptr + 0xE0) as *const u32).read()
        } else {
            relocated(0x00FC9C85)
        }
    };
    let p = pick(this_ptr);
    let q: u32 = callee_cdecl!(0, u32, p, 0x3A);
    if q == 0 {
        unsafe { (buf as *mut u8).write(0) };
        return buf;
    }
    let mut len = q.wrapping_sub(p);
    if (len as i32) > 0x3F {
        len = 0x3F;
    }
    let r: u32 = callee_cdecl!(1, u32, buf, pick(this_ptr), len);
    unsafe { (buf.wrapping_add(len) as *mut u8).write(0) };
    r
});
