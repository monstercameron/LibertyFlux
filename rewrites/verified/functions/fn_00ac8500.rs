// original: 0x00AC8500 copy_stream_vector_and_scale

/// Copy a four-word vector from the object to the caller's output buffer and
/// return one and a half times the float at `this + 8`.
///
/// The copied words come from offsets `+0x50`, `+0x54`, `+0x58` and `+0x5c`;
/// the first and last are copied as raw words, preserving all bit patterns.
/// The returned `f32` is produced through the x87 return channel, and thiscall
/// removes the output pointer argument.
lf_checker_rt::export!(thiscall, rw_00ac8500(this: u32, output: u32) -> f32 {
    const VECTOR_OFFSET: u32 = 0x50;
    const SCALE_INPUT_OFFSET: u32 = 0x08;
    const SCALE: f32 = 1.5;

    unsafe {
        let source = this.wrapping_add(VECTOR_OFFSET) as *const u8;
        let destination = output as *mut u8;
        let y = (source.add(4) as *const f32).read_unaligned();
        (destination.add(4) as *mut f32).write_unaligned(y);
        let x_bits = (source as *const u32).read_unaligned();
        (destination as *mut u32).write_unaligned(x_bits);
        let z = (source.add(8) as *const f32).read_unaligned();
        (destination.add(8) as *mut f32).write_unaligned(z);
        let w_bits = (source.add(12) as *const u32).read_unaligned();
        (destination.add(12) as *mut u32).write_unaligned(w_bits);
        let scale_input = ((this.wrapping_add(SCALE_INPUT_OFFSET)) as *const f32).read_unaligned();
        core::hint::black_box(scale_input) * core::hint::black_box(SCALE)
    }
});
