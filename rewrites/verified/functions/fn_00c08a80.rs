// original: 0x00c08a80 stream_name_set (proposed)

/// Copy the caller's string into the streaming slot and clear its flag.
///
/// `this` points to the slot; when `name` is null or empty nothing happens
/// and `name` is returned. Otherwise the copier (callee 1) receives the
/// slot's buffer at `BUF`, the string and the constant `MAX`, the flag byte
/// at `FLAG` is cleared, and the copier's answer is returned.
///
/// Original: 0x00c08a80 (thiscall, one stack word; copier is cdecl, 3 args).
lf_checker_rt::export!(thiscall, rw_00c08a80(this: u32, name: u32) -> u32 {
    unsafe {
        const BUF: u32 = 0x2c;
        const FLAG: u32 = 0x4b;
        const MAX: u32 = 0x20;
        const COPY: u32 = 1;
        if name == 0 {
            return 0;
        }
        if (name as *const u8).read() == 0 {
            return name;
        }
        let r: u32 =
            lf_checker_rt::callee_cdecl!(COPY, u32, this.wrapping_add(BUF), name, MAX);
        (this.wrapping_add(FLAG) as *mut u8).write(0);
        r
    }
});
