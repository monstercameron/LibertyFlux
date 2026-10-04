// original: 0x00ab6f70 object_kind_is_36
/// Fetches the shared registry object and reports whether its field at +8
/// equals 36.

export!(cdecl, rw_00ab6f70() -> u32 {
    let obj: u32 = callee_thiscall!(1, u32, relocated(0x11D4EC8));
    unsafe { ((*((obj + 8) as *const u32)) == 0x24) as u32 }
});
