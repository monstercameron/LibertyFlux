// original: 0x009e25f0 audio_tracker_state_init
/// Zero the 0x48-byte tracker state, then run its initialiser.
/// Returns the object pointer. (thiscall/0)
export!(thiscall, rw_009e25f0(this: *mut u8) -> u32 {
    unsafe {
        let words = this as *mut u32;
        let mut i = 0usize;
        while i < 18 {
            *words.add(i) = 0;
            i += 1;
        }
        callee_thiscall!(1, u32, this as u32);
        this as u32
    }
});
