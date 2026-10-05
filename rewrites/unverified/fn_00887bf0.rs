// original: 0x00887BF0 stream_reset_view (proposed)

/// Tear down the stream's view and reset its position fields.
///
/// Calls the view-release entry (callee 1) with `[this+0xc], 0, [this+4]`,
/// copies the word at `+0x42` to `+0x44`, moves `[this+0x18]` to `+0x1c`,
/// then zeroes `+0x3c`, `+0x18`, `+0x14` and the word at `+0x46`, sets
/// `+0x40` to -1, and calls the drain entry (callee 2) with 0. The answer
/// is the drain entry's answer.
///
/// Original: 0x00887BF0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00887BF0(this: u32) -> u32 {
    unsafe {
        const RELEASE: u32 = 1;
        const DRAIN: u32 = 2;
        let a = ((this + 0x0c) as *const u32).read_unaligned();
        let b = ((this + 0x04) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(RELEASE, u32, a, 0, b);
        let w = ((this + 0x42) as *const u16).read_unaligned();
        ((this + 0x44) as *mut u16).write_unaligned(w);
        let pos = ((this + 0x18) as *const u32).read_unaligned();
        ((this + 0x3c) as *mut u32).write_unaligned(0);
        ((this + 0x1c) as *mut u32).write_unaligned(pos);
        ((this + 0x40) as *mut u32).write_unaligned(0xffff_ffff);
        ((this + 0x18) as *mut u32).write_unaligned(0);
        ((this + 0x14) as *mut u32).write_unaligned(0);
        ((this + 0x46) as *mut u16).write_unaligned(0);
        lf_checker_rt::callee_cdecl!(DRAIN, u32, 0)
    }
});
