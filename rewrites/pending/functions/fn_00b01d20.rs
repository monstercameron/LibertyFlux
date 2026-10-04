// original: 0x00b01d20 reset_channel_pair
/// Reset two strided sub-objects (this+0x424, step 0x24) with the same
/// value; return the second reset's result.
export!(thiscall, rw_00b01d20(this_: *mut u8, v: u32) -> u32 {
    let mut r = 0;
    for k in 0..2 {
        let obj = unsafe { this_.add(0x424 + k * 0x24) };
        r = callee_thiscall!(1, u32, obj as u32, v);
    }
    r
});
