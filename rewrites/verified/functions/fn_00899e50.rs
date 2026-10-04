// original: 0x00899e50 rage::audLoopingSound::audLoopingSound
/// Constructs an audio looping sound: base setup, then defaults.
///
/// Runs the base initialiser, installs the virtual table, marks the two loop
/// bounds unset, restores unity gains, and clears the counters and flags.
/// Returns the object pointer.
export!(thiscall, rw_00899e50(this: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this);
        core::ptr::write_unaligned(this as *mut u32, relocated(0xE79A1C));
        core::ptr::write_unaligned(this.wrapping_add(0xb0) as *mut u32, 0xFFFF_FFFF);
        core::ptr::write_unaligned(this.wrapping_add(0xb4) as *mut u32, 0xFFFF_FFFF);
        core::ptr::write_unaligned(this.wrapping_add(0xb8) as *mut f32, 1.0);
        core::ptr::write_unaligned(this.wrapping_add(0xbc) as *mut f32, 1.0);
        core::ptr::write_unaligned(this.wrapping_add(0xc0) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0xc4) as *mut u16, 0);
        core::ptr::write_unaligned(this.wrapping_add(0xd8) as *mut u8, 1);
        core::ptr::write_unaligned(this.wrapping_add(0xd0) as *mut u32, 0);
        this
    }
});
