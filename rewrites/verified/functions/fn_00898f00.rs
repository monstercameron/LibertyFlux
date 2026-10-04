// original: 0x00898f00 rage::audTracker::audTracker
/// Constructs an audio tracker: installs its virtual table and clears the count.
export!(thiscall, rw_00898f00(this: u32) -> u32 {
    unsafe {
        core::ptr::write_unaligned(this.wrapping_add(4) as *mut u16, 0);
        core::ptr::write_unaligned(this as *mut u32, relocated(0xE7963C));
        this
    }
});
