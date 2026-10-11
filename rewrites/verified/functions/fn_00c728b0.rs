// original: 0x00C728B0 initialize_animation_bundle

/// Initialize the 0x40-byte animation bundle: write its type marker at offset
/// zero and clear every following dword through offset `0x3c`. Return `this`.
lf_checker_rt::export!(thiscall, rw_00c728b0(this: u32) -> u32 {
    unsafe {
        const TYPE_MARKER: u32 = 0xc479_ff5c;
        const LAST_FIELD: u32 = 0x3c;
        (this as *mut u32).write_unaligned(TYPE_MARKER);
        for offset in (4..=LAST_FIELD).step_by(4) {
            ((this + offset) as *mut u32).write_unaligned(0);
        }
    }
    this
});
