// original: 0x00a7c970 deferred_00a7c970
/// Collect candidates from the helper and return the topmost match.
///
/// Asks the first helper to fill a 60-word frame buffer (it answers how
/// many words it wrote), then tests the written words from the top down
/// with the second helper, which takes each word in the object register
/// and this object on the stack. Returns the first word the tester
/// accepts, or zero when none matches.
export!(thiscall, rw_00a7c970(this_: u32) -> u32 {
    unsafe {
        let mut buf = [0u32; 60];
        let count = callee_thiscall!(1, u32, this_, buf.as_mut_ptr() as u32);
        if count == 0 {
            return 0;
        }
        let mut index = count.wrapping_sub(1);
        loop {
            let word = *buf.as_ptr().add(index as usize);
            let hit = callee_thiscall!(2, u32, word, this_);
            if (hit as u8) != 0 {
                return word;
            }
            if index == 0 {
                return 0;
            }
            index -= 1;
        }
    }
});
