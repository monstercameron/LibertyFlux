// original: 0x00c05800 stream_init_7f (proposed)

/// Initialise a streaming record header, copying a key from an optional source.
///
/// `this` points to the record, `src` is null or points to a source object.
/// Writes the tag `0x7f` at `+0x00`, the shared streaming global into `+0x04`
/// and zero into `+0x01`; the key at `+0x08` comes from the source's `+0x64`
/// word, or is `-1` when `src` is null. Returns the key, or zero for a null
/// source (what the original leaves in `eax`).
///
/// Original: 0x00c05800 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c05800(this: u32, src: u32) -> u32 {
    unsafe {
        const TAG: u8 = 0x7f;
        const GLOBAL_STREAM: u32 = 0x011735a4;
        const SRC_KEY: u32 = 0x64;
        const KEY_MISSING: u32 = 0xffff_ffff;
        (this as *mut u8).write(TAG);
        let g = (lf_checker_rt::relocated(GLOBAL_STREAM) as *const u32).read_unaligned();
        (this.wrapping_add(4) as *mut u32).write_unaligned(g);
        if src != 0 {
            let key = (src.wrapping_add(SRC_KEY) as *const u32).read_unaligned();
            (this.wrapping_add(8) as *mut u32).write_unaligned(key);
            (this.wrapping_add(1) as *mut u8).write(0);
            key
        } else {
            (this.wrapping_add(8) as *mut u32).write_unaligned(KEY_MISSING);
            (this.wrapping_add(1) as *mut u8).write(0);
            0
        }
    }
});
