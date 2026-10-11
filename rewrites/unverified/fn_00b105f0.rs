// original: 0x00B105F0 heli_check_flag_or_visibility

/// Return true when bit 2 is already set in the object's flag byte. Otherwise
/// call the visibility helper and return whether its low-byte result is
/// nonzero. The result is a byte-sized boolean in AL; the method is a
/// no-argument thiscall.
lf_checker_rt::export!(thiscall, rw_00b105f0(this: u32) -> u32 {
    const FLAGS: u32 = 0xF4;
    const READY: u8 = 0x04;

    unsafe {
        if ((this.wrapping_add(FLAGS) as *const u8).read() & READY) != 0 {
            return 1;
        }
        let answer = lf_checker_rt::callee_thiscall!(1, u32, this);
        u32::from((answer as u8) != 0)
    }
});
