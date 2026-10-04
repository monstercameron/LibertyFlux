// original: 0x0097b150 audio_state_flag_test
/// Test whether an audio object's state field selects an active voice.
///
/// Reads the state word at +0x7c, subtracts the two header states and
/// reports 1 when the result names one of the five live voices
/// (0, 2, 4, 6, 10), otherwise 0. Out-of-range states keep their upper
/// bits from the subtraction with the low byte cleared.
export!(thiscall, rw_0097b150(this: *const u8) -> u32 {
    unsafe {
        let t = (*((this.add(0x7c)) as *const u32)).wrapping_sub(2);
        if t > 10 {
            t & 0xffffff00
        } else {
            (0x455u32 >> t) & 1
        }
    }
});
