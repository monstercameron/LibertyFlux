// original: 0x008deb60 allocate_and_construct

/// Allocate a header's three member buffers (two big, one small), run the
/// header-local init helper, register the two embedded sub-objects with the
/// manager, and clear the header tag's low word.
///
/// The allocator takes byte counts (`BIG`, `BIG`, `SMALL`); the answers land
/// in words 0, 1 and 3. Word 8 is cleared before the first registration; the
/// two registrations target `this + 0x30` and `this + 0x3c` with keys `0x2d`
/// and `0x96`. Only the low 16 bits of word 2 are cleared: its high half
/// keeps whatever the caller left.
///
/// Original: thiscall; returns the second registration's answer.
lf_checker_rt::export!(thiscall, rw_008deb60(this: *mut u32) -> u32 {
    unsafe {
        const BIG: u32 = 0x202710;
        const SMALL: u32 = 0x2710;
        *this = lf_checker_rt::callee_cdecl!(1, u32, BIG);
        *this.add(1) = lf_checker_rt::callee_cdecl!(1, u32, BIG);
        *this.add(3) = lf_checker_rt::callee_cdecl!(1, u32, SMALL);
        lf_checker_rt::callee_thiscall!(2, u32, this as u32);
        *this.add(8) = 0;
        lf_checker_rt::callee_thiscall!(3, u32, this.byte_add(0x30) as u32, 0x2du32, 1u32);
        let ans =
            lf_checker_rt::callee_thiscall!(3, u32, this.byte_add(0x3c) as u32, 0x96u32, 1u32);
        *(this.add(2) as *mut u16) = 0;
        ans
    }
});
