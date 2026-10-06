// original: 0x00974850 audio_ptr_release_and_clear (proposed)

/// Release the pointer held at +0xCC and clear the slot. When the slot is
/// null nothing happens. The callee takes one stack word (cdecl).
/// Original: 0x00974850 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00974850(this: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0xCC;
        const RELEASE: u32 = 1;
        let p = ((this.wrapping_add(SLOT)) as *const u32).read_unaligned();
        if p != 0 {
            lf_checker_rt::callee_cdecl!(RELEASE, u32, p);
            ((this.wrapping_add(SLOT)) as *mut u32).write_unaligned(0);
        }
        0
    }
});
