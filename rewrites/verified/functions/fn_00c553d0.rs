// original: 0x00C553D0 reset_event_scan_record (proposed)
/// Clear the dword at `this + 0x10`, the word at `+0x14`, and the byte at
/// `+0x16`, preserving every other byte. The method takes `this` in ECX,
/// reads no other state, and returns the same pointer in EAX.
lf_checker_rt::export!(thiscall, rw_c553d0(this: u32) -> u32 {
    unsafe {
        (this.wrapping_add(0x10) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x14) as *mut u16).write_unaligned(0);
        (this.wrapping_add(0x16) as *mut u8).write(0);
        this
    }
});
