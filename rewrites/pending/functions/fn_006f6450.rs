// original: 0x006f6450 timeout_check
/// Timeout check: has more than the stored limit elapsed since a stamp?
///
/// Reads a clock helper, subtracts the stamp at `+0x24`, and returns 1
/// when the elapsed time exceeds the limit at `+0x38`. Returns 0 when
/// the limit or stamp is zero or the mode at `+0x20` is not 3.
rt::export!(thiscall, rw_006f6450(this: *const u8) -> u8 {
    unsafe {
        if *this.add(0x38).cast::<u32>() == 0 {
            return 0;
        }
        if *this.add(0x20).cast::<u32>() != 3 {
            return 0;
        }
        let stamp = *this.add(0x24).cast::<u32>();
        if stamp == 0 {
            return 0;
        }
        let now: u32 = rt::callee_thiscall!(1, u32, this as u32);
        (now.wrapping_sub(stamp) > *this.add(0x38).cast::<u32>()) as u8
    }
});
