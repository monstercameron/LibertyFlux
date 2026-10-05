// original: 0x00c041b0 stream_pos_get (proposed)

/// Copy a three-word vector out of the streaming object into a caller buffer.
///
/// `this` points to the object; `out` points at three caller-owned dwords
/// that receive the fields at `+0x08`, `+0x0c` and `+0x10`.
/// Returns the third field (what the original leaves in `eax`).
///
/// Original: 0x00c041b0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c041b0(this: u32, out: u32) -> u32 {
    unsafe {
        const F0: u32 = 0x08;
        const F1: u32 = 0x0c;
        const F2: u32 = 0x10;
        (out as *mut u32).write_unaligned((this.wrapping_add(F0) as *const u32).read_unaligned());
        (out.wrapping_add(4) as *mut u32)
            .write_unaligned((this.wrapping_add(F1) as *const u32).read_unaligned());
        let v = (this.wrapping_add(F2) as *const u32).read_unaligned();
        (out.wrapping_add(8) as *mut u32).write_unaligned(v);
        v
    }
});
