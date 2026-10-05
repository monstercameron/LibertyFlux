// original: 0x00b1e4c0 free_list_deep (proposed)

/// Frees a counted pointer list and its elements, then clears the head.
///
/// Thiscall with no stack arguments. Frees the head pointer at +0, clears
/// +0 and +4, then frees each of the count (u16 at +0x0E) element
/// pointers stored 8 bytes apart starting at the array at +8, frees the
/// array itself, and clears +8 and +0x0C. The free routine is cdecl of
/// one word. Returns 0.
lf_checker_rt::export!(thiscall, rw_00b1e4c0(this: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x00;
        const SPARE: u32 = 0x04;
        const ARRAY: u32 = 0x08;
        const TAIL: u32 = 0x0c;
        const COUNT: u32 = 0x0e;
        const ELEM_STRIDE: u32 = 8;
        lf_checker_rt::callee_cdecl!(1, u32, ((this + HEAD) as *const u32).read_unaligned());
        ((this + HEAD) as *mut u32).write_unaligned(0);
        ((this + SPARE) as *mut u32).write_unaligned(0);
        let count = ((this + COUNT) as *const u16).read_unaligned() as u32;
        let array = ((this + ARRAY) as *const u32).read_unaligned();
        let mut i = 0u32;
        while i < count {
            let elem = ((array.wrapping_add(i.wrapping_mul(ELEM_STRIDE))) as *const u32)
                .read_unaligned();
            lf_checker_rt::callee_cdecl!(1, u32, elem);
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(1, u32, array);
        ((this + ARRAY) as *mut u32).write_unaligned(0);
        ((this + TAIL) as *mut u32).write_unaligned(0);
        0
    }
});
