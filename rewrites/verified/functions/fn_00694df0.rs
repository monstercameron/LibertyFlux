// original: 0x00694DF0 component_header_ctor_00694DF0 (proposed)

/// Initialize a two-word component header at `this`.
///
/// The constructor writes its relocated vtable pointer at byte offset zero,
/// clears the word at offset four, and returns the same object pointer. It
/// takes its object in ECX and has no stack args.
lf_checker_rt::export!(thiscall, rw_00694DF0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00FE3944;
        const STATE_WORD: u32 = 4;
        let object = this as *mut u8;
        (object as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (object.add(STATE_WORD as usize) as *mut u32).write_unaligned(0);
        this
    }
});
