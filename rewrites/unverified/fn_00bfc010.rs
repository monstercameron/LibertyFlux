// original: 0x00bfc010 compute_via_frame_temp
/// Run the field word through two helpers, returning the second answer.
///
/// Hands a pointer to a fresh frame slot plus the word at this object's
/// field to the first helper, then hands the same slot to the other
/// helper with the incoming word in the object register. The first helper
/// fills the slot; the second refines it. Returns the second answer.
export!(thiscall, rw_00bfc010(this_: *const u8, arg: u32) -> u32 {
    unsafe {
        let word = *(this_.add(0x40) as *const u32);
        let mut slot: u32 = 0;
        callee_cdecl!(1, u32, &mut slot as *mut u32 as u32, word);
        callee_thiscall!(2, u32, arg, &mut slot as *mut u32 as u32)
    }
});
