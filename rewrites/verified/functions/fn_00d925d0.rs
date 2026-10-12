// original: 0x00D925D0 audio_zone_publish_entry

/// Copies four words from the source record at this+0x70 to the output at
/// this+0x90, then calls the recomputation helper with this and the source
/// pointer. It loads the first three words before writing them, reads the
/// fourth after those writes, and returns the helper's EAX value.
lf_checker_rt::export!(thiscall, rw_00d925d0(this: u32) -> u32 {
    const SOURCE_SLOT: u32 = 0x70;
    const OUTPUT_SLOT: u32 = 0x90;
    const RECOMPUTE_ENTRY: u32 = 1;

    let source = unsafe { (this.wrapping_add(SOURCE_SLOT) as *const u32).read_unaligned() };
    let destination = unsafe { (this.wrapping_add(OUTPUT_SLOT) as *const u32).read_unaligned() };
    let (first, second, third) = unsafe {
        (
            (source as *const u32).read_volatile(),
            (source.wrapping_add(4) as *const u32).read_volatile(),
            (source.wrapping_add(8) as *const u32).read_volatile(),
        )
    };
    unsafe {
        (destination as *mut u32).write_volatile(first);
        (destination.wrapping_add(4) as *mut u32).write_volatile(second);
        (destination.wrapping_add(8) as *mut u32).write_volatile(third);
    }
    let fourth = unsafe { (source.wrapping_add(12) as *const u32).read_volatile() };
    unsafe { (destination.wrapping_add(12) as *mut u32).write_volatile(fourth); }

    lf_checker_rt::callee_thiscall!(RECOMPUTE_ENTRY, u32, this, source)
});