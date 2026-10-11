// original: 0x00C729E0 initialize_animation_bundle_compact

/// Initialize the compact animation bundle: write the type marker at zero,
/// clear the dwords through `+0x3c` and clear the three state words at
/// `+0x40`, `+0x44` and `+0x48`. Later fields are left untouched. Return
/// `this`.
lf_checker_rt::export!(thiscall, rw_00c729e0(this: u32) -> u32 {
    unsafe {
        const TYPE_MARKER: u32 = 0xc479_ff5c;
        for offset in (4..=0x3c).step_by(4) {
            ((this + offset) as *mut u32).write_unaligned(0);
        }
        for offset in [0x40u32, 0x44, 0x48] {
            ((this + offset) as *mut u32).write_unaligned(0);
        }
        (this as *mut u32).write_unaligned(TYPE_MARKER);
    }
    this
});
