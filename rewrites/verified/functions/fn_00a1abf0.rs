// original: 0x00a1abf0 CCamFollowPed::vf0 (symbols)

/// Destroys the camera object, freeing it when the flag asks.
///
/// Runs the chained destructor on `this`; when the low bit of `flag` is
/// set, the object is additionally passed to the freeing callee with the
/// global allocator as its `ecx`. Returns `this`.
///
/// Original: 0x00a1abf0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a1abf0(this: u32, flag: u32) -> u32 {
    unsafe {
        const C_DTOR: u32 = 1;
        const C_FREE: u32 = 2;
        const ALLOCATOR: u32 = 0x012f_b1a0;
        lf_checker_rt::callee_thiscall!(C_DTOR, u32, this);
        if flag & 1 != 0 {
            let alloc = lf_checker_rt::global::<u32>(ALLOCATOR).read_unaligned();
            lf_checker_rt::callee_thiscall!(C_FREE, u32, alloc, this);
        }
        this
    }
});
