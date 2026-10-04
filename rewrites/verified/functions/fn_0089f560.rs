// original: 0x0089F560 rage::audStreamingSound::vf7
// ---------------------------------------------------------------------------
// 0x0089F560 rage::audStreamingSound::vf7: accept a new stream record. Runs
// the base accept first; on success latches the record's count and id, clears
// the stale flags and derives the two mode flags from the option word (+0x70).
// Returns the install result, or the base answer with a zero low byte on
// early rejection.
// ---------------------------------------------------------------------------
export!(thiscall, rw_0089F560(this_ptr: u32, a: u32, b: u32, c: u32) -> u32 {
    let ok = callee_thiscall!(1, u32, this_ptr, a, b, c);
    if ok & 0xFF == 0 {
        return ok & 0xFFFF_FF00;
    }
    let rec = unsafe { *((this_ptr.wrapping_add(0x94)) as *const u32) };
    unsafe {
        let f39 = (this_ptr.wrapping_add(0x39)) as *mut u8;
        *f39 &= 0x7F;
        let f41 = (this_ptr.wrapping_add(0x41)) as *mut u8;
        *f41 &= 0x7F;
        *((this_ptr.wrapping_add(0xD0)) as *mut u32) =
            *((rec.wrapping_add(4)) as *const u8) as u32;
    }
    let flags = unsafe { *((this_ptr.wrapping_add(0x70)) as *const u32) };
    unsafe {
        *((this_ptr.wrapping_add(0xE4)) as *mut u8) =
            ((flags & 0xC0000) == 0x40000) as u8;
        *((this_ptr.wrapping_add(0xE5)) as *mut u8) =
            ((flags & 0x300000) == 0x100000) as u8;
        *((this_ptr.wrapping_add(0xD4)) as *mut u32) = *(rec as *const u32);
    }
    callee_thiscall!(2, u32, this_ptr, c, rec)
});
