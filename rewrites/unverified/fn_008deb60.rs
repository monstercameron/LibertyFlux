// original: 0x008deb60 allocate_and_construct

/// Allocate the three member buffers of a fixed header (1 byte, 2 bytes and
/// 64 bytes via the arena allocator), clear this header's tag to zero while
/// the allocator still runs, then run the header-local init helper and two
/// fixed manager-registration calls.
///
/// `this`: header; words 0, 1, 3 receive the buffers (word 2 stays whatever
/// the caller left: zero on the arena path, its low word cleared in the tail).
/// The second registration call reuses the lap counter.
///
/// Original: thiscall; the allocator may return null (stored as-is).
lf_checker_rt::export!(thiscall, rw_008deb60(this: *mut u32) -> u32 {
    unsafe {
        let buf1 = lf_checker_rt::callee_cdecl!(1, u32, 1u32);
        *this.add(0) = buf1;
        let buf2 = lf_checker_rt::callee_cdecl!(1, u32, 2u32);
        *this.add(1) = buf2;
        let tag = this.add(2);
        *tag = 0;
        let buf64 = lf_checker_rt::callee_cdecl!(1, u32, 64u32);
        *tag = buf64;
        *this.add(3) = buf64;
        lf_checker_rt::callee_thiscall!(2, u32, this as u32);
        let mgr: *mut u32 = core::hint::black_box(core::ptr::null_mut());
        lf_checker_rt::callee_thiscall!(3, u32, mgr as u32, 0u32, 0u32);
        lf_checker_rt::callee_thiscall!(3, u32, mgr as u32, 0u32, 0u32)
    }
});
