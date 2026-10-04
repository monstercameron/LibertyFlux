// original: 0x008adac0 audio_curve_copy_desc_fields
/// Five-field copy from a descriptor record plus mode-byte setup.
///
/// Copies the scalar fields out of `[this]` into the inline cache slots and
/// sets the mode byte. Always reports success.
export!(thiscall, rw_008adac0(this: u32) -> u32 {
    unsafe {
        let edx = ld32(this);
        st32(this.wrapping_add(0x04), ld32(edx.wrapping_add(0x21)));
        st32(this.wrapping_add(0x08), ld32(edx.wrapping_add(0x15)));
        st32(this.wrapping_add(0x0c), ld32(edx.wrapping_add(0x19)));
        st32(this.wrapping_add(0x10), ld32(edx.wrapping_add(0x25)));
        st32(this.wrapping_add(0x14), ld32(edx.wrapping_add(0x1d)));
        ((this.wrapping_add(0x26)) as *mut u8).write_unaligned(1);
        1
    }
});
