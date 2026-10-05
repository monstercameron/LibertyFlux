// original: 0x00c04d80 stream_vec_set (proposed)

/// Load a three-word vector into the streaming object from a caller buffer.
///
/// `this` points to the object; `src` points at three caller-owned dwords
/// whose values are stored into the fields at `+0x0c`, `+0x10` and
/// `+0x14`. Returns the third value (what the original leaves in `eax`).
///
/// Original: 0x00c04d80 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c04d80(this: u32, src: u32) -> u32 {
    unsafe {
        const F0: u32 = 0x0c;
        const F1: u32 = 0x10;
        const F2: u32 = 0x14;
        (this.wrapping_add(F0) as *mut u32)
            .write_unaligned((src as *const u32).read_unaligned());
        (this.wrapping_add(F1) as *mut u32)
            .write_unaligned((src.wrapping_add(4) as *const u32).read_unaligned());
        let v = (src.wrapping_add(8) as *const u32).read_unaligned();
        (this.wrapping_add(F2) as *mut u32).write_unaligned(v);
        v
    }
});
