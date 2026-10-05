// original: 0x008dfdc0 free_buffers_and_release

/// Free an entry's heap buffers and release its two neighbours: the first two
/// buffers are freed only when `this` is non-null (a null entry skips them),
/// the third buffer is freed only when its slot is non-zero, then the release
/// helper runs twice, once for the header at `this + 0x30` and once (as a
/// tail call) for the header at `this + 0x3c`.
///
/// Every freed slot is cleared to zero after its call. A null `this` faults
/// on the third-slot read, like the original.
///
/// Original: thiscall, nullable this; returns the tail call's answer.
lf_checker_rt::export!(thiscall, rw_008dfdc0(this: *mut u32) -> u32 {
    unsafe {
        if !this.is_null() {
            lf_checker_rt::callee_cdecl!(1, u32, *this);
            lf_checker_rt::callee_cdecl!(1, u32, *this.add(1));
            *this = 0;
            *this.add(1) = 0;
        }
        let third = *this.add(3);
        if third != 0 {
            lf_checker_rt::callee_cdecl!(1, u32, third);
            *this.add(3) = 0;
        }
        lf_checker_rt::callee_thiscall!(2, u32, this.byte_add(0x30) as u32);
        lf_checker_rt::callee_thiscall!(3, u32, this.byte_add(0x3c) as u32)
    }
});
