// original: 0x00ca2a10 slot_set_maybe_face

/// Store a face slot and refresh the face when slot 2 changes.
///
/// Stores `val` at slot `idx` of the three words at `+0x10` and raises the
/// present flag at `+0x24`; indices of 3 or more (signed) store nothing.
/// When slot 2 is stored, a non-null value rebuilds the face (callee 1)
/// while a null value tears it down (callee 2). Returns `idx`, or the
/// rebuild/teardown answer on the slot-2 path.
///
/// Original: 0x00ca2a10 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00ca2a10(this: u32, idx: u32, val: u32) -> u32 {
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    #[inline(always)]
    unsafe fn wr8(a: u32, v: u8) {
        unsafe { (a as *mut u8).write(v) }
    }
    unsafe {
        const SLOTS: u32 = 0x10;
        const PRESENT: u32 = 0x24;
        const FACE_SLOT: u32 = 2;
        if (idx as i32) >= 3 {
            return idx;
        }
        wr32(this + SLOTS + idx.wrapping_mul(4), val);
        wr8(this + PRESENT, 1);
        if idx != FACE_SLOT {
            return idx;
        }
        if val != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, this)
        } else {
            lf_checker_rt::callee_thiscall!(2, u32, this)
        }
    }
});
