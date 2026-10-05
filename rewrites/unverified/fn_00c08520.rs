// original: 0x00c08520 stream_teardown (proposed)

/// Tear the streaming list down once, through its three helpers.
///
/// `this` points to the list. The opener (callee 1) always runs first with
/// the image pointer `MAGIC` (relocated) and zero; when the done byte at `DONE` is already set
/// nothing else happens. Otherwise the releaser (callee 2) receives the buffer
/// at `BUF` and the count at `COUNT`, the buffer and the count dword are
/// cleared, the closer (callee 3, no arguments) runs, and the done byte is
/// set. Returns the opener's answer with its low byte forced to 1.
///
/// Original: 0x00c08520 (thiscall, no stack words; opener is cdecl).
lf_checker_rt::export!(thiscall, rw_00c08520(this: u32) -> u32 {
    unsafe {
        const MAGIC: u32 = 0xebde9c;
        const BUF: u32 = 0x00;
        const COUNT: u32 = 0x06;
        const COUNT_DW: u32 = 0x04;
        const DONE: u32 = 0x08;
        const OPEN: u32 = 1;
        const RELEASE: u32 = 2;
        const CLOSE: u32 = 3;
        let r: u32 =
            lf_checker_rt::callee_cdecl!(OPEN, u32, lf_checker_rt::relocated(MAGIC), 0);
        if (this.wrapping_add(DONE) as *const u8).read() == 0 {
            let buf = (this.wrapping_add(BUF) as *const u32).read_unaligned();
            let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
            let _s: u32 = lf_checker_rt::callee_thiscall!(RELEASE, u32, this, buf, count);
            (this.wrapping_add(BUF) as *mut u32).write_unaligned(0);
            (this.wrapping_add(COUNT_DW) as *mut u32).write_unaligned(0);
            let _t: u32 = lf_checker_rt::callee_thiscall!(CLOSE, u32, this);
            (this.wrapping_add(DONE) as *mut u8).write(1);
        }
        (r & 0xffff_ff00) | 1
    }
});
