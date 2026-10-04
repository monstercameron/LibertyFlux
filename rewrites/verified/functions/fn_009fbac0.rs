// original: 0x009fbac0 check_state_equals_one
/// Returns whether the word at `this + 0x114` equals 1.
export!(thiscall, rw_rs227_009fbac0(this_ptr: u32) -> u8 {
    unsafe { (*((this_ptr + 0x114) as *const u32) == 1) as u8 }
});
