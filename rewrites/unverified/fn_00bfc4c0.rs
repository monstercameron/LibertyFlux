// original: 0x00bfc4c0 compute_field2c_copy_via_frame_temp
/// Refine a frame slot through two helpers, store the answer, copy a row.
///
/// Hands a pointer to a fresh frame slot to the first helper with the
/// incoming object in the object register, passes the same slot to the
/// second helper, stores the second helper's answer into this object's
/// field, then copies three words from the incoming object's row into
/// this object's row. Returns the last copied word.
export!(thiscall, rw_00bfc4c0(this_: *mut u8, other: *const u8) -> u32 {
    unsafe {
        let mut slot: u32 = 0;
        callee_thiscall!(1, u32, other as u32, &mut slot as *mut u32 as u32);
        let answer = callee_cdecl!(2, u32, &mut slot as *mut u32 as u32);
        *(this_.add(0x2c) as *mut u32) = answer;
        let w0 = *(other.add(0x30) as *const u32);
        *(this_.add(0x30) as *mut u32) = w0;
        let w1 = *(other.add(0x34) as *const u32);
        *(this_.add(0x34) as *mut u32) = w1;
        let w2 = *(other.add(0x38) as *const u32);
        *(this_.add(0x38) as *mut u32) = w2;
        w2
    }
});
