// original: 0x00bfc490 compute_field40_via_frame_temp
/// Fill a frame slot through the first helper, refine it, store the answer.
///
/// Hands a pointer to a fresh frame slot to the first helper with the
/// incoming word in the object register, passes the same slot to the
/// second helper, then stores the second helper's answer into this
/// object's field. Returns that answer.
export!(thiscall, rw_00bfc490(this_: *mut u8, arg: u32) -> u32 {
    unsafe {
        let mut slot: u32 = 0;
        callee_thiscall!(1, u32, arg, &mut slot as *mut u32 as u32);
        let answer = callee_cdecl!(2, u32, &mut slot as *mut u32 as u32);
        *(this_.add(0x40) as *mut u32) = answer;
        answer
    }
});
