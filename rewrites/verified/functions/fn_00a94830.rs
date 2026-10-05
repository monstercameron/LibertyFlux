// original: 0x00a94830 stream_set_source (proposed)

/// Attach a source path to a stream object: 1 on success, 0 on refusal.
///
/// Stores the tag at `this+0x04`, clears the flag byte at `this+0x0c`, and
/// offers `(path, this, 0)` to the open callee. A zero low byte refuses
/// with 0. Otherwise a null path stores length 0, else the path's byte
/// length (up to its NUL terminator) is stored at `this+0x08`; both give 1.
/// Only `al` carries the result.
///
/// Original: thiscall, two stack arguments (tag, path).
/// One callee (cdecl, 3 args).
lf_checker_rt::export!(thiscall, rw_00a94830(this: u32, tag: u32, path: u32) -> u8 {
    unsafe {
        const TAG: u32 = 0x04;
        const LENGTH: u32 = 0x08;
        const FLAG: u32 = 0x0c;
        const OPEN: u32 = 0;
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        wr32(this.wrapping_add(TAG), tag);
        wr8(this.wrapping_add(FLAG), 0);
        let accepted: u32 = lf_checker_rt::callee_cdecl!(OPEN, u32, path, this, 0);
        if accepted & 0xff == 0 {
            return 0;
        }
        if path == 0 {
            wr32(this.wrapping_add(LENGTH), 0);
            return 1;
        }
        let mut len: u32 = 0;
        while rd8(path.wrapping_add(len)) != 0 {
            len = len.wrapping_add(1);
        }
        wr32(this.wrapping_add(LENGTH), len);
        1
    }
});
