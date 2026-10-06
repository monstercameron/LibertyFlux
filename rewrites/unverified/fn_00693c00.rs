// original: 0x00693C00 track_slot_by_id (proposed)

/// Linear search of a pointer array: `this` points at a container whose
/// dword at +0 is the array base and whose 16-bit word at +4 is the element
/// count (compared unsigned). Returns the first element whose dword at +4
/// equals `id`, or null.
///
/// Original: 0x00693C00 (thiscall, one stack argument, callee-cleanup).
lf_checker_rt::export!(thiscall, rw_00693C00(this: u32, id: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }

        let count = rd16(this.wrapping_add(4)) as u32;
        let base = rd32(this);
        let mut i = 0u32;
        while i < count {
            let elem = rd32(base.wrapping_add(i.wrapping_mul(4)));
            if rd32(elem.wrapping_add(4)) == id {
                return elem;
            }
            i = i.wrapping_add(1);
        }
        0
    }
});
